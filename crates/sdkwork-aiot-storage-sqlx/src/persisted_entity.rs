use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::{params, Connection};
use sdkwork_aiot_storage::AiotStorageAssociation;

use crate::ensure_device_schema;

const ENTITY_STATUS_ACTIVE: i32 = 1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SqlitePersistedEntityRecord {
    pub entity_kind: String,
    pub entity_key: String,
    pub payload_json: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SqlitePersistedEntityError {
    PersistenceFailure,
    NotFound,
    Duplicate,
}

pub struct SqlitePersistedEntityRepository {
    connection: Arc<Mutex<Connection>>,
}

impl SqlitePersistedEntityRepository {
    pub fn new_in_memory() -> Result<Self, rusqlite::Error> {
        let connection = Connection::open_in_memory()?;
        ensure_device_schema(&connection)?;
        Ok(Self {
            connection: Arc::new(Mutex::new(connection)),
        })
    }

    pub fn open(path: impl AsRef<std::path::Path>) -> Result<Self, rusqlite::Error> {
        let connection = Connection::open(path)?;
        ensure_device_schema(&connection)?;
        Ok(Self {
            connection: Arc::new(Mutex::new(connection)),
        })
    }

    pub fn upsert_entity(
        &self,
        association: &AiotStorageAssociation,
        entity_kind: &str,
        entity_key: &str,
        payload_json: &str,
    ) -> Result<(), SqlitePersistedEntityError> {
        let connection = self
            .connection
            .lock()
            .map_err(|_| SqlitePersistedEntityError::PersistenceFailure)?;
        let now = current_timestamp();
        let updated = connection
            .execute(
                "UPDATE iot_admin_entity
                 SET payload_json = ?1, updated_at = ?2, status = ?3
                 WHERE tenant_id = ?4 AND organization_id = ?5 AND entity_kind = ?6 AND entity_key = ?7",
                params![
                    payload_json,
                    now,
                    ENTITY_STATUS_ACTIVE,
                    association.tenant_id,
                    association.organization_id,
                    entity_kind,
                    entity_key,
                ],
            )
            .map_err(|_| SqlitePersistedEntityError::PersistenceFailure)?;
        if updated > 0 {
            return Ok(());
        }

        let next_id: i64 = connection
            .query_row(
                "SELECT COALESCE(MAX(id), 0) + 1 FROM iot_admin_entity",
                [],
                |row| row.get(0),
            )
            .map_err(|_| SqlitePersistedEntityError::PersistenceFailure)?;
        let uuid = format!("admin-entity-{next_id:08}");
        connection
            .execute(
                "INSERT INTO iot_admin_entity (
                    id, uuid, tenant_id, organization_id, data_scope, entity_kind, entity_key,
                    payload_json, status, created_at, updated_at, version
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, 0)",
                params![
                    next_id,
                    uuid,
                    association.tenant_id,
                    association.organization_id,
                    association.data_scope,
                    entity_kind,
                    entity_key,
                    payload_json,
                    ENTITY_STATUS_ACTIVE,
                    now,
                    now,
                ],
            )
            .map_err(|_| SqlitePersistedEntityError::PersistenceFailure)?;
        Ok(())
    }

    pub fn get_entity(
        &self,
        association: &AiotStorageAssociation,
        entity_kind: &str,
        entity_key: &str,
    ) -> Option<SqlitePersistedEntityRecord> {
        let connection = self.connection.lock().ok()?;
        connection
            .query_row(
                "SELECT entity_kind, entity_key, payload_json
                 FROM iot_admin_entity
                 WHERE tenant_id = ?1 AND organization_id = ?2 AND entity_kind = ?3 AND entity_key = ?4 AND status = ?5",
                params![
                    association.tenant_id,
                    association.organization_id,
                    entity_kind,
                    entity_key,
                    ENTITY_STATUS_ACTIVE
                ],
                |row| {
                    Ok(SqlitePersistedEntityRecord {
                        entity_kind: row.get(0)?,
                        entity_key: row.get(1)?,
                        payload_json: row.get(2)?,
                    })
                },
            )
            .ok()
    }

    pub fn list_entities(
        &self,
        association: &AiotStorageAssociation,
        entity_kind: &str,
    ) -> Vec<SqlitePersistedEntityRecord> {
        let connection = match self.connection.lock() {
            Ok(connection) => connection,
            Err(_) => return Vec::new(),
        };
        let mut stmt = match connection.prepare(
            "SELECT entity_kind, entity_key, payload_json
             FROM iot_admin_entity
             WHERE tenant_id = ?1 AND organization_id = ?2 AND entity_kind = ?3 AND status = ?4
             ORDER BY id ASC",
        ) {
            Ok(stmt) => stmt,
            Err(_) => return Vec::new(),
        };
        let rows = stmt.query_map(
            params![
                association.tenant_id,
                association.organization_id,
                entity_kind,
                ENTITY_STATUS_ACTIVE
            ],
            |row| {
                Ok(SqlitePersistedEntityRecord {
                    entity_kind: row.get(0)?,
                    entity_key: row.get(1)?,
                    payload_json: row.get(2)?,
                })
            },
        );
        match rows {
            Ok(rows) => rows.filter_map(Result::ok).collect(),
            Err(_) => Vec::new(),
        }
    }

    pub fn delete_entity(
        &self,
        association: &AiotStorageAssociation,
        entity_kind: &str,
        entity_key: &str,
    ) -> Result<(), SqlitePersistedEntityError> {
        let connection = self
            .connection
            .lock()
            .map_err(|_| SqlitePersistedEntityError::PersistenceFailure)?;
        let now = current_timestamp();
        let updated = connection
            .execute(
                "UPDATE iot_admin_entity
                 SET status = 0, updated_at = ?1
                 WHERE tenant_id = ?2 AND organization_id = ?3 AND entity_kind = ?4 AND entity_key = ?5 AND status = ?6",
                params![
                    now,
                    association.tenant_id,
                    association.organization_id,
                    entity_kind,
                    entity_key,
                    ENTITY_STATUS_ACTIVE
                ],
            )
            .map_err(|_| SqlitePersistedEntityError::PersistenceFailure)?;
        if updated == 0 {
            return Err(SqlitePersistedEntityError::NotFound);
        }
        Ok(())
    }
}

fn current_timestamp() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|value| value.as_secs())
        .unwrap_or(0);
    format!("{seconds}")
}
