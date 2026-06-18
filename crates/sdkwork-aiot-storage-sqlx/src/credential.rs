use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use sdkwork_aiot_storage::AiotStorageAssociation;
use sqlx::Row;

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

    pub fn open(path_or_uri: impl AsRef<Path>) -> Result<Self, sqlx::Error> {
        let url = sqlite_connect_url(path_or_uri.as_ref().to_string_lossy().as_ref());
        let db = BlockingSqlitePool::connect(&url)?;
        ensure_device_schema(&db)?;
        Ok(Self { db })
    }

    pub fn verify_bearer_token(&self, device_id: &str, token: &str) -> bool {
        if device_id.trim().is_empty() || token.trim().is_empty() {
            return false;
        }
        let token_hash = sha256_hex(token.as_bytes());
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
                let credential_hash = sha256_hex(issued_secret.as_bytes());

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
                row.as_ref()
                    .map(|row| row_to_credential_record(row))
                    .transpose()
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
    sha256_hex(seed.as_bytes())
}

fn current_rfc3339_timestamp() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|value| value.as_secs())
        .unwrap_or(0);
    format!("{seconds}")
}

fn sha256_hex(input: &[u8]) -> String {
    let digest = sha256_digest(input);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn sha256_digest(input: &[u8]) -> [u8; 32] {
    let mut message = input.to_vec();
    let bit_len = (message.len() as u64) * 8;
    message.push(0x80);
    while (message.len() % 64) != 56 {
        message.push(0);
    }
    message.extend_from_slice(&bit_len.to_be_bytes());

    let mut h = [
        0x6a09e667u32,
        0xbb67ae85,
        0x3c6ef372,
        0xa54ff53a,
        0x510e527f,
        0x9b05688c,
        0x1f83d9ab,
        0x5be0cd19,
    ];
    for chunk in message.chunks(64) {
        let mut w = [0u32; 64];
        for (index, word) in chunk.chunks(4).enumerate() {
            w[index] = u32::from_be_bytes([word[0], word[1], word[2], word[3]]);
        }
        for index in 16..64 {
            let s0 = w[index - 15].rotate_right(7)
                ^ w[index - 15].rotate_right(18)
                ^ (w[index - 15] >> 3);
            let s1 = w[index - 2].rotate_right(17)
                ^ w[index - 2].rotate_right(19)
                ^ (w[index - 2] >> 10);
            w[index] = w[index - 16]
                .wrapping_add(s0)
                .wrapping_add(w[index - 7])
                .wrapping_add(s1);
        }
        let mut a = h[0];
        let mut b = h[1];
        let mut c = h[2];
        let mut d = h[3];
        let mut e = h[4];
        let mut f = h[5];
        let mut g = h[6];
        let mut hh = h[7];
        for index in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = hh
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[index])
                .wrapping_add(w[index]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);
            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }
        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(hh);
    }
    let mut out = [0u8; 32];
    for (index, word) in h.iter().enumerate() {
        out[index * 4..index * 4 + 4].copy_from_slice(&word.to_be_bytes());
    }
    out
}

const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];
