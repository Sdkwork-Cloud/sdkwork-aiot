use sdkwork_aiot_contract::AiotRequestContext;
use sdkwork_aiot_protocol::{
    InboundFrame, MessageClass, MessageCodec, OutboundFrame, ProtocolEnvelope, ProtocolError,
};
use sdkwork_aiot_runtime::AiotProtocolMessageAction;
use sdkwork_aiot_storage::AiotStorageWriteKind;
use sdkwork_aiot_transport::{
    build_health_response, build_websocket_handshake_response, decode_websocket_frame,
    handle_http_request_bytes, handle_websocket_message_bytes,
    handle_websocket_message_bytes_with_context, websocket_frame_to_inbound_frame, HttpRequest,
    HttpResponse, HttpStatus, TransportServer, WebSocketFrame, WebSocketOpcode,
};

#[derive(Debug, Clone)]
struct FakeCodec {
    envelope: ProtocolEnvelope,
}

impl MessageCodec for FakeCodec {
    fn decode(&self, _frame: InboundFrame) -> Result<ProtocolEnvelope, ProtocolError> {
        Ok(self.envelope.clone())
    }

    fn encode(&self, _envelope: ProtocolEnvelope) -> Result<OutboundFrame, ProtocolError> {
        Ok(OutboundFrame::text("{}"))
    }
}

#[test]
fn health_response_is_plain_http_and_safe_for_load_balancers() {
    let response = build_health_response("sdkwork-aiot-gateway", true);

    assert_eq!(response.status, HttpStatus::Ok);
    assert_eq!(response.header("content-type"), Some("application/json"));
    assert!(response.body.contains("\"ready\":true"));
    assert!(response.body.contains("sdkwork-aiot-gateway"));
}

#[test]
fn websocket_handshake_response_uses_standard_upgrade_headers() {
    let request = HttpRequest::new("GET", "/iot/xiaozhi/ws")
        .with_header("Host", "localhost")
        .with_header("Upgrade", "websocket")
        .with_header("Connection", "Upgrade")
        .with_header("Sec-WebSocket-Key", "dGhlIHNhbXBsZSBub25jZQ==")
        .with_header("Sec-WebSocket-Version", "13");

    let response = build_websocket_handshake_response(&request).expect("handshake");

    assert_eq!(response.status, HttpStatus::SwitchingProtocols);
    assert_eq!(response.header("upgrade"), Some("websocket"));
    assert_eq!(response.header("connection"), Some("Upgrade"));
    assert_eq!(
        response.header("sec-websocket-accept"),
        Some("s3pPLMBiTxaQ9kYGzzhZRbK+xOo=")
    );
}

#[test]
fn websocket_decoder_supports_unmasked_server_side_test_frames() {
    let frame =
        decode_websocket_frame(&[0x81, 0x05, b'h', b'e', b'l', b'l', b'o']).expect("text frame");

    assert_eq!(frame, WebSocketFrame::text("hello"));

    let binary = decode_websocket_frame(&[0x82, 0x02, 0x01, 0x02]).expect("binary frame");
    assert_eq!(binary.opcode, WebSocketOpcode::Binary);
    assert_eq!(binary.payload, vec![1, 2]);

    let close = decode_websocket_frame(&[0x88, 0x00]).expect("close frame");
    assert_eq!(close.opcode, WebSocketOpcode::Close);
}

#[test]
fn transport_server_can_be_composed_from_runtime_bundle_without_binding_ports() {
    let server = TransportServer::standard_standalone().expect("server");

    assert!(server.runtime.supports_protocol("xiaozhi.websocket"));
    assert!(server
        .listeners
        .websocket_routes
        .contains(&"/iot/xiaozhi/ws"));
    assert!(server.health.ready);
}

#[test]
fn transport_server_resolves_protocol_route_metadata_from_runtime_registry() {
    let server = TransportServer::standard_standalone().expect("server");

    let route = server
        .protocol_route_for_path("/iot/xiaozhi/ws")
        .expect("xiaozhi route");

    assert_eq!(route.protocol_id, "xiaozhi.websocket");
    assert_eq!(route.plugin_id, "xiaozhi");
    assert_eq!(route.path, "/iot/xiaozhi/ws");
}

#[test]
fn websocket_frames_convert_to_transport_neutral_inbound_frames() {
    let text = websocket_frame_to_inbound_frame(WebSocketFrame::text(r#"{"type":"hello"}"#));
    assert!(!text.binary);
    assert_eq!(text.payload, br#"{"type":"hello"}"#);

    let binary = websocket_frame_to_inbound_frame(WebSocketFrame {
        opcode: WebSocketOpcode::Binary,
        payload: vec![0x01, 0x02, 0x03],
    });
    assert!(binary.binary);
    assert_eq!(binary.payload, vec![0x01, 0x02, 0x03]);
}

#[test]
fn transport_server_handles_health_ready_and_websocket_upgrade_requests() {
    let server = TransportServer::standard_standalone().expect("server");

    let health =
        handle_http_request_bytes(&server, b"GET /healthz HTTP/1.1\r\nHost: local\r\n\r\n")
            .expect("health");
    assert!(health.starts_with("HTTP/1.1 200"));
    assert!(health.contains("\"ready\":true"));

    let ready = handle_http_request_bytes(&server, b"GET /readyz HTTP/1.1\r\nHost: local\r\n\r\n")
        .expect("ready");
    assert!(ready.starts_with("HTTP/1.1 200"));

    let upgrade = handle_http_request_bytes(
        &server,
        b"GET /iot/xiaozhi/ws HTTP/1.1\r\nHost: local\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\nSec-WebSocket-Version: 13\r\n\r\n",
    )
    .expect("upgrade");
    assert!(upgrade.starts_with("HTTP/1.1 101"));
    assert!(upgrade.contains("sec-websocket-accept: s3pPLMBiTxaQ9kYGzzhZRbK+xOo="));
}

#[test]
fn transport_server_mounts_compatibility_http_routes_without_protocol_specific_dependency() {
    let server = TransportServer::standard_standalone()
        .expect("server")
        .with_http_compatibility_route("/iot/xiaozhi/ota", |request| {
            assert_eq!(request.method, "POST");
            HttpResponse::new(HttpStatus::Ok)
                .with_header("content-type", "application/json")
                .with_body(r#"{"websocket":{"url":"wss://domain/iot/xiaozhi/ws","token":"device-token","version":3}}"#)
        })
        .with_http_compatibility_route("/iot/xiaozhi/activate", |_request| {
            HttpResponse::new(HttpStatus::Accepted)
                .with_header("content-type", "application/json")
                .with_body(r#"{"activation":{"status":"pending"}}"#)
        });

    let ota = handle_http_request_bytes(
        &server,
        b"POST /iot/xiaozhi/ota HTTP/1.1\r\nHost: local\r\nContent-Type: application/json\r\n\r\n{}",
    )
    .expect("ota response");
    let activate = handle_http_request_bytes(
        &server,
        b"POST /iot/xiaozhi/activate HTTP/1.1\r\nHost: local\r\nContent-Type: application/json\r\n\r\n{}",
    )
    .expect("activation response");

    assert!(ota.starts_with("HTTP/1.1 200 OK"));
    assert!(ota.contains(r#""websocket":{"url":"wss://domain/iot/xiaozhi/ws""#));
    assert!(activate.starts_with("HTTP/1.1 202 Accepted"));
    assert!(activate.contains(r#""activation":{"status":"pending"}"#));
}

#[test]
fn transport_websocket_message_handler_uses_runtime_pipeline_without_protocol_specific_dependency()
{
    let server = TransportServer::standard_standalone().expect("server");
    let codec = FakeCodec {
        envelope: ProtocolEnvelope::builder("xiaozhi.websocket", MessageClass::Handshake)
            .device("device-001")
            .session("session-001")
            .semantic_type("hello")
            .build(),
    };
    let frame_bytes = [
        0x81, 0x10, b'{', b'"', b't', b'y', b'p', b'e', b'"', b':', b'"', b'h', b'e', b'l', b'l',
        b'o', b'"', b'}',
    ];

    let result = handle_websocket_message_bytes(&server, "/iot/xiaozhi/ws", &codec, &frame_bytes)
        .expect("pipeline result");

    assert_eq!(
        result.message.action,
        AiotProtocolMessageAction::OpenSession
    );
    assert_eq!(
        result.storage_command.kind,
        AiotStorageWriteKind::OpenSession
    );
    assert_eq!(result.storage_command.primary_table, "iot_device_session");
}

#[test]
fn transport_websocket_message_handler_can_pass_resolved_appbase_context_to_runtime_pipeline() {
    let server = TransportServer::standard_standalone().expect("server");
    let codec = FakeCodec {
        envelope: ProtocolEnvelope::builder("xiaozhi.websocket", MessageClass::Handshake)
            .device("device-001")
            .session("session-001")
            .semantic_type("hello")
            .build(),
    };
    let ctx = AiotRequestContext::new("10001", "20001")
        .with_user("30001")
        .with_data_scope("7");
    let frame_bytes = [
        0x81, 0x10, b'{', b'"', b't', b'y', b'p', b'e', b'"', b':', b'"', b'h', b'e', b'l', b'l',
        b'o', b'"', b'}',
    ];

    let result = handle_websocket_message_bytes_with_context(
        &server,
        "/iot/xiaozhi/ws",
        &ctx,
        &codec,
        &frame_bytes,
    )
    .expect("pipeline result");

    assert_eq!(result.storage_command.association.tenant_id, 10001);
    assert_eq!(result.storage_command.association.organization_id, 20001);
    assert_eq!(result.storage_command.association.user_id, Some(30001));
    assert_eq!(result.storage_command.association.data_scope, 7);
}
