use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};

use sdkwork_aiot_storage::{
    table_contract, AiotProtocolDeadLetterIntent, AiotProtocolIngestUnitOfWork,
    AiotProtocolStorageCommand, AiotStorageWriteReceipt,
};

pub fn schema_version() -> &'static str {
    "0.1.0"
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SqlMigration {
    pub version: &'static str,
    pub name: &'static str,
    pub schema_version: &'static str,
    pub sql: &'static str,
}

pub fn migration_catalog() -> Vec<SqlMigration> {
    vec![SqlMigration {
        version: "0001",
        name: "aiot_core_schema",
        schema_version: schema_version(),
        sql: initial_migration_sql(),
    }]
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SqlBindValue {
    Text(String),
    Int64(i64),
    Null,
}

impl SqlBindValue {
    fn text(value: impl Into<String>) -> Self {
        Self::Text(value.into())
    }

    fn optional_text(value: Option<&str>) -> Self {
        value
            .map(|value| Self::Text(value.to_string()))
            .unwrap_or(Self::Null)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SqlDialect {
    Postgres,
    Sqlite,
}

impl SqlDialect {
    fn placeholder(&self, index: usize) -> String {
        match self {
            Self::Postgres => format!("${index}"),
            Self::Sqlite => "?".to_string(),
        }
    }

    fn placeholders(&self, count: usize) -> String {
        match self {
            Self::Postgres => (1..=count)
                .map(|index| self.placeholder(index))
                .collect::<Vec<_>>()
                .join(", "),
            Self::Sqlite => vec!["?"; count].join(", "),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SqlPlanError {
    pub code: String,
    pub table: Option<String>,
    pub column: Option<String>,
    pub statement_kind: Option<&'static str>,
}

impl SqlPlanError {
    pub fn new(code: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            table: None,
            column: None,
            statement_kind: None,
        }
    }

    pub fn with_table(mut self, table: impl Into<String>) -> Self {
        self.table = Some(table.into());
        self
    }

    pub fn with_statement_kind(mut self, statement_kind: &'static str) -> Self {
        self.statement_kind = Some(statement_kind);
        self
    }

    pub fn with_column(mut self, column: impl Into<String>) -> Self {
        self.column = Some(column.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SqlStatementPlan {
    pub statement_kind: &'static str,
    pub table: &'static str,
    pub dialect: SqlDialect,
    pub sql: String,
    pub binds: Vec<SqlBindValue>,
}

impl SqlStatementPlan {
    pub fn new(statement_kind: &'static str, table: &'static str, sql: impl Into<String>) -> Self {
        Self {
            statement_kind,
            table,
            dialect: SqlDialect::Postgres,
            sql: sql.into(),
            binds: Vec::new(),
        }
    }

    pub fn with_dialect(mut self, dialect: SqlDialect) -> Self {
        self.dialect = dialect;
        self
    }

    pub fn with_binds(mut self, binds: Vec<SqlBindValue>) -> Self {
        self.binds = binds;
        self
    }

    pub fn placeholder_count(&self) -> usize {
        match self.dialect {
            SqlDialect::Postgres => postgres_placeholder_count(&self.sql),
            SqlDialect::Sqlite => self
                .sql
                .chars()
                .filter(|candidate| *candidate == '?')
                .count(),
        }
    }

    pub fn validate(&self) -> Result<(), SqlPlanError> {
        let placeholder_count = self.placeholder_count();
        if placeholder_count != self.binds.len() {
            return Err(SqlPlanError::new("storage.sql.bind_count_mismatch")
                .with_table(self.table)
                .with_statement_kind(self.statement_kind));
        }

        if table_contract(self.table).is_none() {
            return Err(SqlPlanError::new("storage.sql.table.unsupported")
                .with_table(self.table)
                .with_statement_kind(self.statement_kind));
        }

        for column in sql_write_columns(&self.sql) {
            if !initial_migration_declares_column(self.table, &column) {
                return Err(SqlPlanError::new("storage.sql.column.unsupported")
                    .with_table(self.table)
                    .with_column(column)
                    .with_statement_kind(self.statement_kind));
            }
        }

        Ok(())
    }
}

impl SqlStatementPlan {
    fn bound(
        statement_kind: &'static str,
        table: &'static str,
        dialect: SqlDialect,
        sql: impl Into<String>,
        binds: Vec<SqlBindValue>,
    ) -> Self {
        Self::new(statement_kind, table, sql)
            .with_dialect(dialect)
            .with_binds(binds)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SqlStatementBatch {
    pub batch_kind: &'static str,
    pub statements: Vec<SqlStatementPlan>,
}

impl SqlStatementBatch {
    pub fn new(batch_kind: &'static str, statements: Vec<SqlStatementPlan>) -> Self {
        Self {
            batch_kind,
            statements,
        }
    }

    pub fn single(batch_kind: &'static str, statement: SqlStatementPlan) -> Self {
        Self::new(batch_kind, vec![statement])
    }

    pub fn validate(&self) -> Result<(), SqlPlanError> {
        for statement in &self.statements {
            statement.validate()?;
        }

        Ok(())
    }
}

pub trait SqlStatementExecutor: Clone {
    fn execute_idempotency_guard(&self, key: &str, statement: SqlStatementPlan) -> bool;

    fn execute_batch(&self, batch: SqlStatementBatch);

    fn execute_transaction(&self, transaction: SqlTransactionPlan) -> SqlTransactionOutcome {
        let SqlTransactionPlan {
            idempotency_key,
            guard,
            write_batch,
            ..
        } = transaction;

        if !self.execute_idempotency_guard(&idempotency_key, guard) {
            return SqlTransactionOutcome::Duplicate;
        }

        self.execute_batch(write_batch);
        SqlTransactionOutcome::Committed
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SqlProtocolCommandPlan {
    pub idempotency_key: String,
    pub guard: SqlStatementPlan,
    pub write_batch: SqlStatementBatch,
}

impl SqlProtocolCommandPlan {
    pub fn validate(&self) -> Result<(), SqlPlanError> {
        self.guard.validate()?;
        self.write_batch.validate()?;

        Ok(())
    }

    pub fn into_transaction_plan(self) -> SqlTransactionPlan {
        SqlTransactionPlan::new(
            "protocol_ingest",
            self.idempotency_key,
            self.guard,
            self.write_batch,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SqlTransactionFailurePolicy {
    RollbackAll,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SqlTransactionOutcome {
    Committed,
    Duplicate,
    RolledBack { reason_code: String },
}

impl SqlTransactionOutcome {
    pub fn rolled_back(reason_code: impl Into<String>) -> Self {
        Self::RolledBack {
            reason_code: reason_code.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SqlTransactionPlan {
    pub transaction_kind: &'static str,
    pub failure_policy: SqlTransactionFailurePolicy,
    pub idempotency_key: String,
    pub guard: SqlStatementPlan,
    pub write_batch: SqlStatementBatch,
}

impl SqlTransactionPlan {
    pub fn new(
        transaction_kind: &'static str,
        idempotency_key: impl Into<String>,
        guard: SqlStatementPlan,
        write_batch: SqlStatementBatch,
    ) -> Self {
        Self {
            transaction_kind,
            failure_policy: SqlTransactionFailurePolicy::RollbackAll,
            idempotency_key: idempotency_key.into(),
            guard,
            write_batch,
        }
    }

    pub fn ordered_statements(&self) -> Vec<SqlStatementPlan> {
        let mut statements = Vec::with_capacity(1 + self.write_batch.statements.len());
        statements.push(self.guard.clone());
        statements.extend(self.write_batch.statements.iter().cloned());
        statements
    }

    pub fn validate(&self) -> Result<(), SqlPlanError> {
        self.guard.validate()?;
        self.write_batch.validate()?;

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SqlProtocolIngestPlanner {
    dialect: SqlDialect,
}

impl SqlProtocolIngestPlanner {
    pub fn standard() -> Self {
        Self::for_dialect(SqlDialect::Postgres)
    }

    pub fn for_dialect(dialect: SqlDialect) -> Self {
        Self { dialect }
    }

    pub fn dialect(&self) -> SqlDialect {
        self.dialect
    }

    pub fn plan_protocol_command(
        &self,
        command: &AiotProtocolStorageCommand,
    ) -> SqlProtocolCommandPlan {
        self.try_plan_protocol_command(command)
            .expect("standard protocol command plan must be valid")
    }

    pub fn try_plan_protocol_command(
        &self,
        command: &AiotProtocolStorageCommand,
    ) -> Result<SqlProtocolCommandPlan, SqlPlanError> {
        if table_contract(command.primary_table).is_none() {
            return Err(SqlPlanError::new("storage.sql.primary_table.unsupported")
                .with_table(command.primary_table));
        }

        let idempotency_key = command.idempotency_key.clone().unwrap_or_else(|| {
            format!(
                "{}:{}:{}:{}:{}",
                command.protocol_id,
                command.adapter_id,
                command.device_id,
                command.kind.as_str(),
                command.primary_table
            )
        });
        let guard = idempotency_guard_statement(self.dialect, command, &idempotency_key);
        let mut statements = vec![primary_write_statement(
            self.dialect,
            command,
            &idempotency_key,
        )];
        if command.outbox.is_some() {
            statements.push(outbox_write_statement(self.dialect, command));
        }

        let plan = SqlProtocolCommandPlan {
            idempotency_key,
            guard,
            write_batch: SqlStatementBatch::new("protocol_ingest_write", statements),
        };
        plan.validate()?;

        Ok(plan)
    }

    pub fn plan_dead_letter(&self, intent: &AiotProtocolDeadLetterIntent) -> SqlStatementBatch {
        self.try_plan_dead_letter(intent)
            .expect("standard dead-letter plan must be valid")
    }

    pub fn try_plan_dead_letter(
        &self,
        intent: &AiotProtocolDeadLetterIntent,
    ) -> Result<SqlStatementBatch, SqlPlanError> {
        let batch = SqlStatementBatch::single(
            "dead_letter_write",
            dead_letter_write_statement(self.dialect, intent),
        );
        batch.validate()?;

        Ok(batch)
    }
}

impl Default for SqlProtocolIngestPlanner {
    fn default() -> Self {
        Self::standard()
    }
}

#[derive(Debug, Clone, Default)]
pub struct InMemorySqlStatementExecutor {
    state: Arc<Mutex<InMemorySqlStatementExecutorState>>,
}

#[derive(Debug, Default)]
struct InMemorySqlStatementExecutorState {
    idempotency_keys: BTreeSet<String>,
    executed_statements: Vec<SqlStatementPlan>,
    executed_batches: Vec<SqlStatementBatch>,
}

impl InMemorySqlStatementExecutor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn claim_idempotency_key(&self, key: &str) -> bool {
        self.state
            .lock()
            .expect("sql statement executor poisoned")
            .idempotency_keys
            .insert(key.to_string())
    }

    pub fn execute(&self, statement: SqlStatementPlan) {
        self.state
            .lock()
            .expect("sql statement executor poisoned")
            .executed_statements
            .push(statement);
    }

    pub fn execute_batch(&self, batch: SqlStatementBatch) {
        let mut state = self.state.lock().expect("sql statement executor poisoned");
        state
            .executed_statements
            .extend(batch.statements.iter().cloned());
        state.executed_batches.push(batch);
    }

    pub fn executed_statements(&self) -> Vec<SqlStatementPlan> {
        self.state
            .lock()
            .expect("sql statement executor poisoned")
            .executed_statements
            .clone()
    }

    pub fn executed_batches(&self) -> Vec<SqlStatementBatch> {
        self.state
            .lock()
            .expect("sql statement executor poisoned")
            .executed_batches
            .clone()
    }
}

impl SqlStatementExecutor for InMemorySqlStatementExecutor {
    fn execute_idempotency_guard(&self, key: &str, statement: SqlStatementPlan) -> bool {
        let mut state = self.state.lock().expect("sql statement executor poisoned");
        state.executed_statements.push(statement.clone());
        state
            .executed_batches
            .push(SqlStatementBatch::single("idempotency_guard", statement));
        state.idempotency_keys.insert(key.to_string())
    }

    fn execute_batch(&self, batch: SqlStatementBatch) {
        InMemorySqlStatementExecutor::execute_batch(self, batch);
    }
}

#[derive(Debug, Clone)]
pub struct SqlxProtocolIngestUnitOfWork<E: SqlStatementExecutor = InMemorySqlStatementExecutor> {
    executor: E,
    planner: SqlProtocolIngestPlanner,
}

impl<E: SqlStatementExecutor> SqlxProtocolIngestUnitOfWork<E> {
    pub fn new(executor: E) -> Self {
        Self {
            executor,
            planner: SqlProtocolIngestPlanner::standard(),
        }
    }

    pub fn with_planner(executor: E, planner: SqlProtocolIngestPlanner) -> Self {
        Self { executor, planner }
    }
}

impl<E: SqlStatementExecutor> AiotProtocolIngestUnitOfWork for SqlxProtocolIngestUnitOfWork<E> {
    fn execute_protocol_command(
        &self,
        command: &AiotProtocolStorageCommand,
    ) -> AiotStorageWriteReceipt {
        let plan = match self.planner.try_plan_protocol_command(command) {
            Ok(plan) => plan,
            Err(error) => return AiotStorageWriteReceipt::dead_lettered(error.code),
        };
        let outcome = self
            .executor
            .execute_transaction(plan.into_transaction_plan());

        match outcome {
            SqlTransactionOutcome::Committed => AiotStorageWriteReceipt::accepted(
                command.kind,
                command.primary_table,
                command
                    .outbox
                    .as_ref()
                    .map(|outbox| outbox.event_type.clone()),
            ),
            SqlTransactionOutcome::Duplicate => {
                let mut receipt = AiotStorageWriteReceipt::accepted(
                    command.kind,
                    command.primary_table,
                    command
                        .outbox
                        .as_ref()
                        .map(|outbox| outbox.event_type.clone()),
                );
                receipt.duplicate = true;
                receipt
            }
            SqlTransactionOutcome::RolledBack { reason_code } => {
                AiotStorageWriteReceipt::dead_lettered(reason_code)
            }
        }
    }

    fn record_dead_letter(&self, intent: &AiotProtocolDeadLetterIntent) -> AiotStorageWriteReceipt {
        let batch = match self.planner.try_plan_dead_letter(intent) {
            Ok(batch) => batch,
            Err(error) => return AiotStorageWriteReceipt::dead_lettered(error.code),
        };
        self.executor.execute_batch(batch);
        AiotStorageWriteReceipt::dead_lettered(intent.reason_code.clone())
    }
}

fn postgres_placeholder_count(sql: &str) -> usize {
    let mut max_placeholder = 0;
    let bytes = sql.as_bytes();
    let mut index = 0;

    while index < bytes.len() {
        if bytes[index] == b'$' {
            let mut number_index = index + 1;
            let mut value = 0_usize;
            while number_index < bytes.len() && bytes[number_index].is_ascii_digit() {
                value = value
                    .saturating_mul(10)
                    .saturating_add((bytes[number_index] - b'0') as usize);
                number_index += 1;
            }
            if number_index > index + 1 {
                max_placeholder = max_placeholder.max(value);
                index = number_index;
                continue;
            }
        }
        index += 1;
    }

    max_placeholder
}

fn sql_write_columns(sql: &str) -> Vec<String> {
    let trimmed = sql.trim_start();
    let upper = trimmed.to_ascii_uppercase();

    if upper.starts_with("INSERT INTO ") {
        return insert_write_columns(trimmed);
    }

    if upper.starts_with("UPDATE ") {
        return update_write_columns(trimmed);
    }

    Vec::new()
}

fn insert_write_columns(sql: &str) -> Vec<String> {
    let Some(start) = sql.find('(') else {
        return Vec::new();
    };
    let Some(end) = sql[start + 1..].find(')') else {
        return Vec::new();
    };

    comma_separated_identifiers(&sql[start + 1..start + 1 + end])
}

fn update_write_columns(sql: &str) -> Vec<String> {
    let Some(set_start) = find_ascii_case_insensitive(sql, " SET ") else {
        return Vec::new();
    };
    let after_set = set_start + " SET ".len();
    let where_start = find_ascii_case_insensitive(&sql[after_set..], " WHERE ")
        .map(|offset| after_set + offset)
        .unwrap_or(sql.len());

    sql[after_set..where_start]
        .split(',')
        .filter_map(|assignment| assignment.split_once('='))
        .map(|(column, _)| normalize_sql_identifier(column))
        .filter(|column| !column.is_empty())
        .collect()
}

fn comma_separated_identifiers(segment: &str) -> Vec<String> {
    segment
        .split(',')
        .map(normalize_sql_identifier)
        .filter(|column| !column.is_empty())
        .collect()
}

fn normalize_sql_identifier(identifier: &str) -> String {
    identifier
        .trim()
        .trim_matches('"')
        .trim_matches('`')
        .trim_matches('[')
        .trim_matches(']')
        .to_string()
}

fn find_ascii_case_insensitive(haystack: &str, needle: &str) -> Option<usize> {
    haystack
        .to_ascii_uppercase()
        .find(&needle.to_ascii_uppercase())
}

fn initial_migration_declares_column(table: &str, column: &str) -> bool {
    let Some(definition) = initial_migration_table_definition(table) else {
        return false;
    };

    definition
        .lines()
        .map(str::trim)
        .any(|line| line.starts_with(&format!("{column} ")))
}

fn initial_migration_table_definition(table: &str) -> Option<&'static str> {
    let sql = initial_migration_sql();
    let marker = format!("CREATE TABLE {table}");
    let start = sql.find(&marker)?;
    let rest = &sql[start + marker.len()..];
    let end = rest.find("\nCREATE TABLE ").unwrap_or(rest.len());

    Some(&sql[start..start + marker.len() + end])
}

fn idempotency_guard_statement(
    dialect: SqlDialect,
    command: &AiotProtocolStorageCommand,
    idempotency_key: &str,
) -> SqlStatementPlan {
    SqlStatementPlan::bound(
        "idempotency_guard",
        "iot_protocol_ingest_record",
        dialect,
        format!(
            "INSERT INTO iot_protocol_ingest_record (tenant_id, organization_id, data_scope, protocol_id, adapter_id, device_id, message_id, correlation_id, idempotency_key, trace_id, status) VALUES ({}) ON CONFLICT DO NOTHING",
            dialect.placeholders(11)
        ),
        vec![
            SqlBindValue::Int64(command.association.tenant_id),
            SqlBindValue::Int64(command.association.organization_id),
            SqlBindValue::Int64(command.association.data_scope.into()),
            SqlBindValue::text(&command.protocol_id),
            SqlBindValue::text(&command.adapter_id),
            SqlBindValue::text(&command.device_id),
            SqlBindValue::optional_text(command.message_id.as_deref()),
            SqlBindValue::optional_text(command.correlation_id.as_deref()),
            SqlBindValue::text(idempotency_key),
            SqlBindValue::optional_text(command.trace_id.as_deref()),
            SqlBindValue::Int64(0),
        ],
    )
}

fn primary_write_statement(
    dialect: SqlDialect,
    command: &AiotProtocolStorageCommand,
    idempotency_key: &str,
) -> SqlStatementPlan {
    let placeholders = (1..=8)
        .map(|index| dialect.placeholder(index))
        .collect::<Vec<_>>();

    SqlStatementPlan::bound(
        "primary_write",
        "iot_protocol_ingest_record",
        dialect,
        format!(
            "UPDATE iot_protocol_ingest_record SET status = {} WHERE tenant_id = {} AND organization_id = {} AND data_scope = {} AND protocol_id = {} AND adapter_id = {} AND device_id = {} AND idempotency_key = {}",
            placeholders[0],
            placeholders[1],
            placeholders[2],
            placeholders[3],
            placeholders[4],
            placeholders[5],
            placeholders[6],
            placeholders[7]
        ),
        vec![
            SqlBindValue::Int64(1),
            SqlBindValue::Int64(command.association.tenant_id),
            SqlBindValue::Int64(command.association.organization_id),
            SqlBindValue::Int64(command.association.data_scope.into()),
            SqlBindValue::text(&command.protocol_id),
            SqlBindValue::text(&command.adapter_id),
            SqlBindValue::text(&command.device_id),
            SqlBindValue::text(idempotency_key),
        ],
    )
}

fn outbox_write_statement(
    dialect: SqlDialect,
    command: &AiotProtocolStorageCommand,
) -> SqlStatementPlan {
    let outbox = command.outbox.as_ref().expect("outbox intent");
    SqlStatementPlan::bound(
        "outbox_write",
        "iot_outbox_event",
        dialect,
        format!(
            "INSERT INTO iot_outbox_event (tenant_id, organization_id, data_scope, event_type, aggregate_type, aggregate_id, payload, status, trace_id, attempt_count) VALUES ({})",
            dialect.placeholders(10)
        ),
        vec![
            SqlBindValue::Int64(command.association.tenant_id),
            SqlBindValue::Int64(command.association.organization_id),
            SqlBindValue::Int64(command.association.data_scope.into()),
            SqlBindValue::text(&outbox.event_type),
            SqlBindValue::text(&outbox.aggregate_type),
            SqlBindValue::text(&outbox.aggregate_id),
            SqlBindValue::text(&outbox.topic),
            SqlBindValue::Int64(0),
            SqlBindValue::optional_text(command.trace_id.as_deref()),
            SqlBindValue::Int64(0),
        ],
    )
}

fn dead_letter_write_statement(
    dialect: SqlDialect,
    intent: &AiotProtocolDeadLetterIntent,
) -> SqlStatementPlan {
    SqlStatementPlan::bound(
        "dead_letter_write",
        "iot_protocol_message_dead_letter",
        dialect,
        format!(
            "INSERT INTO iot_protocol_message_dead_letter (tenant_id, organization_id, data_scope, protocol_id, adapter_id, device_id, reason_code, payload_ref, trace_id, status) VALUES ({})",
            dialect.placeholders(10)
        ),
        vec![
            SqlBindValue::Int64(intent.association.tenant_id),
            SqlBindValue::Int64(intent.association.organization_id),
            SqlBindValue::Int64(intent.association.data_scope.into()),
            SqlBindValue::text(&intent.protocol_id),
            SqlBindValue::text(&intent.adapter_id),
            SqlBindValue::optional_text(intent.device_id.as_deref()),
            SqlBindValue::text(&intent.reason_code),
            SqlBindValue::optional_text(intent.payload_ref.as_deref()),
            SqlBindValue::optional_text(intent.trace_id.as_deref()),
            SqlBindValue::Int64(0),
        ],
    )
}

pub fn initial_migration_sql() -> &'static str {
    r#"
CREATE TABLE iot_product (
    id BIGINT NOT NULL,
    uuid VARCHAR(64) NOT NULL,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    data_scope INTEGER NOT NULL DEFAULT 0,
    owner_type VARCHAR(32) NOT NULL,
    owner_id VARCHAR(128) NOT NULL,
    product_key VARCHAR(128) NOT NULL,
    display_name VARCHAR(200) NOT NULL,
    status INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    version BIGINT NOT NULL DEFAULT 0,
    created_by BIGINT,
    updated_by BIGINT,
    PRIMARY KEY (id),
    CONSTRAINT uk_iot_product_uuid UNIQUE (uuid),
    CONSTRAINT uk_iot_product_tenant_key UNIQUE (tenant_id, product_key)
);

CREATE TABLE iot_hardware_profile (
    id BIGINT NOT NULL,
    uuid VARCHAR(64) NOT NULL,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    data_scope INTEGER NOT NULL DEFAULT 0,
    owner_type VARCHAR(32) NOT NULL,
    owner_id VARCHAR(128) NOT NULL,
    profile_key VARCHAR(128) NOT NULL,
    chip_family VARCHAR(64) NOT NULL,
    runtime_profile VARCHAR(64) NOT NULL,
    connectivity_profile VARCHAR(64) NOT NULL,
    security_profile VARCHAR(64),
    ota_profile VARCHAR(64),
    status INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    version BIGINT NOT NULL DEFAULT 0,
    created_by BIGINT,
    updated_by BIGINT,
    PRIMARY KEY (id),
    CONSTRAINT uk_iot_hardware_profile_uuid UNIQUE (uuid),
    CONSTRAINT uk_iot_hardware_profile_tenant_key UNIQUE (tenant_id, profile_key)
);

CREATE INDEX idx_iot_hardware_profile_tenant_chip
    ON iot_hardware_profile (tenant_id, chip_family, runtime_profile);

CREATE TABLE iot_protocol_profile (
    id BIGINT NOT NULL,
    uuid VARCHAR(64) NOT NULL,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    data_scope INTEGER NOT NULL DEFAULT 0,
    owner_type VARCHAR(32) NOT NULL,
    owner_id VARCHAR(128) NOT NULL,
    profile_key VARCHAR(128) NOT NULL,
    default_protocol_id VARCHAR(128) NOT NULL,
    allowed_transports TEXT NOT NULL,
    allowed_message_classes TEXT NOT NULL,
    status INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    version BIGINT NOT NULL DEFAULT 0,
    created_by BIGINT,
    updated_by BIGINT,
    PRIMARY KEY (id),
    CONSTRAINT uk_iot_protocol_profile_uuid UNIQUE (uuid),
    CONSTRAINT uk_iot_protocol_profile_tenant_key UNIQUE (tenant_id, profile_key)
);

CREATE INDEX idx_iot_protocol_profile_tenant_protocol
    ON iot_protocol_profile (tenant_id, default_protocol_id, status);

CREATE TABLE iot_capability_model (
    id BIGINT NOT NULL,
    uuid VARCHAR(64) NOT NULL,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    data_scope INTEGER NOT NULL DEFAULT 0,
    owner_type VARCHAR(32) NOT NULL,
    owner_id VARCHAR(128) NOT NULL,
    model_key VARCHAR(128) NOT NULL,
    display_name VARCHAR(200) NOT NULL,
    status INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    version BIGINT NOT NULL DEFAULT 0,
    created_by BIGINT,
    updated_by BIGINT,
    PRIMARY KEY (id),
    CONSTRAINT uk_iot_capability_model_tenant_key UNIQUE (tenant_id, model_key)
);

CREATE TABLE iot_capability_definition (
    id BIGINT NOT NULL,
    uuid VARCHAR(64) NOT NULL,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    data_scope INTEGER NOT NULL DEFAULT 0,
    capability_model_id BIGINT NOT NULL,
    capability_name VARCHAR(128) NOT NULL,
    capability_kind VARCHAR(32) NOT NULL,
    schema_json TEXT NOT NULL,
    status INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    version BIGINT NOT NULL DEFAULT 0,
    PRIMARY KEY (id),
    CONSTRAINT uk_iot_capability_definition_tenant_model_name
        UNIQUE (tenant_id, capability_model_id, capability_name)
);

CREATE TABLE iot_device (
    id BIGINT NOT NULL,
    uuid VARCHAR(64) NOT NULL,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    data_scope INTEGER NOT NULL DEFAULT 0,
    owner_type VARCHAR(32) NOT NULL,
    owner_id VARCHAR(128) NOT NULL,
    device_key VARCHAR(128) NOT NULL,
    product_id BIGINT NOT NULL,
    hardware_profile_id BIGINT,
    protocol_profile_id BIGINT,
    display_name VARCHAR(200) NOT NULL,
    device_id VARCHAR(128) NOT NULL,
    client_id VARCHAR(128),
    serial_number VARCHAR(128),
    mac_address VARCHAR(128),
    chip_family VARCHAR(64),
    runtime_profile VARCHAR(64),
    firmware_version VARCHAR(64),
    auth_state INTEGER NOT NULL DEFAULT 0,
    lifecycle_state INTEGER NOT NULL DEFAULT 0,
    last_seen_at TIMESTAMP,
    metadata TEXT,
    status INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    version BIGINT NOT NULL DEFAULT 0,
    created_by BIGINT,
    updated_by BIGINT,
    deleted_at TIMESTAMP,
    deleted_by BIGINT,
    PRIMARY KEY (id),
    CONSTRAINT uk_iot_device_uuid UNIQUE (uuid),
    CONSTRAINT uk_iot_device_tenant_device_key UNIQUE (tenant_id, device_key),
    CONSTRAINT uk_iot_device_tenant_product_device_id UNIQUE (tenant_id, product_id, device_id),
    CONSTRAINT uk_iot_device_tenant_client_id UNIQUE (tenant_id, client_id)
);

CREATE INDEX idx_iot_device_tenant_product_status
    ON iot_device (tenant_id, product_id, status);

CREATE INDEX idx_iot_device_tenant_last_seen
    ON iot_device (tenant_id, last_seen_at);

CREATE TABLE iot_device_credential (
    id BIGINT NOT NULL,
    uuid VARCHAR(64) NOT NULL,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    data_scope INTEGER NOT NULL DEFAULT 0,
    device_id VARCHAR(128) NOT NULL,
    credential_type VARCHAR(64) NOT NULL,
    credential_hash VARCHAR(256),
    credential_ref VARCHAR(512),
    expires_at TIMESTAMP,
    status INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    version BIGINT NOT NULL DEFAULT 0,
    created_by BIGINT,
    updated_by BIGINT,
    PRIMARY KEY (id)
);

CREATE INDEX idx_iot_device_credential_tenant_device_status
    ON iot_device_credential (tenant_id, device_id, status);

CREATE TABLE iot_device_binding (
    id BIGINT NOT NULL,
    uuid VARCHAR(64) NOT NULL,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    data_scope INTEGER NOT NULL DEFAULT 0,
    device_id VARCHAR(128) NOT NULL,
    binding_type VARCHAR(64) NOT NULL,
    target_type VARCHAR(64) NOT NULL,
    target_id VARCHAR(128) NOT NULL,
    role VARCHAR(64),
    status INTEGER NOT NULL,
    bound_at TIMESTAMP NOT NULL,
    bound_by BIGINT,
    expires_at TIMESTAMP,
    metadata TEXT,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    version BIGINT NOT NULL DEFAULT 0,
    PRIMARY KEY (id)
);

CREATE INDEX idx_iot_device_binding_tenant_target
    ON iot_device_binding (tenant_id, target_type, target_id, status);

CREATE TABLE iot_gateway_child_device (
    id BIGINT NOT NULL,
    uuid VARCHAR(64) NOT NULL,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    data_scope INTEGER NOT NULL DEFAULT 0,
    gateway_device_id VARCHAR(128) NOT NULL,
    child_device_id VARCHAR(128) NOT NULL,
    topology_role VARCHAR(64) NOT NULL,
    status INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    version BIGINT NOT NULL DEFAULT 0,
    PRIMARY KEY (id),
    CONSTRAINT uk_iot_gateway_child_device_tenant_pair
        UNIQUE (tenant_id, gateway_device_id, child_device_id)
);

CREATE TABLE iot_device_connection (
    id BIGINT NOT NULL,
    uuid VARCHAR(64) NOT NULL,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    data_scope INTEGER NOT NULL DEFAULT 0,
    connection_id VARCHAR(128) NOT NULL,
    device_id VARCHAR(128),
    transport VARCHAR(64) NOT NULL,
    remote_addr VARCHAR(256),
    status INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    version BIGINT NOT NULL DEFAULT 0,
    PRIMARY KEY (id),
    CONSTRAINT uk_iot_device_connection_tenant_connection UNIQUE (tenant_id, connection_id)
);

CREATE INDEX idx_iot_device_connection_tenant_device_created
    ON iot_device_connection (tenant_id, device_id, created_at);

CREATE TABLE iot_device_session (
    id BIGINT NOT NULL,
    uuid VARCHAR(64) NOT NULL,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    data_scope INTEGER NOT NULL DEFAULT 0,
    device_id VARCHAR(128) NOT NULL,
    session_id VARCHAR(128) NOT NULL,
    connection_id VARCHAR(128) NOT NULL,
    protocol_id VARCHAR(128) NOT NULL,
    adapter_id VARCHAR(128) NOT NULL,
    node_id VARCHAR(128),
    status INTEGER NOT NULL,
    connected_at TIMESTAMP NOT NULL,
    last_seen_at TIMESTAMP,
    disconnected_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    version BIGINT NOT NULL DEFAULT 0,
    PRIMARY KEY (id),
    CONSTRAINT uk_iot_device_session_tenant_session UNIQUE (tenant_id, session_id)
);

CREATE INDEX idx_iot_device_session_tenant_device_status
    ON iot_device_session (tenant_id, device_id, status);

CREATE TABLE iot_device_online_lease (
    id BIGINT NOT NULL,
    uuid VARCHAR(64) NOT NULL,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    data_scope INTEGER NOT NULL DEFAULT 0,
    device_id VARCHAR(128) NOT NULL,
    session_id VARCHAR(128) NOT NULL,
    node_id VARCHAR(128) NOT NULL,
    lease_expires_at TIMESTAMP NOT NULL,
    status INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    version BIGINT NOT NULL DEFAULT 0,
    PRIMARY KEY (id),
    CONSTRAINT uk_iot_device_online_lease_tenant_device UNIQUE (tenant_id, device_id)
);

CREATE INDEX idx_iot_device_online_lease_tenant_expires
    ON iot_device_online_lease (tenant_id, lease_expires_at);

CREATE TABLE iot_command (
    id BIGINT NOT NULL,
    uuid VARCHAR(64) NOT NULL,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    data_scope INTEGER NOT NULL DEFAULT 0,
    command_id VARCHAR(128) NOT NULL,
    device_id VARCHAR(128) NOT NULL,
    session_id VARCHAR(128),
    capability_name VARCHAR(128) NOT NULL,
    command_name VARCHAR(128) NOT NULL,
    request_payload TEXT NOT NULL,
    status INTEGER NOT NULL,
    idempotency_key VARCHAR(128),
    timeout_at TIMESTAMP,
    ack_at TIMESTAMP,
    result_at TIMESTAMP,
    trace_id VARCHAR(128),
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    version BIGINT NOT NULL DEFAULT 0,
    created_by BIGINT,
    updated_by BIGINT,
    PRIMARY KEY (id),
    CONSTRAINT uk_iot_command_tenant_command_id UNIQUE (tenant_id, command_id),
    CONSTRAINT uk_iot_command_tenant_idempotency_key UNIQUE (tenant_id, idempotency_key)
);

CREATE INDEX idx_iot_command_tenant_device_status_created
    ON iot_command (tenant_id, device_id, status, created_at);

CREATE INDEX idx_iot_command_tenant_status_timeout
    ON iot_command (tenant_id, status, timeout_at);

CREATE TABLE iot_command_delivery (
    id BIGINT NOT NULL,
    uuid VARCHAR(64) NOT NULL,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    data_scope INTEGER NOT NULL DEFAULT 0,
    command_id VARCHAR(128) NOT NULL,
    session_id VARCHAR(128),
    delivery_state VARCHAR(64) NOT NULL,
    status INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    version BIGINT NOT NULL DEFAULT 0,
    PRIMARY KEY (id)
);

CREATE INDEX idx_iot_command_delivery_tenant_session_status
    ON iot_command_delivery (tenant_id, session_id, status);

CREATE TABLE iot_command_result (
    id BIGINT NOT NULL,
    uuid VARCHAR(64) NOT NULL,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    data_scope INTEGER NOT NULL DEFAULT 0,
    command_id VARCHAR(128) NOT NULL,
    result_payload TEXT,
    result_code VARCHAR(128),
    status INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    version BIGINT NOT NULL DEFAULT 0,
    PRIMARY KEY (id)
);

CREATE INDEX idx_iot_command_result_tenant_command
    ON iot_command_result (tenant_id, command_id);

CREATE TABLE iot_device_twin (
    id BIGINT NOT NULL,
    uuid VARCHAR(64) NOT NULL,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    data_scope INTEGER NOT NULL DEFAULT 0,
    device_id VARCHAR(128) NOT NULL,
    desired_version BIGINT NOT NULL DEFAULT 0,
    reported_version BIGINT NOT NULL DEFAULT 0,
    status INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    version BIGINT NOT NULL DEFAULT 0,
    PRIMARY KEY (id),
    CONSTRAINT uk_iot_device_twin_tenant_device UNIQUE (tenant_id, device_id)
);

CREATE TABLE iot_device_twin_property (
    id BIGINT NOT NULL,
    uuid VARCHAR(64) NOT NULL,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    data_scope INTEGER NOT NULL DEFAULT 0,
    device_id VARCHAR(128) NOT NULL,
    property_name VARCHAR(128) NOT NULL,
    desired_value TEXT,
    desired_version BIGINT NOT NULL DEFAULT 0,
    desired_updated_at TIMESTAMP,
    reported_value TEXT,
    reported_version BIGINT NOT NULL DEFAULT 0,
    reported_updated_at TIMESTAMP,
    status INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    version BIGINT NOT NULL DEFAULT 0,
    PRIMARY KEY (id),
    CONSTRAINT uk_iot_twin_property_tenant_device_property
        UNIQUE (tenant_id, device_id, property_name)
);

CREATE INDEX idx_iot_twin_property_tenant_device_property
    ON iot_device_twin_property (tenant_id, device_id, property_name);

CREATE TABLE iot_telemetry_latest (
    id BIGINT NOT NULL,
    uuid VARCHAR(64) NOT NULL,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    data_scope INTEGER NOT NULL DEFAULT 0,
    device_id VARCHAR(128) NOT NULL,
    metric_key VARCHAR(128) NOT NULL,
    metric_value TEXT NOT NULL,
    metric_type VARCHAR(32) NOT NULL,
    measured_at TIMESTAMP NOT NULL,
    status INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    version BIGINT NOT NULL DEFAULT 0,
    PRIMARY KEY (id),
    CONSTRAINT uk_iot_telemetry_latest_tenant_device_key
        UNIQUE (tenant_id, device_id, metric_key)
);

CREATE INDEX idx_iot_telemetry_latest_tenant_device_key
    ON iot_telemetry_latest (tenant_id, device_id, metric_key);

CREATE TABLE iot_telemetry_event (
    id BIGINT NOT NULL,
    uuid VARCHAR(64) NOT NULL,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    data_scope INTEGER NOT NULL DEFAULT 0,
    device_id VARCHAR(128) NOT NULL,
    metric_key VARCHAR(128) NOT NULL,
    metric_value TEXT NOT NULL,
    measured_at TIMESTAMP NOT NULL,
    status INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    version BIGINT NOT NULL DEFAULT 0,
    PRIMARY KEY (id)
);

CREATE INDEX idx_iot_telemetry_event_tenant_device_time
    ON iot_telemetry_event (tenant_id, device_id, measured_at);

CREATE TABLE iot_device_event (
    id BIGINT NOT NULL,
    uuid VARCHAR(64) NOT NULL,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    data_scope INTEGER NOT NULL DEFAULT 0,
    device_id VARCHAR(128) NOT NULL,
    event_type VARCHAR(128) NOT NULL,
    event_payload TEXT NOT NULL,
    status INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    version BIGINT NOT NULL DEFAULT 0,
    PRIMARY KEY (id)
);

CREATE INDEX idx_iot_device_event_tenant_device_time
    ON iot_device_event (tenant_id, device_id, created_at);

CREATE TABLE iot_security_event (
    id BIGINT NOT NULL,
    uuid VARCHAR(64) NOT NULL,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    data_scope INTEGER NOT NULL DEFAULT 0,
    security_event_type VARCHAR(128) NOT NULL,
    severity VARCHAR(64) NOT NULL,
    actor_type VARCHAR(64),
    actor_id VARCHAR(128),
    status INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    version BIGINT NOT NULL DEFAULT 0,
    trace_id VARCHAR(128),
    PRIMARY KEY (id)
);

CREATE INDEX idx_iot_security_event_tenant_time
    ON iot_security_event (tenant_id, created_at);

CREATE TABLE iot_firmware_artifact (
    id BIGINT NOT NULL,
    uuid VARCHAR(64) NOT NULL,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    data_scope INTEGER NOT NULL DEFAULT 0,
    owner_type VARCHAR(32) NOT NULL,
    owner_id VARCHAR(128) NOT NULL,
    artifact_key VARCHAR(128) NOT NULL,
    version_name VARCHAR(128) NOT NULL,
    file_name VARCHAR(256) NOT NULL,
    storage_uri VARCHAR(512) NOT NULL,
    size_bytes BIGINT NOT NULL,
    sha256 VARCHAR(128) NOT NULL,
    signature TEXT,
    signature_algorithm VARCHAR(64),
    target_chip_family VARCHAR(64),
    target_runtime_profile VARCHAR(64),
    status INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    version BIGINT NOT NULL DEFAULT 0,
    created_by BIGINT,
    updated_by BIGINT,
    PRIMARY KEY (id),
    CONSTRAINT uk_iot_firmware_artifact_tenant_key UNIQUE (tenant_id, artifact_key)
);

CREATE TABLE iot_firmware_rollout (
    id BIGINT NOT NULL,
    uuid VARCHAR(64) NOT NULL,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    data_scope INTEGER NOT NULL DEFAULT 0,
    owner_type VARCHAR(32) NOT NULL,
    owner_id VARCHAR(128) NOT NULL,
    artifact_id BIGINT NOT NULL,
    rollout_policy TEXT NOT NULL,
    status INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    version BIGINT NOT NULL DEFAULT 0,
    created_by BIGINT,
    updated_by BIGINT,
    PRIMARY KEY (id)
);

CREATE INDEX idx_iot_firmware_rollout_tenant_status
    ON iot_firmware_rollout (tenant_id, status);

CREATE TABLE iot_firmware_rollout_target (
    id BIGINT NOT NULL,
    uuid VARCHAR(64) NOT NULL,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    data_scope INTEGER NOT NULL DEFAULT 0,
    rollout_id BIGINT NOT NULL,
    target_type VARCHAR(64) NOT NULL,
    target_id VARCHAR(128) NOT NULL,
    status INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    version BIGINT NOT NULL DEFAULT 0,
    PRIMARY KEY (id)
);

CREATE INDEX idx_iot_firmware_rollout_target_tenant_rollout
    ON iot_firmware_rollout_target (tenant_id, rollout_id);

CREATE TABLE iot_firmware_deployment (
    id BIGINT NOT NULL,
    uuid VARCHAR(64) NOT NULL,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    data_scope INTEGER NOT NULL DEFAULT 0,
    rollout_id BIGINT,
    device_id VARCHAR(128) NOT NULL,
    deployment_state VARCHAR(64) NOT NULL,
    status INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    version BIGINT NOT NULL DEFAULT 0,
    PRIMARY KEY (id)
);

CREATE INDEX idx_iot_firmware_deployment_tenant_device_status
    ON iot_firmware_deployment (tenant_id, device_id, status);

CREATE TABLE iot_provisioning_challenge (
    id BIGINT NOT NULL,
    uuid VARCHAR(64) NOT NULL,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    data_scope INTEGER NOT NULL DEFAULT 0,
    challenge_id VARCHAR(128) NOT NULL,
    device_hint VARCHAR(128),
    expires_at TIMESTAMP NOT NULL,
    status INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    version BIGINT NOT NULL DEFAULT 0,
    PRIMARY KEY (id),
    CONSTRAINT uk_iot_provisioning_challenge_tenant_id UNIQUE (tenant_id, challenge_id)
);

CREATE INDEX idx_iot_provisioning_challenge_tenant_expires
    ON iot_provisioning_challenge (tenant_id, expires_at);

CREATE TABLE iot_activation_record (
    id BIGINT NOT NULL,
    uuid VARCHAR(64) NOT NULL,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    data_scope INTEGER NOT NULL DEFAULT 0,
    activation_id VARCHAR(128) NOT NULL,
    device_id VARCHAR(128),
    activation_profile VARCHAR(128) NOT NULL,
    status INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    version BIGINT NOT NULL DEFAULT 0,
    PRIMARY KEY (id)
);

CREATE INDEX idx_iot_activation_record_tenant_device
    ON iot_activation_record (tenant_id, device_id);

CREATE TABLE iot_protocol_message_dead_letter (
    id BIGINT NOT NULL,
    uuid VARCHAR(64) NOT NULL,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    data_scope INTEGER NOT NULL DEFAULT 0,
    protocol_id VARCHAR(128) NOT NULL,
    adapter_id VARCHAR(128) NOT NULL,
    device_id VARCHAR(128),
    reason_code VARCHAR(128) NOT NULL,
    payload_ref VARCHAR(512),
    status INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    version BIGINT NOT NULL DEFAULT 0,
    trace_id VARCHAR(128),
    PRIMARY KEY (id)
);

CREATE INDEX idx_iot_protocol_dead_letter_tenant_created
    ON iot_protocol_message_dead_letter (tenant_id, created_at);

CREATE TABLE iot_outbox_event (
    id BIGINT NOT NULL,
    uuid VARCHAR(64) NOT NULL,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    data_scope INTEGER NOT NULL DEFAULT 0,
    event_id VARCHAR(128) NOT NULL,
    event_type VARCHAR(128) NOT NULL,
    aggregate_type VARCHAR(128) NOT NULL,
    aggregate_id VARCHAR(128) NOT NULL,
    payload TEXT NOT NULL,
    status INTEGER NOT NULL,
    next_attempt_at TIMESTAMP,
    attempt_count INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL,
    published_at TIMESTAMP,
    trace_id VARCHAR(128),
    PRIMARY KEY (id),
    CONSTRAINT uk_iot_outbox_event_tenant_event_id UNIQUE (tenant_id, event_id)
);

CREATE INDEX idx_iot_outbox_event_status_next_attempt
    ON iot_outbox_event (status, next_attempt_at);

CREATE TABLE iot_inbox_event (
    id BIGINT NOT NULL,
    uuid VARCHAR(64) NOT NULL,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    data_scope INTEGER NOT NULL DEFAULT 0,
    source_system VARCHAR(128) NOT NULL,
    message_id VARCHAR(128) NOT NULL,
    consumer_name VARCHAR(128) NOT NULL,
    status INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    version BIGINT NOT NULL DEFAULT 0,
    PRIMARY KEY (id),
    CONSTRAINT uk_iot_inbox_event_consumer_message
        UNIQUE (source_system, message_id, consumer_name)
);

CREATE TABLE iot_audit_log (
    id BIGINT NOT NULL,
    uuid VARCHAR(64) NOT NULL,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    data_scope INTEGER NOT NULL DEFAULT 0,
    operator_id BIGINT,
    action VARCHAR(128) NOT NULL,
    target_type VARCHAR(128) NOT NULL,
    target_id VARCHAR(128) NOT NULL,
    status INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    version BIGINT NOT NULL DEFAULT 0,
    trace_id VARCHAR(128),
    PRIMARY KEY (id)
);

CREATE INDEX idx_iot_audit_log_tenant_created
    ON iot_audit_log (tenant_id, created_at);

CREATE TABLE iot_protocol_ingest_record (
    id BIGINT NOT NULL,
    uuid VARCHAR(64) NOT NULL,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    data_scope INTEGER NOT NULL DEFAULT 0,
    protocol_id VARCHAR(128) NOT NULL,
    adapter_id VARCHAR(128) NOT NULL,
    device_id VARCHAR(128),
    message_id VARCHAR(128),
    correlation_id VARCHAR(128),
    idempotency_key VARCHAR(256) NOT NULL,
    trace_id VARCHAR(128),
    status INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    version BIGINT NOT NULL DEFAULT 0,
    PRIMARY KEY (id),
    CONSTRAINT uk_iot_protocol_ingest_tenant_idempotency
        UNIQUE (tenant_id, idempotency_key)
);

CREATE INDEX idx_iot_protocol_ingest_tenant_message
    ON iot_protocol_ingest_record (tenant_id, protocol_id, message_id);
"#
}
