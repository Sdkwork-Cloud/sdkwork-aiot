//! Single-pool bootstrap for AIoT device persistence surfaces.

use sdkwork_database_sqlx::PoolError;

use crate::schema::ensure_device_schema;
use crate::{
    aiot_device_blocking_pool, BlockingSqlitePool, SqlitePersistedEntityRepository,
    SqliteSqlxCredentialRepository, SqliteSqlxDeviceRepository,
};

/// Shared SQLite pool backing device, credential, and admin-entity repositories.
#[derive(Debug, Clone)]
pub struct AiotDeviceDatabase {
    pool: BlockingSqlitePool,
}

impl AiotDeviceDatabase {
    pub fn open(device_db_path: Option<&str>) -> Result<Self, PoolError> {
        let pool = aiot_device_blocking_pool(device_db_path)?;
        ensure_device_schema(&pool).map_err(PoolError::PoolCreation)?;
        Ok(Self { pool })
    }

    pub fn blocking_pool(&self) -> BlockingSqlitePool {
        self.pool.clone()
    }

    pub fn device_repository(&self) -> Result<SqliteSqlxDeviceRepository, sqlx::Error> {
        SqliteSqlxDeviceRepository::from_blocking_pool(self.pool.clone())
    }

    pub fn credential_repository(&self) -> Result<SqliteSqlxCredentialRepository, sqlx::Error> {
        SqliteSqlxCredentialRepository::from_blocking_pool(self.pool.clone())
    }

    pub fn persisted_entity_repository(
        &self,
    ) -> Result<SqlitePersistedEntityRepository, sqlx::Error> {
        SqlitePersistedEntityRepository::from_blocking_pool(self.pool.clone())
    }
}

/// Opens the canonical shared device database for HTTP services and gateway persistence.
pub fn open_aiot_device_database(
    device_db_path: Option<&str>,
) -> Result<AiotDeviceDatabase, PoolError> {
    AiotDeviceDatabase::open(device_db_path)
}
