use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::{params, Connection};
use sdkwork_aiot_storage::AiotStorageAssociation;

use crate::ensure_device_schema;

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

pub struct SqliteSqlxCredentialRepository {
    connection: Arc<Mutex<Connection>>,
}

impl SqliteSqlxCredentialRepository {
    pub fn new_in_memory() -> Result<Self, rusqlite::Error> {
        let connection = Connection::open_in_memory()?;
        ensure_device_schema(&connection)?;
        Ok(Self {
            connection: Arc::new(Mutex::new(connection)),
        })
    }

    pub fn open(path: impl AsRef<Path>) -> Result<Self, rusqlite::Error> {
        let connection = Connection::open(path)?;
        ensure_device_schema(&connection)?;
        Ok(Self {
            connection: Arc::new(Mutex::new(connection)),
        })
    }

    pub fn verify_bearer_token(&self, device_id: &str, token: &str) -> bool {
        if device_id.trim().is_empty() || token.trim().is_empty() {
            return false;
        }
        let token_hash = sha256_hex(token.as_bytes());
        let connection = match self.connection.lock() {
            Ok(connection) => connection,
            Err(_) => return false,
        };
        let now = current_rfc3339_timestamp();
        connection
            .query_row(
                "SELECT COUNT(1) FROM iot_device_credential
                 WHERE device_id = ?1
                   AND credential_hash = ?2
                   AND status = ?3
                   AND (expires_at IS NULL OR expires_at > ?4)",
                params![device_id, token_hash, CREDENTIAL_STATUS_ACTIVE, now],
                |row| row.get::<_, i64>(0),
            )
            .map(|count| count > 0)
            .unwrap_or(false)
    }

    pub fn create_credential(
        &self,
        command: SqliteCredentialCreateCommand,
    ) -> Result<SqliteDeviceCredentialRecord, SqliteCredentialRepositoryError> {
        let mut connection = self
            .connection
            .lock()
            .map_err(|_| SqliteCredentialRepositoryError::PersistenceFailure)?;
        let tx = connection
            .transaction()
            .map_err(|_| SqliteCredentialRepositoryError::PersistenceFailure)?;

        let next_id: i64 = tx
            .query_row(
                "SELECT COALESCE(MAX(id), 0) + 1 FROM iot_device_credential",
                [],
                |row| row.get(0),
            )
            .map_err(|_| SqliteCredentialRepositoryError::PersistenceFailure)?;
        let credential_id = format!("credential-{next_id:04}");
        let issued_secret = generate_device_secret(&command.device_id, next_id);
        let credential_hash = sha256_hex(issued_secret.as_bytes());
        let now = current_rfc3339_timestamp();

        tx.execute(
            "INSERT INTO iot_device_credential (
                id, uuid, tenant_id, organization_id, data_scope, device_id,
                credential_type, credential_hash, credential_ref, expires_at,
                status, created_at, updated_at, version
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, 0)",
            params![
                next_id,
                credential_id,
                command.association.tenant_id,
                command.association.organization_id,
                command.association.data_scope,
                command.device_id,
                command.credential_type,
                credential_hash,
                credential_id,
                command.expires_at,
                CREDENTIAL_STATUS_ACTIVE,
                now,
                now,
            ],
        )
        .map_err(|_| SqliteCredentialRepositoryError::PersistenceFailure)?;
        tx.commit()
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
    }

    pub fn list_credentials(
        &self,
        association: &AiotStorageAssociation,
        device_id: &str,
    ) -> Vec<SqliteDeviceCredentialRecord> {
        let connection = match self.connection.lock() {
            Ok(connection) => connection,
            Err(_) => return Vec::new(),
        };
        let mut stmt = match connection.prepare(
            "SELECT uuid, tenant_id, organization_id, device_id, credential_type, status,
                    expires_at, created_at, updated_at
             FROM iot_device_credential
             WHERE tenant_id = ?1 AND organization_id = ?2 AND device_id = ?3
             ORDER BY id ASC",
        ) {
            Ok(stmt) => stmt,
            Err(_) => return Vec::new(),
        };
        let rows = stmt.query_map(
            params![
                association.tenant_id,
                association.organization_id,
                device_id
            ],
            |row| {
                Ok(SqliteDeviceCredentialRecord {
                    credential_id: row.get(0)?,
                    tenant_id: row.get(1)?,
                    organization_id: row.get(2)?,
                    device_id: row.get(3)?,
                    credential_type: row.get(4)?,
                    status: credential_status_label(row.get::<_, i32>(5)?).to_string(),
                    expires_at: read_optional_timestamp_column(row, 6)?,
                    created_at: read_timestamp_column(row, 7)?,
                    revoked_at: None,
                    issued_secret: None,
                })
            },
        );
        match rows {
            Ok(rows) => rows.filter_map(Result::ok).collect(),
            Err(_) => Vec::new(),
        }
    }

    pub fn get_credential(
        &self,
        association: &AiotStorageAssociation,
        device_id: &str,
        credential_id: &str,
    ) -> Option<SqliteDeviceCredentialRecord> {
        let connection = self.connection.lock().ok()?;
        connection
            .query_row(
                "SELECT uuid, tenant_id, organization_id, device_id, credential_type, status,
                        expires_at, created_at
                 FROM iot_device_credential
                 WHERE tenant_id = ?1 AND organization_id = ?2 AND device_id = ?3 AND uuid = ?4",
                params![
                    association.tenant_id,
                    association.organization_id,
                    device_id,
                    credential_id
                ],
                |row| {
                    Ok(SqliteDeviceCredentialRecord {
                        credential_id: row.get(0)?,
                        tenant_id: row.get(1)?,
                        organization_id: row.get(2)?,
                        device_id: row.get(3)?,
                        credential_type: row.get(4)?,
                        status: credential_status_label(row.get::<_, i32>(5)?).to_string(),
                        expires_at: read_optional_timestamp_column(row, 6)?,
                        created_at: read_timestamp_column(row, 7)?,
                        revoked_at: None,
                        issued_secret: None,
                    })
                },
            )
            .ok()
    }

    pub fn revoke_credential(
        &self,
        association: &AiotStorageAssociation,
        device_id: &str,
        credential_id: &str,
    ) -> Result<(), SqliteCredentialRepositoryError> {
        let connection = self
            .connection
            .lock()
            .map_err(|_| SqliteCredentialRepositoryError::PersistenceFailure)?;
        let now = current_rfc3339_timestamp();
        let updated = connection
            .execute(
                "UPDATE iot_device_credential
                 SET status = ?1, updated_at = ?2
                 WHERE tenant_id = ?3 AND organization_id = ?4 AND device_id = ?5 AND uuid = ?6",
                params![
                    CREDENTIAL_STATUS_REVOKED,
                    now,
                    association.tenant_id,
                    association.organization_id,
                    device_id,
                    credential_id
                ],
            )
            .map_err(|_| SqliteCredentialRepositoryError::PersistenceFailure)?;
        if updated == 0 {
            return Err(SqliteCredentialRepositoryError::CredentialNotFound);
        }
        Ok(())
    }
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

fn read_timestamp_column(row: &rusqlite::Row<'_>, index: usize) -> Result<String, rusqlite::Error> {
    row.get::<_, String>(index)
        .or_else(|_| row.get::<_, i64>(index).map(|value| value.to_string()))
}

fn read_optional_timestamp_column(
    row: &rusqlite::Row<'_>,
    index: usize,
) -> Result<Option<String>, rusqlite::Error> {
    if matches!(row.get_ref(index)?, rusqlite::types::ValueRef::Null) {
        return Ok(None);
    }
    read_timestamp_column(row, index).map(Some)
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
