use sdkwork_aiot_observability::{redact_header, RuntimeMetricFields, TraceFields};

#[test]
fn trace_fields_include_safe_iot_correlation_context() {
    let fields = TraceFields::new("trace1")
        .tenant("t1")
        .device("dev1")
        .protocol("xiaozhi.websocket")
        .session("sess1");

    assert_eq!(fields.trace_id, "trace1");
    assert_eq!(fields.device_id.as_deref(), Some("dev1"));
}

#[test]
fn sensitive_headers_are_redacted() {
    assert_eq!(
        redact_header("Authorization", "Bearer secret"),
        "<redacted>"
    );
    assert_eq!(redact_header("Device-Id", "aa:bb"), "aa:bb");
}

#[test]
fn runtime_metric_fields_cover_capacity_and_backpressure_without_payloads() {
    let fields = RuntimeMetricFields::new("sdkwork-aiot-gateway")
        .tenant("t1")
        .protocol("xiaozhi.websocket")
        .node("node-a")
        .connections(90_000)
        .sessions(900_000)
        .device_inflight(32)
        .outbox_lag(100_000)
        .backpressure("slow_down");

    assert_eq!(fields.component, "sdkwork-aiot-gateway");
    assert_eq!(fields.tenant_id.as_deref(), Some("t1"));
    assert_eq!(fields.protocol_id.as_deref(), Some("xiaozhi.websocket"));
    assert_eq!(fields.node_id.as_deref(), Some("node-a"));
    assert_eq!(fields.node_connections, Some(90_000));
    assert_eq!(fields.tenant_sessions, Some(900_000));
    assert_eq!(fields.device_inflight, Some(32));
    assert_eq!(fields.outbox_lag, Some(100_000));
    assert_eq!(fields.backpressure_action.as_deref(), Some("slow_down"));
    assert!(!fields.contains_payload_fields());
}
