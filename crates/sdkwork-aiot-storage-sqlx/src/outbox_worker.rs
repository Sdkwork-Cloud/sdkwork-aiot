use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use sdkwork_aiot_service_host::{
    outbox_publisher_from_env, AiotOutboxDispatcher, AiotOutboxDispatcherConfig,
};
use sdkwork_aiot_storage::OutboxEventRepository;

use crate::outbox::SqliteOutboxEventRepository;
use crate::sqlite_sync::{sqlite_connect_url, BlockingSqlitePool};

pub const ENV_DEVICE_DB_PATH: &str = "SDKWORK_AIOT_DEVICE_DB_PATH";
pub const ENV_OUTBOX_DISPATCH_INTERVAL_MS: &str = "SDKWORK_AIOT_OUTBOX_DISPATCH_INTERVAL_MS";
pub const ENV_OUTBOX_LAG_READY_THRESHOLD: &str = "SDKWORK_AIOT_OUTBOX_LAG_READY_THRESHOLD";
pub const ENV_OUTBOX_DISPATCHER_ENABLED: &str = "SDKWORK_AIOT_OUTBOX_DISPATCHER_ENABLED";

pub const DEFAULT_OUTBOX_DISPATCH_INTERVAL_MS: u64 = 1_000;
pub const DEFAULT_OUTBOX_LAG_READY_THRESHOLD: u64 = 10_000;

pub fn outbox_lag_ready_threshold_from_env() -> u64 {
    std::env::var(ENV_OUTBOX_LAG_READY_THRESHOLD)
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(DEFAULT_OUTBOX_LAG_READY_THRESHOLD)
}

pub fn outbox_dispatcher_enabled_from_env(default_when_unset: bool) -> bool {
    match std::env::var(ENV_OUTBOX_DISPATCHER_ENABLED)
        .ok()
        .map(|value| value.trim().to_ascii_lowercase())
        .as_deref()
    {
        Some("0") | Some("false") => false,
        Some("1") | Some("true") => true,
        _ => default_when_unset && configured_device_db_path_from_env().is_some(),
    }
}

pub fn configured_device_db_path_from_env() -> Option<String> {
    std::env::var(ENV_DEVICE_DB_PATH)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

pub fn device_storage_ready_from_env() -> bool {
    let Some(path) = configured_device_db_path_from_env() else {
        return true;
    };
    sqlite_path_ready(&path)
}

pub fn sqlite_path_ready(path: &str) -> bool {
    let url = sqlite_connect_url(path);
    BlockingSqlitePool::connect(&url)
        .and_then(|db| {
            db.run(async {
                sqlx::query_scalar::<_, i64>("SELECT 1")
                    .fetch_one(db.pool())
                    .await
            })
        })
        .is_ok()
}

pub fn outbox_lag_count_from_env() -> Option<u64> {
    open_outbox_repository_from_env().map(|repository| repository.pending_lag_count())
}

pub fn outbox_ready_from_env() -> bool {
    match outbox_lag_count_from_env() {
        None => true,
        Some(lag) => lag <= outbox_lag_ready_threshold_from_env(),
    }
}

pub fn outbox_readiness_probe(
    outbox_lag: Arc<AtomicU64>,
) -> impl Fn() -> bool + Send + Sync + 'static {
    let threshold = outbox_lag_ready_threshold_from_env();
    move || device_storage_ready_from_env() && outbox_lag.load(Ordering::Relaxed) <= threshold
}

pub fn open_outbox_repository_from_env() -> Option<Arc<SqliteOutboxEventRepository>> {
    let path = configured_device_db_path_from_env()?;
    open_outbox_repository_for_path(&path)
}

pub fn open_outbox_repository_for_path(
    path: impl AsRef<Path>,
) -> Option<Arc<SqliteOutboxEventRepository>> {
    match SqliteOutboxEventRepository::open(path) {
        Ok(repository) => Some(Arc::new(repository)),
        Err(error) => {
            eprintln!("sdkwork-aiot-storage-sqlx outbox_repository_open_error={error}");
            None
        }
    }
}

pub fn start_outbox_dispatcher_worker(
    running: Arc<AtomicBool>,
    outbox_lag: Option<Arc<AtomicU64>>,
    default_enabled: bool,
) {
    if !outbox_dispatcher_enabled_from_env(default_enabled) {
        return;
    }
    let Some(repository) = open_outbox_repository_from_env() else {
        return;
    };
    let dispatcher = AiotOutboxDispatcher::new(
        repository,
        outbox_publisher_from_env(),
        AiotOutboxDispatcherConfig::default(),
    );
    let interval_ms = std::env::var(ENV_OUTBOX_DISPATCH_INTERVAL_MS)
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(DEFAULT_OUTBOX_DISPATCH_INTERVAL_MS);
    std::thread::spawn(move || {
        while running.load(Ordering::Relaxed) {
            if let Some(outbox_lag) = outbox_lag.as_ref() {
                outbox_lag.store(dispatcher.pending_lag_count(), Ordering::Relaxed);
            }
            let _published = dispatcher.run_once();
            if let Some(outbox_lag) = outbox_lag.as_ref() {
                outbox_lag.store(dispatcher.pending_lag_count(), Ordering::Relaxed);
            }
            std::thread::sleep(Duration::from_millis(interval_ms));
        }
    });
}

#[cfg(test)]
mod outbox_worker_tests {
    use super::*;

    #[test]
    fn outbox_dispatcher_defaults_to_gateway_only_when_db_path_is_set() {
        std::env::set_var(ENV_DEVICE_DB_PATH, "/tmp/aiot-device.db");
        std::env::remove_var(ENV_OUTBOX_DISPATCHER_ENABLED);

        assert!(outbox_dispatcher_enabled_from_env(true));
        assert!(!outbox_dispatcher_enabled_from_env(false));

        std::env::remove_var(ENV_DEVICE_DB_PATH);
    }

    #[test]
    fn outbox_ready_from_env_honors_lag_threshold() {
        use crate::SqlxPoolSqlStatementExecutor;
        use sdkwork_aiot_storage::{
            AiotOutboxWriteIntent, AiotProtocolIngestUnitOfWork, AiotProtocolStorageCommand,
            AiotStorageWriteKind,
        };

        let unique_suffix = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let db_path = std::env::temp_dir().join(format!("aiot-outbox-ready-{unique_suffix}.db"));
        let _ = std::fs::remove_file(&db_path);

        std::env::set_var(ENV_DEVICE_DB_PATH, db_path.to_string_lossy().to_string());
        std::env::set_var(ENV_OUTBOX_LAG_READY_THRESHOLD, "0");

        assert!(outbox_ready_from_env());

        let executor = SqlxPoolSqlStatementExecutor::open(&db_path).expect("executor");
        let uow = executor.protocol_ingest_unit_of_work();
        let command = AiotProtocolStorageCommand::new(
            "xiaozhi.websocket",
            "xiaozhi",
            "device-outbox-ready-001",
            AiotStorageWriteKind::OpenSession,
            "iot_device_session",
        )
        .with_session_id("session-outbox-ready-001")
        .with_idempotency_key("outbox-ready-idem-001")
        .with_outbox(AiotOutboxWriteIntent::new(
            "iot.device.session.started",
            "device_session",
            "session-outbox-ready-001",
            "iot.protocol.ingested",
        ));
        assert!(uow.execute_protocol_command(&command).accepted);
        assert_eq!(outbox_lag_count_from_env(), Some(1));
        assert!(!outbox_ready_from_env());

        std::env::remove_var(ENV_DEVICE_DB_PATH);
        std::env::remove_var(ENV_OUTBOX_LAG_READY_THRESHOLD);
        let _ = std::fs::remove_file(&db_path);
    }
}
