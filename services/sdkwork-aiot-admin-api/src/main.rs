use std::path::Path;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let device_db_path = configured_device_db_path("SDKWORK_AIOT_ADMIN_API_DEVICE_DB_PATH");
    let shared_repository = Arc::new(build_device_repository(device_db_path.as_deref()));
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

    let bind_addr = std::env::var("SDKWORK_AIOT_APPLICATION_ADMIN_HTTP_BIND")
        .unwrap_or_else(|_| "127.0.0.1:18081".to_string());
    let router = sdkwork_router_iot_backend_api::build_wrapped_backend_api_router(server).await;
    if let Err(error) =
        sdkwork_router_iot_backend_api::serve_backend_api_router(&bind_addr, router).await
    {
        eprintln!("sdkwork-aiot-admin-api serve_error={error}");
        std::process::exit(1);
    }
}

fn build_device_repository(
    device_db_path: Option<&str>,
) -> sdkwork_aiot_storage_sqlx::SqliteSqlxDeviceRepository {
    if let Some(path) = device_db_path {
        ensure_parent_directory_exists(path);
        println!("sdkwork-aiot-admin-api device-db=sqlite file={path}");
    } else {
        println!(
            "sdkwork-aiot-admin-api device-db=sqlite uri={}",
            sdkwork_aiot_storage_sqlx::DEFAULT_SHARED_SQLITE_MEMORY_URI
        );
    }
    sdkwork_aiot_storage_sqlx::open_device_repository(device_db_path)
        .expect("open sqlite aiot device repository")
}

fn build_credential_repository(
    device_db_path: Option<&str>,
) -> Arc<dyn sdkwork_aiot_http_api::AiotCredentialRepository> {
    let open_path =
        device_db_path.unwrap_or(sdkwork_aiot_storage_sqlx::DEFAULT_SHARED_SQLITE_MEMORY_URI);
    if device_db_path.is_some() {
        println!("sdkwork-aiot-admin-api credential-db=sqlite file={open_path}");
    } else {
        println!("sdkwork-aiot-admin-api credential-db=sqlite uri={open_path}");
    }
    Arc::new(
        sdkwork_aiot_http_api::SqliteCredentialRepositoryAdapter::open(open_path)
            .expect("open sqlite credential repository"),
    )
}

fn build_catalog_repository(
    device_db_path: Option<&str>,
) -> Arc<sdkwork_aiot_http_api::AiotCatalogRepositoryHandle> {
    let open_path =
        device_db_path.unwrap_or(sdkwork_aiot_storage_sqlx::DEFAULT_SHARED_SQLITE_MEMORY_URI);
    if device_db_path.is_some() {
        println!("sdkwork-aiot-admin-api catalog-db=sqlite file={open_path}");
    } else {
        println!("sdkwork-aiot-admin-api catalog-db=sqlite uri={open_path}");
    }
    Arc::new(
        sdkwork_aiot_http_api::AiotCatalogRepositoryHandle::open_sqlite(open_path)
            .expect("open sqlite catalog repository"),
    )
}

fn build_firmware_repository(
    device_db_path: Option<&str>,
) -> Arc<sdkwork_aiot_http_api::AiotFirmwareRepositoryHandle> {
    let open_path =
        device_db_path.unwrap_or(sdkwork_aiot_storage_sqlx::DEFAULT_SHARED_SQLITE_MEMORY_URI);
    if device_db_path.is_some() {
        println!("sdkwork-aiot-admin-api firmware-db=sqlite file={open_path}");
    } else {
        println!("sdkwork-aiot-admin-api firmware-db=sqlite uri={open_path}");
    }
    Arc::new(
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
