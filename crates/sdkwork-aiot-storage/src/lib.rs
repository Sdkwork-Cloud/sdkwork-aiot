use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AiotTable {
    pub name: &'static str,
    pub group: &'static str,
}

pub const IOT_DATABASE_PREFIX: &str = "iot";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableProfile {
    TenantOwnerEntity,
    TenantEntity,
    RelationEntity,
    RuntimeFact,
    Projection,
    EventLog,
    OutboxEvent,
    InboxEvent,
    AuditLog,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiotTableContract {
    pub name: &'static str,
    pub group: &'static str,
    pub profile: TableProfile,
    pub system_of_record: bool,
    pub write_owner: &'static str,
    pub required_columns: Vec<&'static str>,
    pub required_indexes: Vec<&'static str>,
}

impl AiotTableContract {
    pub fn new(name: &'static str, group: &'static str, profile: TableProfile) -> Self {
        Self {
            name,
            group,
            profile,
            system_of_record: true,
            write_owner: "sdkwork-aiot-core",
            required_columns: vec![
                "id",
                "uuid",
                "tenant_id",
                "organization_id",
                "data_scope",
                "created_at",
                "updated_at",
                "version",
                "status",
            ],
            required_indexes: Vec::new(),
        }
    }

    pub fn read_model(mut self) -> Self {
        self.system_of_record = false;
        self
    }

    pub fn with_write_owner(mut self, write_owner: &'static str) -> Self {
        self.write_owner = write_owner;
        self
    }

    pub fn with_column(mut self, column: &'static str) -> Self {
        self.required_columns.push(column);
        self
    }

    pub fn with_index(mut self, index: &'static str) -> Self {
        self.required_indexes.push(index);
        self
    }
}

impl AiotTable {
    pub const fn new(name: &'static str, group: &'static str) -> Self {
        Self { name, group }
    }
}

pub const IOT_TABLES: &[AiotTable] = &[
    AiotTable::new("iot_product", "product_catalog"),
    AiotTable::new("iot_hardware_profile", "hardware_profile"),
    AiotTable::new("iot_protocol_profile", "protocol_profile"),
    AiotTable::new("iot_capability_model", "capability_model"),
    AiotTable::new("iot_capability_definition", "capability_model"),
    AiotTable::new("iot_device", "device_registry"),
    AiotTable::new("iot_device_credential", "device_registry"),
    AiotTable::new("iot_device_binding", "device_registry"),
    AiotTable::new("iot_gateway_child_device", "edge_gateway"),
    AiotTable::new("iot_device_connection", "session_runtime"),
    AiotTable::new("iot_device_session", "session_runtime"),
    AiotTable::new("iot_device_online_lease", "session_runtime"),
    AiotTable::new("iot_command", "command_control"),
    AiotTable::new("iot_command_delivery", "command_control"),
    AiotTable::new("iot_command_result", "command_control"),
    AiotTable::new("iot_device_twin", "device_twin"),
    AiotTable::new("iot_device_twin_property", "device_twin"),
    AiotTable::new("iot_telemetry_latest", "telemetry_event"),
    AiotTable::new("iot_telemetry_event", "telemetry_event"),
    AiotTable::new("iot_device_event", "telemetry_event"),
    AiotTable::new("iot_security_event", "telemetry_event"),
    AiotTable::new("iot_firmware_artifact", "ota_provisioning"),
    AiotTable::new("iot_firmware_rollout", "ota_provisioning"),
    AiotTable::new("iot_firmware_rollout_target", "ota_provisioning"),
    AiotTable::new("iot_firmware_deployment", "ota_provisioning"),
    AiotTable::new("iot_provisioning_challenge", "ota_provisioning"),
    AiotTable::new("iot_activation_record", "ota_provisioning"),
    AiotTable::new("iot_outbox_event", "eventing"),
    AiotTable::new("iot_inbox_event", "eventing"),
    AiotTable::new("iot_audit_log", "eventing"),
    AiotTable::new("iot_protocol_ingest_record", "protocol_runtime"),
    AiotTable::new("iot_protocol_message_dead_letter", "protocol_runtime"),
];

pub fn standard_protocol_ingest_storage_ports() -> Vec<&'static str> {
    vec![
        "DeviceSessionRepository",
        "DeviceOnlineLeaseRepository",
        "DeviceCredentialRepository",
        "TelemetryRepository",
        "DeviceEventRepository",
        "DeviceTwinRepository",
        "CommandDeliveryRepository",
        "CommandResultRepository",
        "FirmwareDeploymentRepository",
        "ProvisioningChallengeRepository",
        "GatewayTopologyRepository",
        "SecurityEventRepository",
        "ProtocolDeadLetterRepository",
        "OutboxEventRepository",
    ]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AiotStorageWriteKind {
    OpenSession,
    Authenticate,
    KeepAlive,
    CloseSession,
    ProvisionDevice,
    RecordTelemetry,
    ApplyDesiredTwin,
    DispatchCommand,
    RecordCommandAck,
    RecordCommandResult,
    ProcessMediaFrame,
    EvaluateOta,
    DispatchOta,
    UpdateGatewayTopology,
    RecordSecurityEvent,
    RecordDiagnostic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AiotStorageWriteRoute {
    pub repository_port: &'static str,
    pub primary_table: &'static str,
    pub transactional: bool,
}

impl AiotStorageWriteKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::OpenSession => "open_session",
            Self::Authenticate => "authenticate",
            Self::KeepAlive => "keep_alive",
            Self::CloseSession => "close_session",
            Self::ProvisionDevice => "provision_device",
            Self::RecordTelemetry => "record_telemetry",
            Self::ApplyDesiredTwin => "apply_desired_twin",
            Self::DispatchCommand => "dispatch_command",
            Self::RecordCommandAck => "record_command_ack",
            Self::RecordCommandResult => "record_command_result",
            Self::ProcessMediaFrame => "process_media_frame",
            Self::EvaluateOta => "evaluate_ota",
            Self::DispatchOta => "dispatch_ota",
            Self::UpdateGatewayTopology => "update_gateway_topology",
            Self::RecordSecurityEvent => "record_security_event",
            Self::RecordDiagnostic => "record_diagnostic",
        }
    }

    pub fn storage_route(&self) -> AiotStorageWriteRoute {
        let (repository_port, primary_table) = match self {
            Self::OpenSession | Self::CloseSession => {
                ("DeviceSessionRepository", "iot_device_session")
            }
            Self::Authenticate => ("DeviceCredentialRepository", "iot_device_credential"),
            Self::KeepAlive => ("DeviceOnlineLeaseRepository", "iot_device_online_lease"),
            Self::ProvisionDevice => (
                "ProvisioningChallengeRepository",
                "iot_provisioning_challenge",
            ),
            Self::RecordTelemetry => ("TelemetryRepository", "iot_telemetry_event"),
            Self::ApplyDesiredTwin => ("DeviceTwinRepository", "iot_device_twin_property"),
            Self::DispatchCommand | Self::RecordCommandAck => {
                ("CommandDeliveryRepository", "iot_command_delivery")
            }
            Self::RecordCommandResult => ("CommandResultRepository", "iot_command_result"),
            Self::ProcessMediaFrame | Self::RecordDiagnostic => {
                ("DeviceEventRepository", "iot_device_event")
            }
            Self::EvaluateOta | Self::DispatchOta => {
                ("FirmwareDeploymentRepository", "iot_firmware_deployment")
            }
            Self::UpdateGatewayTopology => {
                ("GatewayTopologyRepository", "iot_gateway_child_device")
            }
            Self::RecordSecurityEvent => ("SecurityEventRepository", "iot_security_event"),
        };

        AiotStorageWriteRoute {
            repository_port,
            primary_table,
            transactional: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiotOutboxWriteIntent {
    pub event_type: String,
    pub aggregate_type: String,
    pub aggregate_id: String,
    pub topic: String,
    pub initial_status: &'static str,
}

impl AiotOutboxWriteIntent {
    pub fn new(
        event_type: impl Into<String>,
        aggregate_type: impl Into<String>,
        aggregate_id: impl Into<String>,
        topic: impl Into<String>,
    ) -> Self {
        Self {
            event_type: event_type.into(),
            aggregate_type: aggregate_type.into(),
            aggregate_id: aggregate_id.into(),
            topic: topic.into(),
            initial_status: "pending",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiotStorageAssociation {
    pub tenant_id: i64,
    pub organization_id: i64,
    pub user_id: Option<i64>,
    pub owner_type: Option<String>,
    pub owner_id: Option<String>,
    pub data_scope: i32,
    pub created_by: Option<i64>,
    pub updated_by: Option<i64>,
    pub deleted_by: Option<i64>,
}

impl AiotStorageAssociation {
    pub fn tenant_org(tenant_id: i64, organization_id: i64) -> Self {
        Self {
            tenant_id,
            organization_id,
            user_id: None,
            owner_type: None,
            owner_id: None,
            data_scope: 0,
            created_by: None,
            updated_by: None,
            deleted_by: None,
        }
    }

    pub fn platform_shared() -> Self {
        Self::tenant_org(0, 0)
    }

    pub fn with_user_id(mut self, user_id: i64) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn with_owner(
        mut self,
        owner_type: impl Into<String>,
        owner_id: impl Into<String>,
    ) -> Self {
        self.owner_type = Some(owner_type.into());
        self.owner_id = Some(owner_id.into());
        self
    }

    pub fn with_data_scope(mut self, data_scope: i32) -> Self {
        self.data_scope = data_scope;
        self
    }

    pub fn with_created_by(mut self, created_by: i64) -> Self {
        self.created_by = Some(created_by);
        self
    }

    pub fn with_updated_by(mut self, updated_by: i64) -> Self {
        self.updated_by = Some(updated_by);
        self
    }

    pub fn with_deleted_by(mut self, deleted_by: i64) -> Self {
        self.deleted_by = Some(deleted_by);
        self
    }
}

impl Default for AiotStorageAssociation {
    fn default() -> Self {
        Self::platform_shared()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiotProtocolStorageCommand {
    pub association: AiotStorageAssociation,
    pub protocol_id: String,
    pub adapter_id: String,
    pub device_id: String,
    pub kind: AiotStorageWriteKind,
    pub primary_table: &'static str,
    pub message_id: Option<String>,
    pub correlation_id: Option<String>,
    pub session_id: Option<String>,
    pub trace_id: Option<String>,
    pub idempotency_key: Option<String>,
    pub requires_transaction: bool,
    pub dead_letter_on_failure: bool,
    pub outbox: Option<AiotOutboxWriteIntent>,
}

impl AiotProtocolStorageCommand {
    pub fn new(
        protocol_id: impl Into<String>,
        adapter_id: impl Into<String>,
        device_id: impl Into<String>,
        kind: AiotStorageWriteKind,
        primary_table: &'static str,
    ) -> Self {
        let protocol_id = protocol_id.into();
        let adapter_id = adapter_id.into();
        let device_id = device_id.into();
        let idempotency_key = Some(format!(
            "{}:{}:{}:{}:{}",
            protocol_id,
            adapter_id,
            device_id,
            kind.as_str(),
            primary_table
        ));

        Self {
            association: AiotStorageAssociation::default(),
            protocol_id,
            adapter_id,
            device_id,
            kind,
            primary_table,
            message_id: None,
            correlation_id: None,
            session_id: None,
            trace_id: None,
            idempotency_key,
            requires_transaction: true,
            dead_letter_on_failure: true,
            outbox: None,
        }
    }

    pub fn with_message_id(mut self, message_id: impl Into<String>) -> Self {
        self.message_id = Some(message_id.into());
        self
    }

    pub fn with_association(mut self, association: AiotStorageAssociation) -> Self {
        self.association = association;
        self
    }

    pub fn with_correlation_id(mut self, correlation_id: impl Into<String>) -> Self {
        self.correlation_id = Some(correlation_id.into());
        self
    }

    pub fn with_session_id(mut self, session_id: impl Into<String>) -> Self {
        self.session_id = Some(session_id.into());
        self
    }

    pub fn with_trace_id(mut self, trace_id: impl Into<String>) -> Self {
        self.trace_id = Some(trace_id.into());
        self
    }

    pub fn with_idempotency_key(mut self, idempotency_key: impl Into<String>) -> Self {
        self.idempotency_key = Some(idempotency_key.into());
        self
    }

    pub fn with_outbox(mut self, outbox: AiotOutboxWriteIntent) -> Self {
        self.outbox = Some(outbox);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiotStorageWriteReceipt {
    pub accepted: bool,
    pub duplicate: bool,
    pub write_kind: Option<AiotStorageWriteKind>,
    pub primary_table: Option<String>,
    pub outbox_event_type: Option<String>,
    pub dead_letter_reason: Option<String>,
}

impl AiotStorageWriteReceipt {
    pub fn accepted(
        write_kind: AiotStorageWriteKind,
        primary_table: impl Into<String>,
        outbox_event_type: Option<String>,
    ) -> Self {
        Self {
            accepted: true,
            duplicate: false,
            write_kind: Some(write_kind),
            primary_table: Some(primary_table.into()),
            outbox_event_type,
            dead_letter_reason: None,
        }
    }

    pub fn dead_lettered(reason_code: impl Into<String>) -> Self {
        Self {
            accepted: false,
            duplicate: false,
            write_kind: None,
            primary_table: None,
            outbox_event_type: None,
            dead_letter_reason: Some(reason_code.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiotPrimaryWriteRecord {
    pub association: AiotStorageAssociation,
    pub protocol_id: String,
    pub adapter_id: String,
    pub device_id: String,
    pub write_kind: AiotStorageWriteKind,
    pub primary_table: &'static str,
    pub idempotency_key: String,
    pub message_id: Option<String>,
    pub correlation_id: Option<String>,
    pub trace_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiotOutboxEventRecord {
    pub association: AiotStorageAssociation,
    pub event_type: String,
    pub aggregate_type: String,
    pub aggregate_id: String,
    pub topic: String,
    pub initial_status: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct InMemoryProtocolIngestSnapshot {
    pub primary_writes: Vec<AiotPrimaryWriteRecord>,
    pub outbox_events: Vec<AiotOutboxEventRecord>,
    pub dead_letters: Vec<AiotProtocolDeadLetterIntent>,
}

#[derive(Debug, Default)]
struct InMemoryProtocolIngestState {
    seen_idempotency_keys: BTreeSet<String>,
    snapshot: InMemoryProtocolIngestSnapshot,
}

#[derive(Debug, Clone, Default)]
pub struct InMemoryProtocolIngestUnitOfWork {
    state: Arc<Mutex<InMemoryProtocolIngestState>>,
}

impl InMemoryProtocolIngestUnitOfWork {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn snapshot(&self) -> InMemoryProtocolIngestSnapshot {
        self.state
            .lock()
            .expect("in-memory uow poisoned")
            .snapshot
            .clone()
    }
}

impl AiotProtocolIngestUnitOfWork for InMemoryProtocolIngestUnitOfWork {
    fn execute_protocol_command(
        &self,
        command: &AiotProtocolStorageCommand,
    ) -> AiotStorageWriteReceipt {
        let mut state = self.state.lock().expect("in-memory uow poisoned");
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

        if !state.seen_idempotency_keys.insert(idempotency_key.clone()) {
            let mut receipt = AiotStorageWriteReceipt::accepted(
                command.kind,
                command.primary_table,
                command
                    .outbox
                    .as_ref()
                    .map(|outbox| outbox.event_type.clone()),
            );
            receipt.duplicate = true;
            return receipt;
        }

        state.snapshot.primary_writes.push(AiotPrimaryWriteRecord {
            association: command.association.clone(),
            protocol_id: command.protocol_id.clone(),
            adapter_id: command.adapter_id.clone(),
            device_id: command.device_id.clone(),
            write_kind: command.kind,
            primary_table: command.primary_table,
            idempotency_key,
            message_id: command.message_id.clone(),
            correlation_id: command.correlation_id.clone(),
            trace_id: command.trace_id.clone(),
        });

        if let Some(outbox) = &command.outbox {
            state.snapshot.outbox_events.push(AiotOutboxEventRecord {
                association: command.association.clone(),
                event_type: outbox.event_type.clone(),
                aggregate_type: outbox.aggregate_type.clone(),
                aggregate_id: outbox.aggregate_id.clone(),
                topic: outbox.topic.clone(),
                initial_status: outbox.initial_status,
            });
        }

        AiotStorageWriteReceipt::accepted(
            command.kind,
            command.primary_table,
            command
                .outbox
                .as_ref()
                .map(|outbox| outbox.event_type.clone()),
        )
    }

    fn record_dead_letter(&self, intent: &AiotProtocolDeadLetterIntent) -> AiotStorageWriteReceipt {
        self.state
            .lock()
            .expect("in-memory uow poisoned")
            .snapshot
            .dead_letters
            .push(intent.clone());

        AiotStorageWriteReceipt::dead_lettered(intent.reason_code.clone())
    }
}

pub trait AiotProtocolIngestUnitOfWork {
    fn execute_protocol_command(
        &self,
        command: &AiotProtocolStorageCommand,
    ) -> AiotStorageWriteReceipt;

    fn record_dead_letter(&self, intent: &AiotProtocolDeadLetterIntent) -> AiotStorageWriteReceipt;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiotProtocolDeadLetterIntent {
    pub association: AiotStorageAssociation,
    pub protocol_id: String,
    pub adapter_id: String,
    pub device_id: Option<String>,
    pub reason_code: String,
    pub payload_ref: Option<String>,
    pub raw_payload: Option<String>,
    pub trace_id: Option<String>,
}

impl AiotProtocolDeadLetterIntent {
    pub fn new(
        protocol_id: impl Into<String>,
        adapter_id: impl Into<String>,
        reason_code: impl Into<String>,
        payload_ref: impl Into<String>,
    ) -> Self {
        Self {
            association: AiotStorageAssociation::default(),
            protocol_id: protocol_id.into(),
            adapter_id: adapter_id.into(),
            device_id: None,
            reason_code: reason_code.into(),
            payload_ref: Some(payload_ref.into()),
            raw_payload: None,
            trace_id: None,
        }
    }

    pub fn from_protocol_error(
        protocol_id: impl Into<String>,
        adapter_id: impl Into<String>,
        reason_code: impl Into<String>,
        payload_ref: impl Into<String>,
    ) -> Self {
        Self::new(protocol_id, adapter_id, reason_code, payload_ref)
    }

    pub fn with_device_id(mut self, device_id: impl Into<String>) -> Self {
        self.device_id = Some(device_id.into());
        self
    }

    pub fn with_association(mut self, association: AiotStorageAssociation) -> Self {
        self.association = association;
        self
    }

    pub fn with_trace_id(mut self, trace_id: impl Into<String>) -> Self {
        self.trace_id = Some(trace_id.into());
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AiotRetryPolicy {
    pub max_attempts: u32,
    pub dead_letter_after_attempts: u32,
    pub initial_backoff_seconds: u64,
    pub max_backoff_seconds: u64,
}

impl AiotRetryPolicy {
    pub fn standard() -> Self {
        Self {
            max_attempts: 12,
            dead_letter_after_attempts: 12,
            initial_backoff_seconds: 1,
            max_backoff_seconds: 300,
        }
    }

    pub fn backoff_seconds(&self, attempt: u32) -> u64 {
        let multiplier = 1_u64.checked_shl(attempt.min(63)).unwrap_or(u64::MAX);
        self.initial_backoff_seconds
            .saturating_mul(multiplier)
            .min(self.max_backoff_seconds)
    }

    pub fn should_dead_letter(&self, attempt_count: u32) -> bool {
        attempt_count >= self.dead_letter_after_attempts
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiotStorageFailure {
    pub reason_code: String,
    pub attempt_count: u32,
    pub retryable: bool,
}

impl AiotStorageFailure {
    pub fn retryable(reason_code: impl Into<String>, attempt_count: u32) -> Self {
        Self {
            reason_code: reason_code.into(),
            attempt_count,
            retryable: true,
        }
    }

    pub fn fatal(reason_code: impl Into<String>) -> Self {
        Self {
            reason_code: reason_code.into(),
            attempt_count: 0,
            retryable: false,
        }
    }

    pub fn disposition(&self, policy: &AiotRetryPolicy) -> AiotStorageFailureDisposition {
        if !self.retryable || policy.should_dead_letter(self.attempt_count) {
            return AiotStorageFailureDisposition::DeadLetter {
                reason_code: self.reason_code.clone(),
            };
        }

        AiotStorageFailureDisposition::Retry {
            next_attempt_in_seconds: policy.backoff_seconds(self.attempt_count),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AiotStorageFailureDisposition {
    Retry { next_attempt_in_seconds: u64 },
    DeadLetter { reason_code: String },
}

pub fn standard_dead_letter_reason_catalog() -> Vec<&'static str> {
    vec![
        "decode.unsupported_message_type",
        "decode.invalid_frame",
        "auth.failed",
        "auth.replay_detected",
        "storage.write_failed",
        "command.route_unavailable",
        "ota.policy_violation",
        "backpressure.dead_letter_non_critical",
    ]
}

pub fn table_contract(name: &str) -> Option<AiotTableContract> {
    match name {
        "iot_product" => Some(
            AiotTableContract::new(
                "iot_product",
                "product_catalog",
                TableProfile::TenantOwnerEntity,
            )
            .with_column("owner_type")
            .with_column("owner_id")
            .with_index("uk_iot_product_tenant_code"),
        ),
        "iot_hardware_profile" => Some(
            AiotTableContract::new(
                "iot_hardware_profile",
                "hardware_profile",
                TableProfile::TenantOwnerEntity,
            )
            .with_column("chip_family")
            .with_column("runtime_profile")
            .with_index("idx_iot_hardware_profile_tenant_chip"),
        ),
        "iot_protocol_profile" => Some(
            AiotTableContract::new(
                "iot_protocol_profile",
                "protocol_profile",
                TableProfile::TenantOwnerEntity,
            )
            .with_column("default_protocol_id")
            .with_index("idx_iot_protocol_profile_tenant_protocol"),
        ),
        "iot_capability_model" => Some(
            AiotTableContract::new(
                "iot_capability_model",
                "capability_model",
                TableProfile::TenantOwnerEntity,
            )
            .with_column("owner_type")
            .with_column("owner_id"),
        ),
        "iot_capability_definition" => Some(
            AiotTableContract::new(
                "iot_capability_definition",
                "capability_model",
                TableProfile::TenantEntity,
            )
            .with_column("capability_model_id")
            .with_column("capability_name")
            .with_column("capability_kind")
            .with_column("schema_json")
            .with_index("uk_iot_capability_definition_tenant_model_name"),
        ),
        "iot_device" => Some(
            AiotTableContract::new(
                "iot_device",
                "device_registry",
                TableProfile::TenantOwnerEntity,
            )
            .with_column("owner_type")
            .with_column("owner_id")
            .with_column("device_key")
            .with_column("product_id")
            .with_column("device_id")
            .with_column("client_id")
            .with_column("chip_family")
            .with_column("runtime_profile")
            .with_column("firmware_version")
            .with_column("created_by")
            .with_column("updated_by")
            .with_index("uk_iot_device_uuid")
            .with_index("uk_iot_device_tenant_device_key")
            .with_index("uk_iot_device_tenant_product_device_id")
            .with_index("uk_iot_device_tenant_client_id")
            .with_index("idx_iot_device_tenant_product_status")
            .with_index("idx_iot_device_tenant_last_seen"),
        ),
        "iot_device_credential" => Some(
            AiotTableContract::new(
                "iot_device_credential",
                "device_registry",
                TableProfile::TenantEntity,
            )
            .with_column("device_id")
            .with_column("credential_type")
            .with_column("credential_hash")
            .with_column("credential_ref")
            .with_column("expires_at")
            .with_index("idx_iot_device_credential_tenant_device_status"),
        ),
        "iot_device_binding" => Some(
            AiotTableContract::new(
                "iot_device_binding",
                "device_registry",
                TableProfile::RelationEntity,
            )
            .with_column("device_id")
            .with_column("target_type")
            .with_column("target_id")
            .with_index("idx_iot_device_binding_tenant_target"),
        ),
        "iot_gateway_child_device" => Some(
            AiotTableContract::new(
                "iot_gateway_child_device",
                "edge_gateway",
                TableProfile::RelationEntity,
            )
            .with_column("gateway_device_id")
            .with_column("child_device_id")
            .with_column("topology_role")
            .with_index("uk_iot_gateway_child_device_tenant_pair"),
        ),
        "iot_device_connection" => Some(
            AiotTableContract::new(
                "iot_device_connection",
                "session_runtime",
                TableProfile::RuntimeFact,
            )
            .with_column("connection_id")
            .with_column("device_id")
            .with_column("transport")
            .with_column("remote_addr")
            .with_index("idx_iot_device_connection_tenant_device_created"),
        ),
        "iot_device_session" => Some(
            AiotTableContract::new(
                "iot_device_session",
                "session_runtime",
                TableProfile::RuntimeFact,
            )
            .with_column("device_id")
            .with_column("session_id")
            .with_column("protocol_id")
            .with_index("idx_iot_device_session_tenant_device_status"),
        ),
        "iot_device_online_lease" => Some(
            AiotTableContract::new(
                "iot_device_online_lease",
                "session_runtime",
                TableProfile::Projection,
            )
            .read_model()
            .with_write_owner("sdkwork-aiot-runtime")
            .with_column("device_id")
            .with_column("session_id")
            .with_column("node_id")
            .with_column("lease_expires_at")
            .with_index("idx_iot_device_online_lease_tenant_expires"),
        ),
        "iot_command" => Some(
            AiotTableContract::new("iot_command", "command_control", TableProfile::RuntimeFact)
                .with_column("command_id")
                .with_column("device_id")
                .with_column("capability_name")
                .with_column("command_name")
                .with_column("idempotency_key")
                .with_column("trace_id")
                .with_index("idx_iot_command_tenant_device_status_created")
                .with_index("idx_iot_command_tenant_status_timeout")
                .with_index("uk_iot_command_tenant_idempotency_key"),
        ),
        "iot_command_delivery" => Some(
            AiotTableContract::new(
                "iot_command_delivery",
                "command_control",
                TableProfile::RuntimeFact,
            )
            .with_column("command_id")
            .with_column("session_id")
            .with_column("delivery_state")
            .with_index("idx_iot_command_delivery_tenant_session_status"),
        ),
        "iot_command_result" => Some(
            AiotTableContract::new(
                "iot_command_result",
                "command_control",
                TableProfile::RuntimeFact,
            )
            .with_column("command_id")
            .with_column("result_payload")
            .with_column("result_code")
            .with_index("idx_iot_command_result_tenant_command"),
        ),
        "iot_device_twin" => Some(
            AiotTableContract::new("iot_device_twin", "device_twin", TableProfile::Projection)
                .read_model()
                .with_column("device_id")
                .with_column("desired_version")
                .with_column("reported_version")
                .with_index("uk_iot_device_twin_tenant_device"),
        ),
        "iot_device_twin_property" => Some(
            AiotTableContract::new(
                "iot_device_twin_property",
                "device_twin",
                TableProfile::Projection,
            )
            .read_model()
            .with_column("device_id")
            .with_column("property_name")
            .with_column("desired_value")
            .with_column("reported_value")
            .with_index("idx_iot_twin_property_tenant_device_property"),
        ),
        "iot_telemetry_latest" => Some(
            AiotTableContract::new(
                "iot_telemetry_latest",
                "telemetry_event",
                TableProfile::Projection,
            )
            .read_model()
            .with_column("device_id")
            .with_column("metric_key")
            .with_column("metric_value")
            .with_index("idx_iot_telemetry_latest_tenant_device_key"),
        ),
        "iot_telemetry_event" => Some(
            AiotTableContract::new(
                "iot_telemetry_event",
                "telemetry_event",
                TableProfile::EventLog,
            )
            .with_column("device_id")
            .with_column("metric_key")
            .with_column("metric_value")
            .with_column("measured_at")
            .with_index("idx_iot_telemetry_event_tenant_device_time"),
        ),
        "iot_device_event" => Some(
            AiotTableContract::new(
                "iot_device_event",
                "telemetry_event",
                TableProfile::EventLog,
            )
            .with_column("device_id")
            .with_column("event_type")
            .with_column("event_payload")
            .with_index("idx_iot_device_event_tenant_device_time"),
        ),
        "iot_security_event" => Some(
            AiotTableContract::new(
                "iot_security_event",
                "telemetry_event",
                TableProfile::EventLog,
            )
            .with_column("security_event_type")
            .with_column("severity")
            .with_column("actor_type")
            .with_column("actor_id")
            .with_index("idx_iot_security_event_tenant_time"),
        ),
        "iot_firmware_artifact" => Some(
            AiotTableContract::new(
                "iot_firmware_artifact",
                "ota_provisioning",
                TableProfile::TenantOwnerEntity,
            )
            .with_column("artifact_key")
            .with_column("sha256")
            .with_column("signature")
            .with_index("uk_iot_firmware_artifact_tenant_key"),
        ),
        "iot_firmware_rollout" => Some(
            AiotTableContract::new(
                "iot_firmware_rollout",
                "ota_provisioning",
                TableProfile::TenantOwnerEntity,
            )
            .with_column("owner_type")
            .with_column("owner_id")
            .with_column("artifact_id")
            .with_column("rollout_policy")
            .with_index("idx_iot_firmware_rollout_tenant_status"),
        ),
        "iot_firmware_rollout_target" => Some(
            AiotTableContract::new(
                "iot_firmware_rollout_target",
                "ota_provisioning",
                TableProfile::RelationEntity,
            )
            .with_column("rollout_id")
            .with_column("target_type")
            .with_column("target_id")
            .with_index("idx_iot_firmware_rollout_target_tenant_rollout"),
        ),
        "iot_firmware_deployment" => Some(
            AiotTableContract::new(
                "iot_firmware_deployment",
                "ota_provisioning",
                TableProfile::RuntimeFact,
            )
            .with_column("rollout_id")
            .with_column("device_id")
            .with_column("deployment_state")
            .with_index("idx_iot_firmware_deployment_tenant_device_status"),
        ),
        "iot_provisioning_challenge" => Some(
            AiotTableContract::new(
                "iot_provisioning_challenge",
                "ota_provisioning",
                TableProfile::RuntimeFact,
            )
            .with_column("challenge_id")
            .with_column("device_hint")
            .with_column("expires_at")
            .with_index("idx_iot_provisioning_challenge_tenant_expires"),
        ),
        "iot_activation_record" => Some(
            AiotTableContract::new(
                "iot_activation_record",
                "ota_provisioning",
                TableProfile::EventLog,
            )
            .with_column("activation_id")
            .with_column("device_id")
            .with_column("activation_profile")
            .with_index("idx_iot_activation_record_tenant_device"),
        ),
        "iot_outbox_event" => Some(
            AiotTableContract::new("iot_outbox_event", "eventing", TableProfile::OutboxEvent)
                .with_column("event_id")
                .with_column("event_type")
                .with_column("aggregate_type")
                .with_column("aggregate_id")
                .with_column("payload")
                .with_column("next_attempt_at")
                .with_column("attempt_count")
                .with_column("trace_id")
                .with_index("idx_iot_outbox_event_status_next_attempt"),
        ),
        "iot_inbox_event" => Some(
            AiotTableContract::new("iot_inbox_event", "eventing", TableProfile::InboxEvent)
                .with_column("source_system")
                .with_column("message_id")
                .with_column("consumer_name")
                .with_index("uk_iot_inbox_event_consumer_message"),
        ),
        "iot_audit_log" => Some(
            AiotTableContract::new("iot_audit_log", "eventing", TableProfile::AuditLog)
                .with_column("operator_id")
                .with_column("action")
                .with_column("target_type")
                .with_column("target_id")
                .with_index("idx_iot_audit_log_tenant_created"),
        ),
        "iot_protocol_ingest_record" => Some(
            AiotTableContract::new(
                "iot_protocol_ingest_record",
                "protocol_runtime",
                TableProfile::InboxEvent,
            )
            .with_write_owner("sdkwork-aiot-storage-sqlx")
            .with_column("protocol_id")
            .with_column("adapter_id")
            .with_column("device_id")
            .with_column("message_id")
            .with_column("correlation_id")
            .with_column("idempotency_key")
            .with_column("trace_id")
            .with_index("uk_iot_protocol_ingest_tenant_idempotency")
            .with_index("idx_iot_protocol_ingest_tenant_message"),
        ),
        "iot_protocol_message_dead_letter" => Some(
            AiotTableContract::new(
                "iot_protocol_message_dead_letter",
                "protocol_runtime",
                TableProfile::EventLog,
            )
            .with_write_owner("sdkwork-aiot-runtime")
            .with_column("protocol_id")
            .with_column("adapter_id")
            .with_column("reason_code")
            .with_column("payload_ref")
            .with_index("idx_iot_protocol_dead_letter_tenant_created"),
        ),
        _ => None,
    }
}
