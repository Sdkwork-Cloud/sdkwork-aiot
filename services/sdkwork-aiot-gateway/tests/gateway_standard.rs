use sdkwork_aiot_gateway::standard_gateway_server;
use sdkwork_aiot_transport::handle_http_request_bytes;

#[test]
fn standard_gateway_server_mounts_xiaozhi_ota_compatibility_route() {
    let server = standard_gateway_server().expect("gateway server");

    let response = handle_http_request_bytes(
        &server,
        b"POST /iot/xiaozhi/ota HTTP/1.1\r\nHost: domain\r\nContent-Type: application/json\r\n\r\n{}",
    )
    .expect("xiaozhi ota response");

    assert!(response.starts_with("HTTP/1.1 200 OK"));
    assert!(response.contains(r#""websocket":{"url":"wss://domain/iot/xiaozhi/ws""#));
    assert!(response.contains(r#""version":3"#));
    assert!(response.contains(r#""server_time":{"timestamp":"#));
}

#[test]
fn standard_gateway_server_mounts_xiaozhi_activation_pending_route() {
    let server = standard_gateway_server().expect("gateway server");

    let response = handle_http_request_bytes(
        &server,
        b"POST /iot/xiaozhi/activate HTTP/1.1\r\nHost: domain\r\nContent-Type: application/json\r\n\r\n{}",
    )
    .expect("xiaozhi activation response");

    assert!(response.starts_with("HTTP/1.1 202 Accepted"));
    assert!(response.contains(r#""activation":{"status":"pending""#));
}

#[test]
fn standard_gateway_server_enforces_xiaozhi_activation_post_method() {
    let server = standard_gateway_server().expect("gateway server");

    let response = handle_http_request_bytes(
        &server,
        b"GET /iot/xiaozhi/activate HTTP/1.1\r\nHost: domain\r\n\r\n",
    )
    .expect("xiaozhi activation method response");

    assert!(response.starts_with("HTTP/1.1 400 Bad Request"));
    assert!(response.contains("gateway.xiaozhi.activate.method"));
}
