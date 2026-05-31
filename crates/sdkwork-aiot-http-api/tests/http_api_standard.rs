use sdkwork_aiot_http_api::{
    handle_api_request_bytes, handle_resolved_api_request, resolve_api_request,
    route_contract_for_request, standard_admin_api_server, standard_api_route_contracts,
    standard_app_api_server, AiotApiRequestContext, AiotApiSurface, AiotResolvedApiRequest,
};
use sdkwork_aiot_transport::{HttpRequest, HttpStatus};

#[test]
fn admin_api_server_exposes_runtime_backed_protocol_catalog() {
    let server = standard_admin_api_server().expect("admin api server");

    assert_eq!(server.surface(), AiotApiSurface::Admin);
    assert!(server.runtime().supports_protocol("xiaozhi.websocket"));

    let response = handle_api_request_bytes(
        &server,
        b"GET /backend/v3/api/iot/protocol_adapters HTTP/1.1\r\nHost: local\r\nAuthorization: Bearer app-token\r\nAccess-Token: user-token\r\nX-Sdkwork-Tenant-Id: 10001\r\nX-Sdkwork-Organization-Id: 20001\r\nX-Sdkwork-Permission-Scope: iot.protocolAdapters.read\r\n\r\n",
    )
    .expect("protocol adapter catalog");

    assert!(response.starts_with("HTTP/1.1 200"));
    assert!(response.contains(r#""code":"0""#));
    assert!(response.contains(r#""data":["#));
    assert!(response.contains(r#""protocolId":"xiaozhi.websocket""#));
    assert!(response.contains(r#""pluginId":"xiaozhi""#));
    assert!(response.contains(r#""scope":"CompatibilityPlugin""#));
    assert!(response.contains(r#""codecs":["JsonText","JsonRpc","BinaryMedia"]"#));
    assert!(response.contains(r#""sessionPolicies":["StatefulDeviceSession"]"#));
    assert!(response.contains(r#""securityModes":["bearer_token","hmac"]"#));
    assert!(response.contains(r#""hardwareFamilies":["esp32","esp32_s3"]"#));
    assert!(response.contains(r#""transports":["WebSocket","Mqtt","Udp"]"#));
}

#[test]
fn app_api_server_exposes_safe_device_collection_boundary() {
    let server = standard_app_api_server().expect("app api server");

    assert_eq!(server.surface(), AiotApiSurface::App);

    let response = handle_api_request_bytes(
        &server,
        b"GET /app/v3/api/iot/devices HTTP/1.1\r\nHost: local\r\nAuthorization: Bearer app-token\r\nAccess-Token: user-token\r\nX-Sdkwork-Tenant-Id: 10001\r\nX-Sdkwork-Organization-Id: 20001\r\nX-Sdkwork-Permission-Scope: iot.devices.read\r\n\r\n",
    )
    .expect("device list");

    assert!(response.starts_with("HTTP/1.1 200"));
    assert!(response.contains(r#""code":"0""#));
    assert!(response.contains(r#""data":[]"#));
}

#[test]
fn admin_api_server_exposes_runtime_capacity_policy_from_standard_bundle() {
    let server = standard_admin_api_server().expect("admin api server");

    let response = handle_api_request_bytes(
        &server,
        b"GET /backend/v3/api/iot/runtime/capacity HTTP/1.1\r\nHost: local\r\nAuthorization: Bearer app-token\r\nAccess-Token: user-token\r\nX-Sdkwork-Tenant-Id: 10001\r\nX-Sdkwork-Organization-Id: 20001\r\nX-Sdkwork-Permission-Scope: iot.runtime.read\r\n\r\n",
    )
    .expect("runtime capacity");

    assert!(response.starts_with("HTTP/1.1 200"));
    assert!(response.contains(r#""code":"0""#));
    assert!(response.contains(r#""nodeId":"local""#));
    assert!(response.contains(r#""maxConnectionsPerNode":"100000""#));
    assert!(response.contains(r#""maxSessionsPerTenant":"1000000""#));
    assert!(response.contains(r#""maxInflightPerDevice":64"#));
    assert!(response.contains(r#""sessionLeaseTtlSeconds":90"#));
    assert!(response.contains(
        r#""backpressure":{"warnLag":"100000","rejectLag":"500000","deadLetterLag":"1000000"}"#
    ));
    assert!(response.contains(r#""orderedDeviceCommands":true"#));
    assert!(response.contains(r#""idempotentIngest":true"#));
}

#[test]
fn app_and_admin_api_servers_share_health_and_ready_contracts() {
    for server in [
        standard_admin_api_server().unwrap(),
        standard_app_api_server().unwrap(),
    ] {
        let health =
            handle_api_request_bytes(&server, b"GET /healthz HTTP/1.1\r\nHost: local\r\n\r\n")
                .expect("health");
        assert!(health.starts_with("HTTP/1.1 200"));
        assert!(health.contains(r#""ready":true"#));

        let ready =
            handle_api_request_bytes(&server, b"GET /readyz HTTP/1.1\r\nHost: local\r\n\r\n")
                .expect("ready");
        assert!(ready.starts_with("HTTP/1.1 200"));
        assert!(ready.contains(r#""ready":true"#));
    }
}

#[test]
fn protected_api_routes_require_sdkwork_dual_token_and_resolved_appbase_context() {
    let admin = standard_admin_api_server().expect("admin api server");

    let missing_tokens = handle_api_request_bytes(
        &admin,
        b"GET /backend/v3/api/iot/protocol_adapters HTTP/1.1\r\nHost: local\r\n\r\n",
    )
    .expect("missing tokens problem");
    assert!(missing_tokens.starts_with("HTTP/1.1 401"));
    assert!(missing_tokens.contains("application/problem+json"));
    assert!(missing_tokens.contains("api.auth.missing_dual_token"));

    let missing_context = handle_api_request_bytes(
        &admin,
        b"GET /backend/v3/api/iot/protocol_adapters HTTP/1.1\r\nHost: local\r\nAuthorization: Bearer app-token\r\nAccess-Token: user-token\r\n\r\n",
    )
    .expect("missing context problem");
    assert!(missing_context.starts_with("HTTP/1.1 403"));
    assert!(missing_context.contains("api.context.missing"));

    let invalid_context = handle_api_request_bytes(
        &admin,
        b"GET /backend/v3/api/iot/protocol_adapters HTTP/1.1\r\nHost: local\r\nAuthorization: Bearer app-token\r\nAccess-Token: user-token\r\nX-Sdkwork-Tenant-Id: tenant-a\r\nX-Sdkwork-Organization-Id: 20001\r\n\r\n",
    )
    .expect("invalid context problem");
    assert!(invalid_context.starts_with("HTTP/1.1 400"));
    assert!(invalid_context.contains("api.context.invalid_tenant_id"));
}

#[test]
fn protected_api_request_resolution_exposes_appbase_context_to_downstream_handlers() {
    let admin = standard_admin_api_server().expect("admin api server");
    let request = HttpRequest::new("GET", "/backend/v3/api/iot/runtime/capacity")
        .with_header("Authorization", "Bearer app-token")
        .with_header("Access-Token", "user-token")
        .with_header("X-Sdkwork-Tenant-Id", "10001")
        .with_header("X-Sdkwork-Organization-Id", "20001")
        .with_header("X-Sdkwork-User-Id", "30001")
        .with_header("X-Sdkwork-Data-Scope", "7")
        .with_header("X-Sdkwork-Permission-Scope", "iot.runtime.read");

    let resolved = resolve_api_request(&request).expect("resolved api request");

    assert_eq!(
        resolved.request().path,
        "/backend/v3/api/iot/runtime/capacity"
    );
    match resolved.context() {
        AiotApiRequestContext::Protected(ctx) => {
            assert_eq!(ctx.tenant_id, "10001");
            assert_eq!(ctx.organization_id, "20001");
            assert_eq!(ctx.user_id.as_deref(), Some("30001"));
            assert_eq!(ctx.data_scope, vec!["7".to_string()]);
        }
        AiotApiRequestContext::Public => panic!("protected API route must carry context"),
    }

    let response = handle_resolved_api_request(&admin, &resolved);
    assert_eq!(response.status, HttpStatus::Ok);
    assert!(response.body.contains(r#""code":"0""#));
}

#[test]
fn protected_api_handler_rejects_unresolved_public_context_before_dispatch() {
    let admin = standard_admin_api_server().expect("admin api server");
    let request = HttpRequest::new("GET", "/backend/v3/api/iot/runtime/capacity");
    let unresolved = AiotResolvedApiRequest::public(&request);

    let response = handle_resolved_api_request(&admin, &unresolved);

    assert_eq!(response.status, HttpStatus::Forbidden);
    assert!(response.body.contains("api.context.missing"));
}

#[test]
fn standard_api_route_contracts_declare_surface_operation_and_permission_boundaries() {
    let contracts = standard_api_route_contracts();

    let app_devices = contracts
        .iter()
        .find(|route| route.path == "/app/v3/api/iot/devices")
        .expect("app devices route contract");
    assert_eq!(app_devices.surface, AiotApiSurface::App);
    assert_eq!(app_devices.method, "GET");
    assert_eq!(app_devices.operation_id, "devices.list");
    assert_eq!(app_devices.required_permission, "iot.devices.read");

    let backend_protocols = contracts
        .iter()
        .find(|route| route.path == "/backend/v3/api/iot/protocol_adapters")
        .expect("backend protocol adapter route contract");
    assert_eq!(backend_protocols.surface, AiotApiSurface::Admin);
    assert_eq!(backend_protocols.operation_id, "protocolAdapters.list");
    assert_eq!(
        backend_protocols.required_permission,
        "iot.protocolAdapters.read"
    );

    assert!(contracts
        .iter()
        .all(|route| route.operation_id.contains('.')));
    assert!(contracts.iter().any(|route| {
        route.operation_id == "runtime.capacity.retrieve"
            && route.required_permission == "iot.runtime.read"
    }));
}

#[test]
fn protected_api_routes_require_resolved_permission_scope_from_appbase_context() {
    let admin = standard_admin_api_server().expect("admin api server");

    let missing_permission = handle_api_request_bytes(
        &admin,
        b"GET /backend/v3/api/iot/protocol_adapters HTTP/1.1\r\nHost: local\r\nAuthorization: Bearer app-token\r\nAccess-Token: user-token\r\nX-Sdkwork-Tenant-Id: 10001\r\nX-Sdkwork-Organization-Id: 20001\r\nX-Sdkwork-Permission-Scope: iot.devices.read\r\n\r\n",
    )
    .expect("missing permission problem");
    assert!(missing_permission.starts_with("HTTP/1.1 403"));
    assert!(missing_permission.contains("api.permission.denied"));
    assert!(missing_permission.contains("iot.protocolAdapters.read"));

    let allowed = handle_api_request_bytes(
        &admin,
        b"GET /backend/v3/api/iot/protocol_adapters HTTP/1.1\r\nHost: local\r\nAuthorization: Bearer app-token\r\nAccess-Token: user-token\r\nX-Sdkwork-Tenant-Id: 10001\r\nX-Sdkwork-Organization-Id: 20001\r\nX-Sdkwork-Permission-Scope: iot.protocolAdapters.read\r\n\r\n",
    )
    .expect("allowed protocol adapters");
    assert!(allowed.starts_with("HTTP/1.1 200"));
}

#[test]
fn templated_api_route_contracts_match_concrete_paths_before_dispatch() {
    let app = standard_app_api_server().expect("app api server");
    let request = HttpRequest::new("POST", "/app/v3/api/iot/devices/device-001/commands");
    let contract = route_contract_for_request(AiotApiSurface::App, &request)
        .expect("templated command route contract");

    assert_eq!(contract.path, "/app/v3/api/iot/devices/{deviceId}/commands");
    assert_eq!(contract.operation_id, "devices.commands.create");
    assert_eq!(contract.required_permission, "iot.commands.execute");

    let missing_permission = handle_api_request_bytes(
        &app,
        b"POST /app/v3/api/iot/devices/device-001/commands HTTP/1.1\r\nHost: local\r\nAuthorization: Bearer app-token\r\nAccess-Token: user-token\r\nX-Sdkwork-Tenant-Id: 10001\r\nX-Sdkwork-Organization-Id: 20001\r\nX-Sdkwork-Permission-Scope: iot.devices.read\r\n\r\n",
    )
    .expect("templated route permission problem");

    assert!(missing_permission.starts_with("HTTP/1.1 403"));
    assert!(missing_permission.contains("api.permission.denied"));
    assert!(missing_permission.contains("iot.commands.execute"));
}

#[test]
fn declared_backend_collection_routes_return_standard_empty_collections() {
    let admin = standard_admin_api_server().expect("admin api server");

    for (path, permission) in [
        ("/backend/v3/api/iot/products", "iot.products.read"),
        ("/backend/v3/api/iot/hardware_profiles", "iot.profiles.read"),
        ("/backend/v3/api/iot/protocol_profiles", "iot.profiles.read"),
        ("/backend/v3/api/iot/devices", "iot.devices.read"),
    ] {
        let request = format!(
            "GET {path} HTTP/1.1\r\nHost: local\r\nAuthorization: Bearer app-token\r\nAccess-Token: user-token\r\nX-Sdkwork-Tenant-Id: 10001\r\nX-Sdkwork-Organization-Id: 20001\r\nX-Sdkwork-Permission-Scope: {permission}\r\n\r\n"
        );

        let response = handle_api_request_bytes(&admin, request.as_bytes())
            .unwrap_or_else(|error| panic!("{path} failed with {}", error.code));

        assert!(
            response.starts_with("HTTP/1.1 200"),
            "{path} should be mounted, got {response}"
        );
        assert!(response.contains(r#""code":"0""#), "{path} missing code");
        assert!(
            response.contains(r#""data":[]"#),
            "{path} missing empty data"
        );
        assert!(
            !response.contains("api.route.unsupported"),
            "{path} must not fall through to unsupported route"
        );
    }
}

#[test]
fn api_server_rejects_cross_surface_routes_with_problem_json() {
    let app = standard_app_api_server().expect("app api server");

    let response = handle_api_request_bytes(
        &app,
        b"GET /backend/v3/api/iot/protocol_adapters HTTP/1.1\r\nHost: local\r\nAuthorization: Bearer app-token\r\nAccess-Token: user-token\r\nX-Sdkwork-Tenant-Id: 10001\r\nX-Sdkwork-Organization-Id: 20001\r\nX-Sdkwork-Permission-Scope: iot.protocolAdapters.read\r\n\r\n",
    )
    .expect("problem json");

    assert!(response.starts_with("HTTP/1.1 404"));
    assert!(response.contains("application/problem+json"));
    assert!(response.contains("api.route.unsupported"));
}
