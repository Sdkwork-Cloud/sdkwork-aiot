use std::time::{SystemTime, UNIX_EPOCH};

use sdkwork_aiot_adapter_xiaozhi::{
    xiaozhi_activation_pending_response, xiaozhi_ota_response, XiaozhiOtaMetadata,
    XIAOZHI_ACTIVATE_PATH, XIAOZHI_OTA_PATH, XIAOZHI_WS_PATH,
};
use sdkwork_aiot_transport::{
    HttpRequest, HttpResponse, HttpStatus, TransportError, TransportServer,
};

pub fn standard_gateway_server() -> Result<TransportServer, TransportError> {
    Ok(TransportServer::standard_standalone()?
        .with_http_compatibility_route(XIAOZHI_OTA_PATH, xiaozhi_ota_http_handler)
        .with_http_compatibility_route(XIAOZHI_ACTIVATE_PATH, xiaozhi_activation_http_handler))
}

pub fn xiaozhi_ota_http_handler(request: &HttpRequest) -> HttpResponse {
    if request.method != "POST" && request.method != "GET" {
        return problem_response(HttpStatus::BadRequest, "gateway.xiaozhi.ota.method");
    }

    let host = request.header("host").unwrap_or("localhost");
    let ws_scheme = websocket_scheme(request);
    let token = request
        .header("authorization")
        .map(str::to_string)
        .or_else(|| std::env::var("SDKWORK_AIOT_XIAOZHI_DEVICE_TOKEN").ok())
        .unwrap_or_else(|| "device-token".to_string());
    let version = request
        .header("protocol-version")
        .and_then(|value| value.parse::<u32>().ok())
        .unwrap_or(3);

    let metadata = XiaozhiOtaMetadata::new()
        .with_websocket(
            format!("{ws_scheme}://{host}{XIAOZHI_WS_PATH}"),
            token,
            version,
        )
        .with_server_time(current_unix_time_millis(), 480);

    HttpResponse::new(HttpStatus::Ok)
        .with_header("content-type", "application/json")
        .with_body(xiaozhi_ota_response(metadata))
}

pub fn xiaozhi_activation_http_handler(request: &HttpRequest) -> HttpResponse {
    if request.method != "POST" {
        return problem_response(HttpStatus::BadRequest, "gateway.xiaozhi.activate.method");
    }

    HttpResponse::new(HttpStatus::Accepted)
        .with_header("content-type", "application/json")
        .with_body(xiaozhi_activation_pending_response("activation pending"))
}

fn websocket_scheme(request: &HttpRequest) -> &'static str {
    match request.header("x-forwarded-proto") {
        Some(proto) if proto.eq_ignore_ascii_case("http") => "ws",
        _ => "wss",
    }
}

fn current_unix_time_millis() -> i64 {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    i64::try_from(duration.as_millis()).unwrap_or(i64::MAX)
}

fn problem_response(status: HttpStatus, code: &str) -> HttpResponse {
    HttpResponse::new(status)
        .with_header("content-type", "application/problem+json")
        .with_body(format!(
            r#"{{"type":"about:blank","title":"{}","status":{},"code":"{}"}}"#,
            status.reason(),
            status.code(),
            code
        ))
}
