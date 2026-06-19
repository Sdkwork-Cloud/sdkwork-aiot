use std::time::{SystemTime, UNIX_EPOCH};

use sdkwork_aiot_storage::AiotStorageAssociation;
use sqlx::Row;

use crate::schema::ensure_device_schema;
use crate::sqlite_sync::{sqlite_connect_url, BlockingSqlitePool};

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

impl From<sqlx::Error> for SqlitePersistedEntityError {
    fn from(_: sqlx::Error) -> Self {
        Self::PersistenceFailure
    }
}

pub struct SqlitePersistedEntityRepository {
    db: BlockingSqlitePool,
}

impl SqlitePersistedEntityRepository {
    pub fn new_in_memory() -> Result<Self, sqlx::Error> {
        Self::open("file:sdkwork-aiot-admin-entity?mode=memory&cache=shared")
    }

    pub fn from_blocking_pool(db: BlockingSqlitePool) -> Result<Self, sqlx::Error> {
        ensure_device_schema(&db)?;
        Ok(Self { db })
    }

    pub fn open(path_or_uri: impl AsRef<std::path::Path>) -> Result<Self, sqlx::Error> {
        let url = sqlite_connect_url(path_or_uri.as_ref().to_string_lossy().as_ref());
        let db = BlockingSqlitePool::connect(&url)?;
        Self::from_blocking_pool(db)
    }

    pub fn upsert_entity(
        &self,
        association: &AiotStorageAssociation,
        entity_kind: &str,
        entity_key: &str,
        payload_json: &str,
    ) -> Result<(), SqlitePersistedEntityError> {
        let now = current_timestamp();
        let association = association.clone();
        let entity_kind = entity_kind.to_string();
        let entity_key = entity_key.to_string();
        let payload_json = payload_json.to_string();
        self.db
            .with_transaction(|tx| {
                Box::pin(async move {
                    let updated = sqlx::query(
                        "UPDATE iot_admin_entity
                         SET payload_json = ?1, updated_at = ?2, status = ?3
                         WHERE tenant_id = ?4 AND organization_id = ?5 AND entity_kind = ?6 AND entity_key = ?7",
                    )
                    .bind(&payload_json)
                    .bind(&now)
                    .bind(ENTITY_STATUS_ACTIVE)
                    .bind(association.tenant_id)
                    .bind(association.organization_id)
                    .bind(&entity_kind)
                    .bind(&entity_key)
                    .execute(&mut **tx)
                    .await
                    .map_err(|_| SqlitePersistedEntityError::PersistenceFailure)?
                    .rows_affected();
                    if updated > 0 {
                        return Ok(());
                    }

                    let next_id: i64 = sqlx::query_scalar(
                        "SELECT COALESCE(MAX(id), 0) + 1 FROM iot_admin_entity",
                    )
                    .fetch_one(&mut **tx)
                    .await
                    .map_err(|_| SqlitePersistedEntityError::PersistenceFailure)?;
                    let uuid = format!("admin-entity-{next_id:08}");
                    sqlx::query(
                        "INSERT INTO iot_admin_entity (
                            id, uuid, tenant_id, organization_id, data_scope, entity_kind, entity_key,
                            payload_json, status, created_at, updated_at, version
                        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, 0)",
                    )
                    .bind(next_id)
                    .bind(&uuid)
                    .bind(association.tenant_id)
                    .bind(association.organization_id)
                    .bind(association.data_scope)
                    .bind(&entity_kind)
                    .bind(&entity_key)
                    .bind(&payload_json)
                    .bind(ENTITY_STATUS_ACTIVE)
                    .bind(&now)
                    .bind(&now)
                    .execute(&mut **tx)
                    .await
                    .map_err(|_| SqlitePersistedEntityError::PersistenceFailure)?;
                    Ok(())
                })
            })
    }

    pub fn get_entity(
        &self,
        association: &AiotStorageAssociation,
        entity_kind: &str,
        entity_key: &str,
    ) -> Option<SqlitePersistedEntityRecord> {
        self.db
            .run(async {
                let row = sqlx::query(
                    "SELECT entity_kind, entity_key, payload_json
                     FROM iot_admin_entity
                     WHERE tenant_id = ?1 AND organization_id = ?2 AND entity_kind = ?3 AND entity_key = ?4 AND status = ?5",
                )
                .bind(association.tenant_id)
                .bind(association.organization_id)
                .bind(entity_kind)
                .bind(entity_key)
                .bind(ENTITY_STATUS_ACTIVE)
                .fetch_optional(self.db.pool())
                .await?;
                row.as_ref()
                    .map(row_to_entity_record)
                    .transpose()
            })
            .ok()
            .flatten()
    }

    pub fn list_entities(
        &self,
        association: &AiotStorageAssociation,
        entity_kind: &str,
    ) -> Vec<SqlitePersistedEntityRecord> {
        self.db
            .run::<_, Vec<SqlitePersistedEntityRecord>, sqlx::Error>(async {
                let rows = sqlx::query(
                    "SELECT entity_kind, entity_key, payload_json
                     FROM iot_admin_entity
                     WHERE tenant_id = ?1 AND organization_id = ?2 AND entity_kind = ?3 AND status = ?4
                     ORDER BY id ASC",
                )
                .bind(association.tenant_id)
                .bind(association.organization_id)
                .bind(entity_kind)
                .bind(ENTITY_STATUS_ACTIVE)
                .fetch_all(self.db.pool())
                .await?;
                Ok(rows
                    .into_iter()
                    .filter_map(|row| row_to_entity_record(&row).ok())
                    .collect::<Vec<_>>())
            })
            .unwrap_or_default()
    }

    pub fn delete_entity(
        &self,
        association: &AiotStorageAssociation,
        entity_kind: &str,
        entity_key: &str,
    ) -> Result<(), SqlitePersistedEntityError> {
        let now = current_timestamp();
        let updated = self
            .db
            .run(async {
                sqlx::query(
                    "UPDATE iot_admin_entity
                     SET status = 0, updated_at = ?1
                     WHERE tenant_id = ?2 AND organization_id = ?3 AND entity_kind = ?4 AND entity_key = ?5 AND status = ?6",
                )
                .bind(&now)
                .bind(association.tenant_id)
                .bind(association.organization_id)
                .bind(entity_kind)
                .bind(entity_key)
                .bind(ENTITY_STATUS_ACTIVE)
                .execute(self.db.pool())
                .await
                .map(|result| result.rows_affected())
            })
            .map_err(|_| SqlitePersistedEntityError::PersistenceFailure)?;
        if updated == 0 {
            return Err(SqlitePersistedEntityError::NotFound);
        }
        Ok(())
    }
}

fn row_to_entity_record(
    row: &sqlx::sqlite::SqliteRow,
) -> Result<SqlitePersistedEntityRecord, sqlx::Error> {
    Ok(SqlitePersistedEntityRecord {
        entity_kind: row.try_get("entity_kind")?,
        entity_key: row.try_get("entity_key")?,
        payload_json: row.try_get("payload_json")?,
    })
}

fn current_timestamp() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|value| value.as_secs())
        .unwrap_or(0);
    format!("{seconds}")
}
