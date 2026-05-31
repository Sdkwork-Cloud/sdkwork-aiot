use std::io::{Read, Write};
use std::net::TcpListener;
use std::time::Duration;

fn main() {
    let server = sdkwork_aiot_http_api::standard_app_api_server().expect("app api server");
    let plan = sdkwork_aiot_runtime::RuntimeServicePlan::standard();

    println!(
        "sdkwork-aiot-app-api mode={:?} app_routes={} components={}",
        server.runtime().mode(),
        plan.app_routes.len(),
        server.runtime().component_names().len()
    );

    if std::env::var("SDKWORK_AIOT_APP_API_NO_LISTEN").as_deref() == Ok("1") {
        return;
    }

    let bind_addr = std::env::var("SDKWORK_AIOT_APP_API_BIND")
        .unwrap_or_else(|_| "127.0.0.1:18082".to_string());
    serve(&server, &bind_addr);
}

fn serve(server: &sdkwork_aiot_http_api::AiotApiServer, bind_addr: &str) {
    let listener = TcpListener::bind(bind_addr).expect("bind app api listener");
    println!("sdkwork-aiot-app-api listening on http://{bind_addr}");

    for stream in listener.incoming() {
        let mut stream = match stream {
            Ok(stream) => stream,
            Err(error) => {
                eprintln!("sdkwork-aiot-app-api accept_error={error}");
                continue;
            }
        };
        let _ = stream.set_read_timeout(Some(Duration::from_secs(5)));

        let mut buffer = [0u8; 8192];
        let read = match stream.read(&mut buffer) {
            Ok(read) => read,
            Err(error) => {
                eprintln!("sdkwork-aiot-app-api read_error={error}");
                continue;
            }
        };
        if read == 0 {
            continue;
        }

        let response =
            match sdkwork_aiot_http_api::handle_api_request_bytes(server, &buffer[..read]) {
                Ok(response) => response,
                Err(error) => problem_response(&error.code),
            };

        if let Err(error) = stream.write_all(response.as_bytes()) {
            eprintln!("sdkwork-aiot-app-api write_error={error}");
        }
    }
}

fn problem_response(code: &str) -> String {
    let body = format!(
        r#"{{"type":"about:blank","title":"Bad Request","status":400,"code":"{}"}}"#,
        code
    );
    format!(
        "HTTP/1.1 400 Bad Request\r\ncontent-type: application/problem+json\r\ncontent-length: {}\r\n\r\n{}",
        body.len(),
        body
    )
}
