use std::io::{Read, Write};
use std::net::TcpListener;
use std::time::Duration;

fn main() {
    let server = sdkwork_aiot_gateway::standard_gateway_server().expect("gateway transport server");

    println!(
        "sdkwork-aiot-gateway mode={:?} components={} xiaozhi_websocket={}",
        server.runtime.mode(),
        server.runtime.component_names().len(),
        server.runtime.supports_protocol("xiaozhi.websocket")
    );

    if std::env::var("SDKWORK_AIOT_GATEWAY_NO_LISTEN").as_deref() == Ok("1") {
        return;
    }

    let bind_addr = std::env::var("SDKWORK_AIOT_GATEWAY_BIND")
        .unwrap_or_else(|_| "127.0.0.1:18080".to_string());
    let listener = TcpListener::bind(&bind_addr).expect("bind gateway listener");
    println!("sdkwork-aiot-gateway listening on http://{bind_addr}");

    for stream in listener.incoming() {
        let mut stream = match stream {
            Ok(stream) => stream,
            Err(error) => {
                eprintln!("sdkwork-aiot-gateway accept_error={error}");
                continue;
            }
        };
        let _ = stream.set_read_timeout(Some(Duration::from_secs(5)));

        let mut buffer = [0u8; 8192];
        let read = match stream.read(&mut buffer) {
            Ok(read) => read,
            Err(error) => {
                eprintln!("sdkwork-aiot-gateway read_error={error}");
                continue;
            }
        };
        if read == 0 {
            continue;
        }

        let response =
            match sdkwork_aiot_transport::handle_http_request_bytes(&server, &buffer[..read]) {
                Ok(response) => response,
                Err(error) => format!(
                    "HTTP/1.1 400 Bad Request\r\ncontent-type: application/problem+json\r\ncontent-length: {}\r\n\r\n{}",
                    error.code.len() + 65,
                    format!(
                        r#"{{"type":"about:blank","title":"Bad Request","status":400,"code":"{}"}}"#,
                        error.code
                    )
                ),
            };

        if let Err(error) = stream.write_all(response.as_bytes()) {
            eprintln!("sdkwork-aiot-gateway write_error={error}");
        }
    }
}
