use std::net::TcpListener;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    setup_shutdown_signal_handler(Arc::clone(&running));

    let device_db_path = configured_device_db_path("SDKWORK_AIOT_ADMIN_API_DEVICE_DB_PATH");
    let shared_repository = std::sync::Arc::new(build_device_repository(device_db_path.as_deref()));
    let credential_repository = build_credential_repository(device_db_path.as_deref());
    let catalog_repository = build_catalog_repository(device_db_path.as_deref());
    let firmware_repository = build_firmware_repository(device_db_path.as_deref());
    let server = Arc::new(
        sdkwork_aiot_http_api::standard_admin_api_server()
            .expect("admin api server")
            .with_device_repository(shared_repository.clone())
            .with_command_repository(shared_repository.clone())
            .with_event_repository(shared_repository.clone())
            .with_twin_repository(shared_repository)
            .with_credential_repository(credential_repository)
            .with_catalog_repository(catalog_repository)
            .with_firmware_repository(firmware_repository),
    );
    let plan = sdkwork_aiot_runtime::RuntimeServicePlan::standard();

    println!(
        "sdkwork-aiot-admin-api mode={:?} backend_routes={} components={}",
        server.runtime().mode(),
        plan.backend_routes.len(),
        server.runtime().component_names().len()
    );

    if std::env::var("SDKWORK_AIOT_ADMIN_API_NO_LISTEN").as_deref() == Ok("1") {
        return;
    }

    let bind_addr = std::env::var("SDKWORK_AIOT_ADMIN_API_BIND")
        .unwrap_or_else(|_| "127.0.0.1:18081".to_string());
    serve(&server, &bind_addr, running);
}

fn build_device_repository(
    device_db_path: Option<&str>,
) -> sdkwork_aiot_storage_sqlx::SqliteSqlxDeviceRepository {
    if let Some(path) = device_db_path {
        ensure_parent_directory_exists(path);
        println!("sdkwork-aiot-admin-api device-db=sqlite file={path}");
        return sdkwork_aiot_storage_sqlx::SqliteSqlxDeviceRepository::open(path)
            .expect("open sqlite aiot device repository");
    }

    let shared_uri = sdkwork_aiot_storage_sqlx::DEFAULT_SHARED_SQLITE_MEMORY_URI;
    println!("sdkwork-aiot-admin-api device-db=sqlite uri={shared_uri}");
    sdkwork_aiot_storage_sqlx::SqliteSqlxDeviceRepository::open(shared_uri)
        .expect("open shared sqlite aiot device repository")
}

fn build_credential_repository(
    device_db_path: Option<&str>,
) -> std::sync::Arc<dyn sdkwork_aiot_http_api::AiotCredentialRepository> {
    let open_path =
        device_db_path.unwrap_or(sdkwork_aiot_storage_sqlx::DEFAULT_SHARED_SQLITE_MEMORY_URI);
    if device_db_path.is_some() {
        println!("sdkwork-aiot-admin-api credential-db=sqlite file={open_path}");
    } else {
        println!("sdkwork-aiot-admin-api credential-db=sqlite uri={open_path}");
    }
    std::sync::Arc::new(
        sdkwork_aiot_http_api::SqliteCredentialRepositoryAdapter::open(open_path)
            .expect("open sqlite credential repository"),
    )
}

fn build_catalog_repository(
    device_db_path: Option<&str>,
) -> std::sync::Arc<sdkwork_aiot_http_api::AiotCatalogRepositoryHandle> {
    let open_path =
        device_db_path.unwrap_or(sdkwork_aiot_storage_sqlx::DEFAULT_SHARED_SQLITE_MEMORY_URI);
    if device_db_path.is_some() {
        println!("sdkwork-aiot-admin-api catalog-db=sqlite file={open_path}");
    } else {
        println!("sdkwork-aiot-admin-api catalog-db=sqlite uri={open_path}");
    }
    std::sync::Arc::new(
        sdkwork_aiot_http_api::AiotCatalogRepositoryHandle::open_sqlite(open_path)
            .expect("open sqlite catalog repository"),
    )
}

fn build_firmware_repository(
    device_db_path: Option<&str>,
) -> std::sync::Arc<sdkwork_aiot_http_api::AiotFirmwareRepositoryHandle> {
    let open_path =
        device_db_path.unwrap_or(sdkwork_aiot_storage_sqlx::DEFAULT_SHARED_SQLITE_MEMORY_URI);
    if device_db_path.is_some() {
        println!("sdkwork-aiot-admin-api firmware-db=sqlite file={open_path}");
    } else {
        println!("sdkwork-aiot-admin-api firmware-db=sqlite uri={open_path}");
    }
    std::sync::Arc::new(
        sdkwork_aiot_http_api::AiotFirmwareRepositoryHandle::open_sqlite(open_path)
            .expect("open sqlite firmware repository"),
    )
}

fn configured_device_db_path(service_env_key: &str) -> Option<String> {
    std::env::var(service_env_key)
        .ok()
        .or_else(|| std::env::var("SDKWORK_AIOT_DEVICE_DB_PATH").ok())
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn ensure_parent_directory_exists(path: &str) {
    let parent = Path::new(path).parent();
    if let Some(parent) = parent {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).expect("create sqlite parent directory");
        }
    }
}

fn setup_shutdown_signal_handler(running: Arc<AtomicBool>) {
    if let Err(error) = ctrlc::set_handler(move || {
        running.store(false, Ordering::SeqCst);
    }) {
        eprintln!("sdkwork-aiot-admin-api ctrlc_handler_error={error}");
    }
}

fn serve(
    server: &Arc<sdkwork_aiot_http_api::AiotApiServer>,
    bind_addr: &str,
    running: Arc<AtomicBool>,
) {
    let listener = TcpListener::bind(bind_addr).expect("bind admin api listener");
    println!("sdkwork-aiot-admin-api listening on http://{bind_addr}");

    let handler = {
        let server = Arc::clone(server);
        Arc::new(move |bytes: Vec<u8>| {
            match sdkwork_aiot_http_api::handle_api_request_bytes(server.as_ref(), &bytes) {
                Ok(response) => response,
                Err(error) => sdkwork_aiot_http_api::format_api_error_response(&error.code),
            }
        })
    };

    sdkwork_aiot_transport::serve_http_concurrent(
        listener,
        handler,
        sdkwork_aiot_transport::HttpServeOptions {
            read_timeout: Some(Duration::from_secs(5)),
            shutdown: Some(running),
            ..sdkwork_aiot_transport::HttpServeOptions::default()
        },
    );
}
