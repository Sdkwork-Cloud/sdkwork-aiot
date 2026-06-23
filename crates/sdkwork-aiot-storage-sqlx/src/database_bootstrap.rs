//! AIoT device database bootstrap through `sdkwork-database`.

use sdkwork_database_config::{DatabaseConfig, DatabaseEngine, DeploymentMode};
use sdkwork_database_sqlx::{
    create_pool_from_config, create_pool_from_env, DatabasePool, PoolError,
};

use crate::outbox_worker::configured_device_db_path_from_env;
use crate::sqlite_sync::BlockingSqlitePool;

pub const AIOT_DEVICE_DATABASE_SERVICE_NAME: &str = "AIOT_DEVICE";

/// Returned when callers request Postgres before Phase K repository migration lands.
pub const POSTGRES_DEVICE_PERSISTENCE_DEFERRED: &str = "Postgres device repositories require Phase K migration; use SDKWORK_AIOT_DEVICE_DB_PATH for SQLite file persistence until SDKWORK_AIOT_DEVICE_DATABASE postgres pools are supported";

const EXPLICIT_DEVICE_DATABASE_ENV_KEYS: &[&str] = &[
    "SDKWORK_AIOT_DEVICE_DATABASE_URL",
    "SDKWORK_AIOT_DEVICE_DATABASE_ENGINE",
    "SDKWORK_AIOT_DEVICE_DATABASE_MODE",
    "SDKWORK_AIOT_DEVICE_DATABASE_TABLE_PREFIX",
];

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
    resolve_device_database_config_from_env(device_db_path)
        .expect("device database config must resolve for sqlite-backed callers")
}

/// Resolves device database config honoring file path, env path, explicit SDKWork database env, or memory default.
pub fn resolve_device_database_config_from_env(
    device_db_path: Option<&str>,
) -> Result<DatabaseConfig, PoolError> {
    if let Some(path) = device_db_path {
        return Ok(aiot_device_sqlite_file_config(path));
    }
    if let Some(path) = configured_device_db_path_from_env() {
        return Ok(aiot_device_sqlite_file_config(&path));
    }
    if explicit_device_database_env_configured() {
        let config = DatabaseConfig::from_env(AIOT_DEVICE_DATABASE_SERVICE_NAME)
            .map_err(|error| PoolError::DatabaseConfig(error.to_string()))?;
        if config.engine == DatabaseEngine::Postgres {
            return Err(PoolError::DatabaseConfig(
                POSTGRES_DEVICE_PERSISTENCE_DEFERRED.to_owned(),
            ));
        }
        return Ok(config);
    }
    Ok(aiot_device_sqlite_memory_config())
}

fn explicit_device_database_env_configured() -> bool {
    EXPLICIT_DEVICE_DATABASE_ENV_KEYS.iter().any(|key| {
        std::env::var(key)
            .ok()
            .filter(|value| !value.is_empty())
            .is_some()
    })
}

/// Returns the SQLite URL from a validated SDKWork database config.
#[allow(dead_code)]
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
    aiot_device_blocking_pool_from_env(device_db_path)
}

/// Opens a synchronous SQLite pool from path, env path, explicit SQLite database env, or memory default.
pub fn aiot_device_blocking_pool_from_env(
    device_db_path: Option<&str>,
) -> Result<BlockingSqlitePool, PoolError> {
    let config = resolve_device_database_config_from_env(device_db_path)?;
    match config.engine {
        DatabaseEngine::Sqlite => {
            let runtime = tokio::runtime::Runtime::new()
                .map_err(|error| PoolError::DatabaseConfig(format!("tokio runtime: {error}")))?;
            let database_pool = runtime.block_on(create_pool_from_config(config))?;
            let sqlite_pool = database_pool.as_sqlite().ok_or_else(|| {
                PoolError::DatabaseConfig("AIoT device database requires SQLite engine".to_owned())
            })?;
            BlockingSqlitePool::from_pool(sqlite_pool.clone()).map_err(PoolError::PoolCreation)
        }
        DatabaseEngine::Postgres => Err(PoolError::DatabaseConfig(
            POSTGRES_DEVICE_PERSISTENCE_DEFERRED.to_owned(),
        )),
    }
}

#[cfg(test)]
mod database_bootstrap_tests {
    use super::*;

    #[test]
    fn resolve_device_database_config_defaults_to_shared_memory_without_env() {
        for key in EXPLICIT_DEVICE_DATABASE_ENV_KEYS {
            std::env::remove_var(key);
        }
        std::env::remove_var("SDKWORK_AIOT_DEVICE_DB_PATH");

        let config = resolve_device_database_config(None);
        assert_eq!(config.engine, DatabaseEngine::Sqlite);
        assert!(config.url.contains("mode=memory"));
    }

    #[test]
    fn resolve_device_database_config_from_env_rejects_postgres_until_phase_k() {
        std::env::remove_var("SDKWORK_AIOT_DEVICE_DB_PATH");
        for key in EXPLICIT_DEVICE_DATABASE_ENV_KEYS {
            std::env::remove_var(key);
        }
        std::env::set_var("SDKWORK_AIOT_DEVICE_DATABASE_ENGINE", "postgres");
        std::env::set_var(
            "SDKWORK_AIOT_DEVICE_DATABASE_URL",
            "postgres://localhost/aiot",
        );

        let error = resolve_device_database_config_from_env(None)
            .expect_err("postgres must fail fast before repository migration");
        assert!(error.to_string().contains("Phase K"));

        std::env::remove_var("SDKWORK_AIOT_DEVICE_DATABASE_ENGINE");
        std::env::remove_var("SDKWORK_AIOT_DEVICE_DATABASE_URL");
    }
}
