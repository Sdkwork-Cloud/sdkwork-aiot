use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::{RuntimeMetricFields, TraceFields};

const DEFAULT_OTLP_TIMEOUT_MS: u64 = 500;
const DEFAULT_OTLP_SERVICE_NAME: &str = "sdkwork-aiot";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OtlpEndpoint {
    pub host: String,
    pub port: u16,
    pub path: String,
}

pub fn configured_otlp_endpoint() -> Option<OtlpEndpoint> {
    let raw = std::env::var("SDKWORK_AIOT_OTLP_ENDPOINT")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())?;
    parse_otlp_endpoint(&raw)
}

pub fn otlp_export_enabled() -> bool {
    configured_otlp_endpoint().is_some()
}

pub fn configured_otlp_service_name() -> String {
    std::env::var("SDKWORK_AIOT_OTLP_SERVICE_NAME")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| DEFAULT_OTLP_SERVICE_NAME.to_string())
}

pub fn export_trace_event(event: &str, fields: &TraceFields) {
    let Some(endpoint) = configured_otlp_endpoint() else {
        return;
    };
    let payload = format_otlp_trace_payload(event, fields, &configured_otlp_service_name());
    dispatch_otlp_export(endpoint, payload);
}

pub fn export_runtime_metric(fields: &RuntimeMetricFields) {
    let Some(endpoint) = configured_otlp_endpoint() else {
        return;
    };
    let payload = format_otlp_runtime_metric_payload(fields, &configured_otlp_service_name());
    dispatch_otlp_export(endpoint, payload);
}

pub fn export_api_request_trace(method: &str, path: &str, status: u16) {
    let Some(endpoint) = configured_otlp_endpoint() else {
        return;
    };
    let trace_id = otlp_hex_id(&format!("api:{method}:{path}"), 16);
    let fields = TraceFields::new(trace_id).session(format!("{method} {path}"));
    let mut payload =
        format_otlp_trace_payload("api.request", &fields, &configured_otlp_service_name());
    payload = inject_span_attributes(
        &payload,
        &[
            ("http.method", method),
            ("http.route", path),
            ("http.status_code", &status.to_string()),
        ],
    );
    dispatch_otlp_export(endpoint, payload);
}

pub fn format_otlp_trace_payload(event: &str, fields: &TraceFields, service_name: &str) -> String {
    let trace_id = otlp_hex_id(&fields.trace_id, 16);
    let span_id = otlp_hex_id(&format!("{event}:{}", fields.trace_id), 8);
    let now_nano = unix_time_nanos();
    let mut attributes = vec![
        otlp_string_attribute("event.name", event),
        otlp_string_attribute("trace.source_id", &fields.trace_id),
    ];
    append_optional_string_attribute(&mut attributes, "tenant.id", fields.tenant_id.as_deref());
    append_optional_string_attribute(
        &mut attributes,
        "organization.id",
        fields.organization_id.as_deref(),
    );
    append_optional_string_attribute(&mut attributes, "product.id", fields.product_id.as_deref());
    append_optional_string_attribute(&mut attributes, "device.id", fields.device_id.as_deref());
    append_optional_string_attribute(&mut attributes, "session.id", fields.session_id.as_deref());
    append_optional_string_attribute(
        &mut attributes,
        "connection.id",
        fields.connection_id.as_deref(),
    );
    append_optional_string_attribute(&mut attributes, "adapter.id", fields.adapter_id.as_deref());
    append_optional_string_attribute(
        &mut attributes,
        "protocol.id",
        fields.protocol_id.as_deref(),
    );
    append_optional_string_attribute(
        &mut attributes,
        "message.class",
        fields.message_class.as_deref(),
    );
    append_optional_string_attribute(
        &mut attributes,
        "semantic.type",
        fields.semantic_type.as_deref(),
    );
    append_optional_string_attribute(&mut attributes, "command.id", fields.command_id.as_deref());

    format_otlp_span_payload(
        service_name,
        &trace_id,
        &span_id,
        event,
        now_nano,
        now_nano,
        &attributes,
    )
}

pub fn format_otlp_runtime_metric_payload(
    fields: &RuntimeMetricFields,
    service_name: &str,
) -> String {
    let trace_id = otlp_hex_id(
        &format!(
            "metric:{}:{}",
            fields.component,
            fields.node_id.as_deref().unwrap_or("local")
        ),
        16,
    );
    let span_id = otlp_hex_id(&format!("runtime.metric:{}", fields.component), 8);
    let now_nano = unix_time_nanos();
    let mut attributes = vec![
        otlp_string_attribute("event.name", "runtime.metric"),
        otlp_string_attribute("component", &fields.component),
    ];
    append_optional_string_attribute(&mut attributes, "tenant.id", fields.tenant_id.as_deref());
    append_optional_string_attribute(
        &mut attributes,
        "protocol.id",
        fields.protocol_id.as_deref(),
    );
    append_optional_string_attribute(&mut attributes, "node.id", fields.node_id.as_deref());
    append_optional_u64_attribute(&mut attributes, "node.connections", fields.node_connections);
    append_optional_u64_attribute(&mut attributes, "tenant.sessions", fields.tenant_sessions);
    append_optional_u64_attribute(&mut attributes, "device.inflight", fields.device_inflight);
    append_optional_u64_attribute(&mut attributes, "outbox.lag", fields.outbox_lag);
    append_optional_string_attribute(
        &mut attributes,
        "backpressure.action",
        fields.backpressure_action.as_deref(),
    );

    format_otlp_span_payload(
        service_name,
        &trace_id,
        &span_id,
        "runtime.metric",
        now_nano,
        now_nano,
        &attributes,
    )
}

pub fn parse_otlp_endpoint(raw: &str) -> Option<OtlpEndpoint> {
    let trimmed = raw.trim();
    let without_scheme = trimmed
        .strip_prefix("http://")
        .or_else(|| trimmed.strip_prefix("https://"))?;
    let (authority, path) = match without_scheme.split_once('/') {
        Some((authority, rest)) => (authority, format!("/{rest}")),
        None => (without_scheme, "/v1/traces".to_string()),
    };
    let (host, port) = match authority.rsplit_once(':') {
        Some((host, port)) => (host.to_string(), port.parse().ok()?),
        None => (authority.to_string(), 4318),
    };
    if host.is_empty() {
        return None;
    }
    Some(OtlpEndpoint { host, port, path })
}

fn dispatch_otlp_export(endpoint: OtlpEndpoint, payload: String) {
    std::thread::spawn(move || {
        let _ = post_otlp_json(&endpoint, &payload);
    });
}

fn post_otlp_json(endpoint: &OtlpEndpoint, payload: &str) -> Result<(), String> {
    let addr: SocketAddr = format!("{}:{}", endpoint.host, endpoint.port)
        .parse()
        .map_err(|error: std::net::AddrParseError| error.to_string())?;
    let timeout = Duration::from_millis(otlp_timeout_ms());
    let mut stream =
        TcpStream::connect_timeout(&addr, timeout).map_err(|error| error.to_string())?;
    let _ = stream.set_read_timeout(Some(timeout));
    let _ = stream.set_write_timeout(Some(timeout));

    let request = format!(
        "POST {} HTTP/1.1\r\nHost: {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        endpoint.path,
        endpoint.host,
        payload.len(),
        payload
    );
    stream
        .write_all(request.as_bytes())
        .map_err(|error| error.to_string())?;
    let mut response = Vec::new();
    stream
        .read_to_end(&mut response)
        .map_err(|error| error.to_string())?;
    let response_text = String::from_utf8_lossy(&response);
    if response_text.starts_with("HTTP/1.1 2") || response_text.starts_with("HTTP/1.0 2") {
        Ok(())
    } else {
        Err(format!("otlp_export_failed status={response_text}"))
    }
}

fn format_otlp_span_payload(
    service_name: &str,
    trace_id: &str,
    span_id: &str,
    name: &str,
    start_nano: u128,
    end_nano: u128,
    attributes: &[String],
) -> String {
    let attributes_json = attributes.join(",");
    format!(
        r#"{{"resourceSpans":[{{"resource":{{"attributes":[{{"key":"service.name","value":{{"stringValue":"{service_name}"}}}}]}},"scopeSpans":[{{"spans":[{{"traceId":"{trace_id}","spanId":"{span_id}","name":"{name}","kind":1,"startTimeUnixNano":"{start_nano}","endTimeUnixNano":"{end_nano}","attributes":[{attributes_json}]}}]}}]}}]}}"#
    )
}

fn inject_span_attributes(payload: &str, pairs: &[(&str, &str)]) -> String {
    let extra = pairs
        .iter()
        .map(|(key, value)| otlp_string_attribute(key, value))
        .collect::<Vec<_>>()
        .join(",");
    payload.replacen("\"attributes\":[", &format!("\"attributes\":[{extra},"), 1)
}

fn otlp_string_attribute(key: &str, value: &str) -> String {
    format!(
        r#"{{"key":"{}","value":{{"stringValue":"{}"}}}}"#,
        json_escape(key),
        json_escape(value)
    )
}

fn append_optional_string_attribute(attributes: &mut Vec<String>, key: &str, value: Option<&str>) {
    if let Some(value) = value {
        attributes.push(otlp_string_attribute(key, value));
    }
}

fn append_optional_u64_attribute(attributes: &mut Vec<String>, key: &str, value: Option<u64>) {
    if let Some(value) = value {
        attributes.push(format!(
            r#"{{"key":"{}","value":{{"intValue":"{}"}}}}"#,
            json_escape(key),
            value
        ));
    }
}

fn otlp_hex_id(input: &str, byte_len: usize) -> String {
    let mut state1 = 0u64;
    let mut state2 = 0u64;
    for (index, byte) in input.bytes().enumerate() {
        if index % 2 == 0 {
            state1 = state1.wrapping_mul(31).wrapping_add(u64::from(byte));
        } else {
            state2 = state2.wrapping_mul(37).wrapping_add(u64::from(byte));
        }
    }
    let mut out = format!("{state1:016x}{state2:016x}");
    out.truncate(byte_len * 2);
    if out.len() < byte_len * 2 {
        out.push_str(&"0".repeat(byte_len * 2 - out.len()));
    }
    out
}

fn unix_time_nanos() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|value| value.as_nanos())
        .unwrap_or(0)
}

fn otlp_timeout_ms() -> u64 {
    std::env::var("SDKWORK_AIOT_OTLP_TIMEOUT_MS")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(DEFAULT_OTLP_TIMEOUT_MS)
        .max(1)
}

fn json_escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}
