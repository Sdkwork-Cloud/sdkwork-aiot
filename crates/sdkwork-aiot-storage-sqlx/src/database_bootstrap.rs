//! AIoT device database bootstrap through `sdkwork-database`.

use sdkwork_database_config::{DatabaseConfig, DatabaseEngine, DeploymentMode};
use sdkwork_database_sqlx::{
    create_pool_from_config, create_pool_from_env, DatabasePool, PoolError,
};

use crate::sqlite_sync::BlockingSqlitePool;

pub const AIOT_DEVICE_DATABASE_SERVICE_NAME: &str = "AIOT_DEVICE";

/// Canonical SQLite memory configuration for tests and default dev processes.
pub fn aiot_device_sqlite_memory_config() -> DatabaseConfig {
    DatabaseConfig {
        engine: DatabaseEngine::Sqlite,
        url: "file:sdkwork-aiot-device-db?mode=memory&cache=shared".to_owned(),
        mode: DeploymentMode::Standalone,
        table_prefix: "iot_".to_owned(),
        max_connections: 5,
        min_connections: 1,
        ..DatabaseConfig::default()
    }
}

/// SQLite file configuration for durable device persistence.
pub fn aiot_device_sqlite_file_config(path: &str) -> DatabaseConfig {
    DatabaseConfig {
        engine: DatabaseEngine::Sqlite,
        url: format!("sqlite:{}?mode=rwc", path.replace('\\', "/")),
        mode: DeploymentMode::Standalone,
        table_prefix: "iot_".to_owned(),
        max_connections: 5,
        min_connections: 1,
        ..DatabaseConfig::default()
    }
}

/// Resolves the active device database config from explicit path or shared memory default.
pub fn resolve_device_database_config(device_db_path: Option<&str>) -> DatabaseConfig {
    device_db_path
        .map(aiot_device_sqlite_file_config)
        .unwrap_or_else(aiot_device_sqlite_memory_config)
}

/// Returns the SQLite URL from a validated SDKWork database config.
pub fn sqlite_url_from_config(config: &DatabaseConfig) -> &str {
    assert_eq!(config.engine, DatabaseEngine::Sqlite);
    assert!(
        config.table_prefix.starts_with("iot_"),
        "AIoT device database must use iot_ table prefix"
    );
    &config.url
}

/// Opens a shared-memory SQLite pool through `sdkwork-database-sqlx`.
pub async fn aiot_device_sqlite_memory_pool() -> Result<DatabasePool, PoolError> {
    create_pool_from_config(aiot_device_sqlite_memory_config()).await
}

/// Loads a device database pool from `SDKWORK_AIOT_DEVICE_DATABASE_*` environment variables.
pub async fn aiot_device_pool_from_env() -> Result<Option<DatabasePool>, PoolError> {
    create_pool_from_env(AIOT_DEVICE_DATABASE_SERVICE_NAME).await
}

/// Opens a synchronous SQLite pool through `sdkwork-database-sqlx` for legacy sync repositories.
pub fn aiot_device_blocking_pool(
    device_db_path: Option<&str>,
) -> Result<BlockingSqlitePool, PoolError> {
    let config = resolve_device_database_config(device_db_path);
    debug_assert!(!sqlite_url_from_config(&config).is_empty());
    let runtime = tokio::runtime::Runtime::new()
        .map_err(|error| PoolError::DatabaseConfig(format!("tokio runtime: {error}")))?;
    let database_pool = runtime.block_on(create_pool_from_config(config))?;
    let sqlite_pool = database_pool.as_sqlite().ok_or_else(|| {
        PoolError::DatabaseConfig("AIoT device database requires SQLite engine".to_owned())
    })?;
    BlockingSqlitePool::from_pool(sqlite_pool.clone()).map_err(PoolError::PoolCreation)
}
