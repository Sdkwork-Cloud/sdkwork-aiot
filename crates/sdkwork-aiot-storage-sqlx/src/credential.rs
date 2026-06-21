use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use sdkwork_aiot_storage::AiotStorageAssociation;
use sqlx::Row;

use sdkwork_utils_rust::{is_blank, sha256_hash};

use crate::schema::ensure_device_schema;
use crate::sqlite_sync::{
    read_optional_timestamp_column, read_timestamp_column, sqlite_connect_url, BlockingSqlitePool,
};

const CREDENTIAL_STATUS_ACTIVE: i32 = 1;
const CREDENTIAL_STATUS_REVOKED: i32 = 0;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SqliteDeviceCredentialRecord {
    pub credential_id: String,
    pub tenant_id: i64,
    pub organization_id: i64,
    pub device_id: String,
    pub credential_type: String,
    pub status: String,
    pub expires_at: Option<String>,
    pub created_at: String,
    pub revoked_at: Option<String>,
    pub issued_secret: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SqliteCredentialCreateCommand {
    pub association: AiotStorageAssociation,
    pub device_id: String,
    pub credential_type: String,
    pub expires_at: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SqliteCredentialRepositoryError {
    PersistenceFailure,
    CredentialNotFound,
}

impl From<sqlx::Error> for SqliteCredentialRepositoryError {
    fn from(_: sqlx::Error) -> Self {
        Self::PersistenceFailure
    }
}

pub struct SqliteSqlxCredentialRepository {
    db: BlockingSqlitePool,
}

impl SqliteSqlxCredentialRepository {
    pub fn new_in_memory() -> Result<Self, sqlx::Error> {
        Self::open("file:sdkwork-aiot-credential?mode=memory&cache=shared")
    }

    pub fn from_blocking_pool(db: BlockingSqlitePool) -> Result<Self, sqlx::Error> {
        ensure_device_schema(&db)?;
        Ok(Self { db })
    }

    pub fn open(path_or_uri: impl AsRef<Path>) -> Result<Self, sqlx::Error> {
        let url = sqlite_connect_url(path_or_uri.as_ref().to_string_lossy().as_ref());
        let db = BlockingSqlitePool::connect(&url)?;
        Self::from_blocking_pool(db)
    }

    pub fn verify_bearer_token(&self, device_id: &str, token: &str) -> bool {
        if is_blank(Some(device_id)) || is_blank(Some(token)) {
            return false;
        }
        let token_hash = sha256_hash(token.as_bytes());
        let now = current_rfc3339_timestamp();
        self.db
            .run::<_, bool, sqlx::Error>(async {
                let count: i64 = sqlx::query_scalar(
                    "SELECT COUNT(1) FROM iot_device_credential
                     WHERE device_id = ?1
                       AND credential_hash = ?2
                       AND status = ?3
                       AND (expires_at IS NULL OR expires_at > ?4)",
                )
                .bind(device_id)
                .bind(token_hash)
                .bind(CREDENTIAL_STATUS_ACTIVE)
                .bind(now)
                .fetch_one(self.db.pool())
                .await?;
                Ok(count > 0)
            })
            .unwrap_or(false)
    }

    pub fn create_credential(
        &self,
        command: SqliteCredentialCreateCommand,
    ) -> Result<SqliteDeviceCredentialRecord, SqliteCredentialRepositoryError> {
        let now = current_rfc3339_timestamp();
        self.db.with_transaction(|tx| {
            Box::pin(async move {
                let next_id: i64 = sqlx::query_scalar(
                    "SELECT COALESCE(MAX(id), 0) + 1 FROM iot_device_credential",
                )
                .fetch_one(&mut **tx)
                .await
                .map_err(|_| SqliteCredentialRepositoryError::PersistenceFailure)?;
                let credential_id = format!("credential-{next_id:04}");
                let issued_secret = generate_device_secret(&command.device_id, next_id);
                let credential_hash = sha256_hash(issued_secret.as_bytes());

                sqlx::query(
                    "INSERT INTO iot_device_credential (
                            id, uuid, tenant_id, organization_id, data_scope, device_id,
                            credential_type, credential_hash, credential_ref, expires_at,
                            status, created_at, updated_at, version
                        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, 0)",
                )
                .bind(next_id)
                .bind(&credential_id)
                .bind(command.association.tenant_id)
                .bind(command.association.organization_id)
                .bind(command.association.data_scope)
                .bind(&command.device_id)
                .bind(&command.credential_type)
                .bind(credential_hash)
                .bind(&credential_id)
                .bind(command.expires_at.as_deref())
                .bind(CREDENTIAL_STATUS_ACTIVE)
                .bind(&now)
                .bind(&now)
                .execute(&mut **tx)
                .await
                .map_err(|_| SqliteCredentialRepositoryError::PersistenceFailure)?;

                Ok(SqliteDeviceCredentialRecord {
                    credential_id,
                    tenant_id: command.association.tenant_id,
                    organization_id: command.association.organization_id,
                    device_id: command.device_id,
                    credential_type: command.credential_type,
                    status: "active".to_string(),
                    expires_at: command.expires_at,
                    created_at: now.clone(),
                    revoked_at: None,
                    issued_secret: Some(issued_secret),
                })
            })
        })
    }

    pub fn list_credentials(
        &self,
        association: &AiotStorageAssociation,
        device_id: &str,
    ) -> Vec<SqliteDeviceCredentialRecord> {
        self.db
            .run::<_, Vec<SqliteDeviceCredentialRecord>, sqlx::Error>(async {
                let rows = sqlx::query(
                    "SELECT uuid, tenant_id, organization_id, device_id, credential_type, status,
                            expires_at, created_at, updated_at
                     FROM iot_device_credential
                     WHERE tenant_id = ?1 AND organization_id = ?2 AND device_id = ?3
                     ORDER BY id ASC",
                )
                .bind(association.tenant_id)
                .bind(association.organization_id)
                .bind(device_id)
                .fetch_all(self.db.pool())
                .await?;
                Ok(rows
                    .into_iter()
                    .filter_map(|row| row_to_credential_record(&row).ok())
                    .collect::<Vec<_>>())
            })
            .unwrap_or_default()
    }

    pub fn get_credential(
        &self,
        association: &AiotStorageAssociation,
        device_id: &str,
        credential_id: &str,
    ) -> Option<SqliteDeviceCredentialRecord> {
        self.db
            .run(async {
                let row = sqlx::query(
                    "SELECT uuid, tenant_id, organization_id, device_id, credential_type, status,
                            expires_at, created_at
                     FROM iot_device_credential
                     WHERE tenant_id = ?1 AND organization_id = ?2 AND device_id = ?3 AND uuid = ?4",
                )
                .bind(association.tenant_id)
                .bind(association.organization_id)
                .bind(device_id)
                .bind(credential_id)
                .fetch_optional(self.db.pool())
                .await?;
                row.as_ref().map(row_to_credential_record).transpose()
            })
            .ok()
            .flatten()
    }

    pub fn revoke_credential(
        &self,
        association: &AiotStorageAssociation,
        device_id: &str,
        credential_id: &str,
    ) -> Result<(), SqliteCredentialRepositoryError> {
        let now = current_rfc3339_timestamp();
        let updated = self
            .db
            .run(async {
                sqlx::query(
                    "UPDATE iot_device_credential
                     SET status = ?1, updated_at = ?2
                     WHERE tenant_id = ?3 AND organization_id = ?4 AND device_id = ?5 AND uuid = ?6",
                )
                .bind(CREDENTIAL_STATUS_REVOKED)
                .bind(&now)
                .bind(association.tenant_id)
                .bind(association.organization_id)
                .bind(device_id)
                .bind(credential_id)
                .execute(self.db.pool())
                .await
                .map(|result| result.rows_affected())
            })
            .map_err(|_| SqliteCredentialRepositoryError::PersistenceFailure)?;
        if updated == 0 {
            return Err(SqliteCredentialRepositoryError::CredentialNotFound);
        }
        Ok(())
    }
}

fn row_to_credential_record(
    row: &sqlx::sqlite::SqliteRow,
) -> Result<SqliteDeviceCredentialRecord, sqlx::Error> {
    let status_code: i32 = row.try_get("status")?;
    Ok(SqliteDeviceCredentialRecord {
        credential_id: row.try_get("uuid")?,
        tenant_id: row.try_get("tenant_id")?,
        organization_id: row.try_get("organization_id")?,
        device_id: row.try_get("device_id")?,
        credential_type: row.try_get("credential_type")?,
        status: credential_status_label(status_code).to_string(),
        expires_at: read_optional_timestamp_column(row, "expires_at")?,
        created_at: read_timestamp_column(row, "created_at")?,
        revoked_at: None,
        issued_secret: None,
    })
}

fn credential_status_label(status: i32) -> &'static str {
    if status == CREDENTIAL_STATUS_ACTIVE {
        "active"
    } else {
        "revoked"
    }
}

fn generate_device_secret(device_id: &str, sequence: i64) -> String {
    let seed = format!(
        "{}:{}:{}",
        device_id,
        sequence,
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|value| value.as_nanos())
            .unwrap_or(0)
    );
    sha256_hash(seed.as_bytes())
}

fn current_rfc3339_timestamp() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|value| value.as_secs())
        .unwrap_or(0);
    format!("{seconds}")
}
