use sdkwork_aiot_contract::{
    AiotRequestContext, IOT_PERMISSION_COMMANDS_EXECUTE, IOT_PERMISSION_DEVICES_READ,
    IOT_PERMISSION_PRODUCTS_READ, IOT_PERMISSION_PROFILES_READ,
    IOT_PERMISSION_PROTOCOL_ADAPTERS_READ, IOT_PERMISSION_RUNTIME_READ, IOT_PERMISSION_TWINS_READ,
};
use sdkwork_aiot_runtime::{standard_aiot_runtime, AiotRuntime, RuntimeBuildError, RuntimeMode};
use sdkwork_aiot_transport::{build_health_response, HttpRequest, HttpResponse, HttpStatus};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AiotApiSurface {
    Admin,
    App,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AiotApiRouteContract {
    pub surface: AiotApiSurface,
    pub method: &'static str,
    pub path: &'static str,
    pub operation_id: &'static str,
    pub required_permission: &'static str,
}

pub fn standard_api_route_contracts() -> Vec<AiotApiRouteContract> {
    vec![
        AiotApiRouteContract {
            surface: AiotApiSurface::App,
            method: "GET",
            path: "/app/v3/api/iot/devices",
            operation_id: "devices.list",
            required_permission: IOT_PERMISSION_DEVICES_READ,
        },
        AiotApiRouteContract {
            surface: AiotApiSurface::App,
            method: "GET",
            path: "/app/v3/api/iot/devices/{deviceId}",
            operation_id: "devices.retrieve",
            required_permission: IOT_PERMISSION_DEVICES_READ,
        },
        AiotApiRouteContract {
            surface: AiotApiSurface::App,
            method: "POST",
            path: "/app/v3/api/iot/devices/{deviceId}/commands",
            operation_id: "devices.commands.create",
            required_permission: IOT_PERMISSION_COMMANDS_EXECUTE,
        },
        AiotApiRouteContract {
            surface: AiotApiSurface::App,
            method: "GET",
            path: "/app/v3/api/iot/devices/{deviceId}/twin",
            operation_id: "devices.twin.retrieve",
            required_permission: IOT_PERMISSION_TWINS_READ,
        },
        AiotApiRouteContract {
            surface: AiotApiSurface::App,
            method: "GET",
            path: "/app/v3/api/iot/devices/{deviceId}/events",
            operation_id: "devices.events.list",
            required_permission: IOT_PERMISSION_DEVICES_READ,
        },
        AiotApiRouteContract {
            surface: AiotApiSurface::Admin,
            method: "GET",
            path: "/backend/v3/api/iot/products",
            operation_id: "products.list",
            required_permission: IOT_PERMISSION_PRODUCTS_READ,
        },
        AiotApiRouteContract {
            surface: AiotApiSurface::Admin,
            method: "GET",
            path: "/backend/v3/api/iot/hardware_profiles",
            operation_id: "hardwareProfiles.list",
            required_permission: IOT_PERMISSION_PROFILES_READ,
        },
        AiotApiRouteContract {
            surface: AiotApiSurface::Admin,
            method: "GET",
            path: "/backend/v3/api/iot/protocol_profiles",
            operation_id: "protocolProfiles.list",
            required_permission: IOT_PERMISSION_PROFILES_READ,
        },
        AiotApiRouteContract {
            surface: AiotApiSurface::Admin,
            method: "GET",
            path: "/backend/v3/api/iot/capability_models/{capabilityModelId}",
            operation_id: "capabilityModels.retrieve",
            required_permission: IOT_PERMISSION_PROFILES_READ,
        },
        AiotApiRouteContract {
            surface: AiotApiSurface::Admin,
            method: "GET",
            path: "/backend/v3/api/iot/devices",
            operation_id: "devices.list",
            required_permission: IOT_PERMISSION_DEVICES_READ,
        },
        AiotApiRouteContract {
            surface: AiotApiSurface::Admin,
            method: "POST",
            path: "/backend/v3/api/iot/devices/{deviceId}/credentials",
            operation_id: "devices.credentials.create",
            required_permission: sdkwork_aiot_contract::IOT_PERMISSION_DEVICES_WRITE,
        },
        AiotApiRouteContract {
            surface: AiotApiSurface::Admin,
            method: "POST",
            path: "/backend/v3/api/iot/firmware_artifacts",
            operation_id: "firmwareArtifacts.create",
            required_permission: sdkwork_aiot_contract::IOT_PERMISSION_FIRMWARE_WRITE,
        },
        AiotApiRouteContract {
            surface: AiotApiSurface::Admin,
            method: "POST",
            path: "/backend/v3/api/iot/firmware_rollouts",
            operation_id: "firmwareRollouts.create",
            required_permission: sdkwork_aiot_contract::IOT_PERMISSION_FIRMWARE_ROLLOUT,
        },
        AiotApiRouteContract {
            surface: AiotApiSurface::Admin,
            method: "GET",
            path: "/backend/v3/api/iot/protocol_adapters",
            operation_id: "protocolAdapters.list",
            required_permission: IOT_PERMISSION_PROTOCOL_ADAPTERS_READ,
        },
        AiotApiRouteContract {
            surface: AiotApiSurface::Admin,
            method: "GET",
            path: "/backend/v3/api/iot/runtime/capacity",
            operation_id: "runtime.capacity.retrieve",
            required_permission: IOT_PERMISSION_RUNTIME_READ,
        },
    ]
}

pub fn route_contract_for_request(
    surface: AiotApiSurface,
    request: &HttpRequest,
) -> Option<AiotApiRouteContract> {
    standard_api_route_contracts().into_iter().find(|route| {
        route.surface == surface
            && route.method.eq_ignore_ascii_case(&request.method)
            && route_path_matches(route.path, &request.path)
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiotApiServer {
    surface: AiotApiSurface,
    runtime: AiotRuntime,
}

impl AiotApiServer {
    pub fn new(surface: AiotApiSurface, runtime: AiotRuntime) -> Self {
        Self { surface, runtime }
    }

    pub fn surface(&self) -> AiotApiSurface {
        self.surface
    }

    pub fn runtime(&self) -> &AiotRuntime {
        &self.runtime
    }
}

pub fn standard_admin_api_server() -> Result<AiotApiServer, RuntimeBuildError> {
    Ok(AiotApiServer::new(
        AiotApiSurface::Admin,
        standard_aiot_runtime(RuntimeMode::Standalone)?,
    ))
}

pub fn standard_app_api_server() -> Result<AiotApiServer, RuntimeBuildError> {
    Ok(AiotApiServer::new(
        AiotApiSurface::App,
        standard_aiot_runtime(RuntimeMode::Standalone)?,
    ))
}

pub fn handle_api_request_bytes(
    server: &AiotApiServer,
    bytes: &[u8],
) -> Result<String, AiotApiError> {
    let request = parse_http_request(bytes)?;
    let response = handle_api_request(server, &request);
    Ok(format_http_response(&response))
}

pub fn handle_api_request(server: &AiotApiServer, request: &HttpRequest) -> HttpResponse {
    let resolved = match resolve_api_request(request) {
        Ok(resolved) => resolved,
        Err(response) => return response,
    };

    handle_resolved_api_request(server, &resolved)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AiotApiRequestContext {
    Public,
    Protected(AiotRequestContext),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiotResolvedApiRequest<'a> {
    request: &'a HttpRequest,
    context: AiotApiRequestContext,
}

impl<'a> AiotResolvedApiRequest<'a> {
    pub fn public(request: &'a HttpRequest) -> Self {
        Self {
            request,
            context: AiotApiRequestContext::Public,
        }
    }

    pub fn protected(request: &'a HttpRequest, context: AiotRequestContext) -> Self {
        Self {
            request,
            context: AiotApiRequestContext::Protected(context),
        }
    }

    pub fn request(&self) -> &HttpRequest {
        self.request
    }

    pub fn context(&self) -> &AiotApiRequestContext {
        &self.context
    }
}

pub fn resolve_api_request(
    request: &HttpRequest,
) -> Result<AiotResolvedApiRequest<'_>, HttpResponse> {
    if is_protected_iot_api_path(&request.path) {
        return resolve_protected_request_context(request)
            .map(|ctx| AiotResolvedApiRequest::protected(request, ctx));
    }

    Ok(AiotResolvedApiRequest::public(request))
}

pub fn handle_resolved_api_request(
    server: &AiotApiServer,
    resolved: &AiotResolvedApiRequest<'_>,
) -> HttpResponse {
    let request = resolved.request();
    if is_protected_iot_api_path(&request.path)
        && !matches!(resolved.context(), AiotApiRequestContext::Protected(_))
    {
        return problem_response(
            HttpStatus::Forbidden,
            "api.context.missing",
            "Resolved appbase context is required",
        );
    }
    if let Err(response) = enforce_route_permission(server.surface, resolved) {
        return response;
    }

    match request.path.as_str() {
        "/healthz" | "/readyz" => build_health_response("sdkwork-aiot-http-api", true),
        "/backend/v3/api/iot/protocol_adapters" if server.surface == AiotApiSurface::Admin => {
            HttpResponse::new(HttpStatus::Ok)
                .with_header("content-type", "application/json")
                .with_body(protocol_adapters_json(server.runtime()))
        }
        "/backend/v3/api/iot/runtime/capacity" if server.surface == AiotApiSurface::Admin => {
            HttpResponse::new(HttpStatus::Ok)
                .with_header("content-type", "application/json")
                .with_body(runtime_capacity_json())
        }
        "/backend/v3/api/iot/products"
        | "/backend/v3/api/iot/hardware_profiles"
        | "/backend/v3/api/iot/protocol_profiles"
        | "/backend/v3/api/iot/devices"
            if server.surface == AiotApiSurface::Admin =>
        {
            standard_empty_collection_response()
        }
        "/app/v3/api/iot/devices" if server.surface == AiotApiSurface::App => {
            standard_empty_collection_response()
        }
        _ => problem_response(
            HttpStatus::NotFound,
            "api.route.unsupported",
            "API route is not mounted on this surface",
        ),
    }
}

fn is_protected_iot_api_path(path: &str) -> bool {
    path.starts_with("/backend/v3/api/iot") || path.starts_with("/app/v3/api/iot")
}

fn resolve_protected_request_context(
    request: &HttpRequest,
) -> Result<AiotRequestContext, HttpResponse> {
    if is_blank_header(request, "authorization") || is_blank_header(request, "access-token") {
        return Err(problem_response(
            HttpStatus::Unauthorized,
            "api.auth.missing_dual_token",
            "SDKWork dual token is required",
        ));
    }

    let tenant_id = required_header(request, "x-sdkwork-tenant-id").map_err(|_| {
        problem_response(
            HttpStatus::Forbidden,
            "api.context.missing",
            "Resolved appbase context is required",
        )
    })?;
    let organization_id = required_header(request, "x-sdkwork-organization-id").map_err(|_| {
        problem_response(
            HttpStatus::Forbidden,
            "api.context.missing",
            "Resolved appbase context is required",
        )
    })?;

    parse_i64(tenant_id).map_err(|_| {
        problem_response(
            HttpStatus::BadRequest,
            "api.context.invalid_tenant_id",
            "Resolved tenant id is invalid",
        )
    })?;
    parse_i64(organization_id).map_err(|_| {
        problem_response(
            HttpStatus::BadRequest,
            "api.context.invalid_organization_id",
            "Resolved organization id is invalid",
        )
    })?;

    let mut ctx = AiotRequestContext::new(tenant_id, organization_id);

    if let Some(user_id) = optional_header(request, "x-sdkwork-user-id") {
        parse_i64(user_id).map_err(|_| {
            problem_response(
                HttpStatus::BadRequest,
                "api.context.invalid_user_id",
                "Resolved user id is invalid",
            )
        })?;
        ctx = ctx.with_user(user_id);
    }

    if let Some(data_scope) = optional_header(request, "x-sdkwork-data-scope") {
        data_scope.parse::<i32>().map_err(|_| {
            problem_response(
                HttpStatus::BadRequest,
                "api.context.invalid_data_scope",
                "Resolved data scope is invalid",
            )
        })?;
        ctx = ctx.with_data_scope(data_scope);
    }
    for permission in permission_scope_headers(request) {
        ctx = ctx.with_permission(permission);
    }

    Ok(ctx)
}

fn enforce_route_permission(
    surface: AiotApiSurface,
    resolved: &AiotResolvedApiRequest<'_>,
) -> Result<(), HttpResponse> {
    let request = resolved.request();
    let Some(route) = route_contract_for_request(surface, request) else {
        return Ok(());
    };

    let AiotApiRequestContext::Protected(ctx) = resolved.context() else {
        return Err(problem_response(
            HttpStatus::Forbidden,
            "api.context.missing",
            "Resolved appbase context is required",
        ));
    };

    if ctx.has_permission(route.required_permission) {
        Ok(())
    } else {
        Err(permission_denied_response(route.required_permission))
    }
}

fn route_path_matches(template: &str, path: &str) -> bool {
    let template_segments = template.trim_matches('/').split('/').collect::<Vec<_>>();
    let path_segments = path.trim_matches('/').split('/').collect::<Vec<_>>();

    if template_segments.len() != path_segments.len() {
        return false;
    }

    template_segments
        .iter()
        .zip(path_segments.iter())
        .all(|(template, actual)| {
            (template.starts_with('{') && template.ends_with('}') && !actual.is_empty())
                || template == actual
        })
}

fn permission_scope_headers(request: &HttpRequest) -> Vec<&str> {
    optional_header(request, "x-sdkwork-permission-scope")
        .into_iter()
        .flat_map(|value| value.split(','))
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .collect()
}

fn is_blank_header(request: &HttpRequest, name: &str) -> bool {
    optional_header(request, name).is_none()
}

fn required_header<'a>(request: &'a HttpRequest, name: &str) -> Result<&'a str, ()> {
    optional_header(request, name).ok_or(())
}

fn optional_header<'a>(request: &'a HttpRequest, name: &str) -> Option<&'a str> {
    request
        .header(name)
        .map(str::trim)
        .filter(|value| !value.is_empty())
}

fn parse_i64(value: &str) -> Result<i64, std::num::ParseIntError> {
    value.parse::<i64>()
}

fn protocol_adapters_json(runtime: &AiotRuntime) -> String {
    let adapters = runtime
        .protocol_routes()
        .iter()
        .map(|route| {
            let adapter = runtime.protocol_adapter_for(&route.protocol_id);
            let scope = adapter
                .map(|adapter| format!("{:?}", adapter.scope))
                .unwrap_or_default();
            let transports = adapter
                .map(|adapter| debug_array(adapter.transports.iter()))
                .unwrap_or_default();
            let codecs = adapter
                .map(|adapter| debug_array(adapter.codecs.iter()))
                .unwrap_or_default();
            let session_policies = adapter
                .map(|adapter| debug_array(adapter.session_policies.iter()))
                .unwrap_or_default();
            let security_modes = adapter
                .map(|adapter| string_array(adapter.security_modes.iter()))
                .unwrap_or_default();
            let hardware_families = adapter
                .map(|adapter| string_array(adapter.hardware_families.iter()))
                .unwrap_or_default();
            let runtime_profiles = adapter
                .map(|adapter| string_array(adapter.runtime_profiles.iter()))
                .unwrap_or_default();
            let firmware_profiles = adapter
                .map(|adapter| string_array(adapter.firmware_profiles.iter()))
                .unwrap_or_default();

            format!(
                r#"{{"path":"{}","protocolId":"{}","pluginId":"{}","scope":"{}","transport":"{}","transports":[{}],"codecs":[{}],"sessionPolicies":[{}],"securityModes":[{}],"hardwareFamilies":[{}],"runtimeProfiles":[{}],"firmwareProfiles":[{}],"kind":"{}"}}"#,
                route.path,
                route.protocol_id,
                route.plugin_id,
                scope,
                format!("{:?}", route.transport),
                transports,
                codecs,
                session_policies,
                security_modes,
                hardware_families,
                runtime_profiles,
                firmware_profiles,
                route_kind_name(route.kind)
            )
        })
        .collect::<Vec<_>>()
        .join(",");

    format!(r#"{{"code":"0","data":[{adapters}]}}"#)
}

fn runtime_capacity_json() -> String {
    let policy = sdkwork_aiot_runtime::AiotRuntimeCapacityPolicy::standard();

    format!(
        r#"{{"code":"0","data":{{"nodeId":"{}","maxConnectionsPerNode":"{}","maxSessionsPerTenant":"{}","maxInflightPerDevice":{},"sessionLeaseTtlSeconds":{},"sessionLeaseRenewSeconds":{},"outboxMaxAttempts":{},"deadLetterAfterAttempts":{},"backpressure":{{"warnLag":"{}","rejectLag":"{}","deadLetterLag":"{}"}},"orderedDeviceCommands":{},"idempotentIngest":{}}}}}"#,
        policy.node_id,
        policy.max_connections_per_node,
        policy.max_sessions_per_tenant,
        policy.max_inflight_per_device,
        policy.session_lease_ttl_seconds,
        policy.session_lease_renew_seconds,
        policy.outbox_max_attempts,
        policy.dead_letter_after_attempts,
        policy.outbox_warn_lag,
        policy.outbox_reject_lag,
        policy.outbox_dead_letter_lag,
        policy.enable_ordered_device_commands,
        policy.enable_idempotent_ingest
    )
}

fn standard_empty_collection_response() -> HttpResponse {
    HttpResponse::new(HttpStatus::Ok)
        .with_header("content-type", "application/json")
        .with_body(r#"{"code":"0","data":[]}"#)
}

fn debug_array<'a, T, I>(values: I) -> String
where
    T: std::fmt::Debug + 'a,
    I: IntoIterator<Item = &'a T>,
{
    values
        .into_iter()
        .map(|value| format!(r#""{value:?}""#))
        .collect::<Vec<_>>()
        .join(",")
}

fn string_array<'a, I>(values: I) -> String
where
    I: IntoIterator<Item = &'a String>,
{
    values
        .into_iter()
        .map(|value| format!(r#""{}""#, json_escape(value)))
        .collect::<Vec<_>>()
        .join(",")
}

fn json_escape(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

fn route_kind_name(kind: sdkwork_aiot_runtime::AiotProtocolRouteKind) -> &'static str {
    match kind {
        sdkwork_aiot_runtime::AiotProtocolRouteKind::DeviceSession => "deviceSession",
        sdkwork_aiot_runtime::AiotProtocolRouteKind::OtaMetadata => "otaMetadata",
        sdkwork_aiot_runtime::AiotProtocolRouteKind::Provisioning => "provisioning",
        sdkwork_aiot_runtime::AiotProtocolRouteKind::BridgeIngress => "bridgeIngress",
        sdkwork_aiot_runtime::AiotProtocolRouteKind::Callback => "callback",
    }
}

fn problem_response(status: HttpStatus, code: &str, title: &str) -> HttpResponse {
    HttpResponse::new(status)
        .with_header("content-type", "application/problem+json")
        .with_body(format!(
            r#"{{"type":"about:blank","title":"{}","status":{},"code":"{}"}}"#,
            title,
            status.code(),
            code
        ))
}

fn permission_denied_response(required_permission: &str) -> HttpResponse {
    HttpResponse::new(HttpStatus::Forbidden)
        .with_header("content-type", "application/problem+json")
        .with_body(format!(
            r#"{{"type":"about:blank","title":"Permission denied","status":403,"code":"api.permission.denied","requiredPermission":"{}"}}"#,
            json_escape(required_permission)
        ))
}

fn parse_http_request(bytes: &[u8]) -> Result<HttpRequest, AiotApiError> {
    let raw = std::str::from_utf8(bytes).map_err(|_| AiotApiError::new("api.http.invalid_utf8"))?;
    let mut lines = raw.split("\r\n");
    let request_line = lines
        .next()
        .ok_or_else(|| AiotApiError::new("api.http.empty"))?;
    let mut parts = request_line.split_whitespace();
    let method = parts
        .next()
        .ok_or_else(|| AiotApiError::new("api.http.missing_method"))?;
    let path = parts
        .next()
        .ok_or_else(|| AiotApiError::new("api.http.missing_path"))?;
    let mut request = HttpRequest::new(method, path);

    for line in lines {
        if line.is_empty() {
            break;
        }
        if let Some((name, value)) = line.split_once(':') {
            request = request.with_header(name.trim(), value.trim());
        }
    }

    Ok(request)
}

fn format_http_response(response: &HttpResponse) -> String {
    let mut out = format!(
        "HTTP/1.1 {} {}\r\n",
        response.status.code(),
        response.status.reason()
    );
    for (name, value) in response.headers() {
        out.push_str(name);
        out.push_str(": ");
        out.push_str(value);
        out.push_str("\r\n");
    }
    out.push_str("content-length: ");
    out.push_str(response.body.len().to_string().as_str());
    out.push_str("\r\n\r\n");
    out.push_str(&response.body);
    out
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiotApiError {
    pub code: String,
}

impl AiotApiError {
    pub fn new(code: impl Into<String>) -> Self {
        Self { code: code.into() }
    }
}
