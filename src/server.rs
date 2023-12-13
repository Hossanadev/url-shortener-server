use serde::Serialize;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
};

const HOST: &str = "127.0.0.1";
const PORT: &str = "8080";
const CORS_HEADERS: &str = "Access-Control-Allow-Headers: Content-Type\r\n\
                            Access-Control-Allow-Methods: POST, GET, OPTIONS\r\n\
                            Access-Control-Allow-Origin: *\r\n";
#[derive(Debug, Serialize)]
struct Url {
    id: i32,
    long_url: String,
    short_url: String,
    date_created: String,
    clicks: String,
}

pub fn server() {
    let listener = TcpListener::bind(format!("{HOST}:{PORT}")).unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let url_data = vec![
        Url {
            id: 1,
            long_url: "https://www.google.com".to_string(),
            short_url: "https://localhost:8080/1".to_string(),
            date_created: "2022-01-01".to_string(),
            clicks: "0".to_string(),
        },
        Url {
            id: 2,
            long_url: "https://www.facebook.com".to_string(),
            short_url: "https://localhost:8080/2".to_string(),
            date_created: "2022-01-01".to_string(),
            clicks: "1".to_string(),
        },
        Url {
            id: 3,
            long_url: "https://www.twitter.com".to_string(),
            short_url: "https://localhost:8080/3".to_string(),
            date_created: "2022-01-01".to_string(),
            clicks: "0".to_string(),
        },
        Url {
            id: 4,
            long_url: "https://www.linkedin.com".to_string(),
            short_url: "https://localhost:8080/4".to_string(),
            date_created: "2022-01-01".to_string(),
            clicks: "0".to_string(),
        },
    ];

    let buf_reader = BufReader::new(&mut stream);
    if let Some(Ok(http_request)) = buf_reader.lines().next() {
        println!("Request: {:#?}", http_request);

        let response = match http_request.as_str() {
            "POST /login HTTP/1.1" => generate_response(
                serde_json::json!({"email": "hossanadev@gmail.com", "password": "12345"}),
            ),
            "GET /list HTTP/1.1" => generate_response(serde_json::json!({"data": url_data})),
            "GET /listStats HTTP/1.1" => generate_response(
                serde_json::json!({"stats": {"total_clicked": 1, "total_unclicked": 3, "total_shortened": 4, "total_deleted": 0}}),
            ),
            _ => generate_response(serde_json::json!({"error": "Not Found!"})),
        };

        if let Err(e) = stream.write_all(response.as_bytes()) {
            eprintln!("Error writing to stream: {}", e);
        }
    }
}

fn generate_response(json_data: serde_json::Value) -> String {
    let content_type = "Content-Type: application/json";
    let server_header = "Server: URL Manager";
    let json_str = serde_json::to_string(&json_data).unwrap();
    format!(
        "HTTP/1.1 200 OK\r\n{content_type}\r\n{server_header}\r\n{cors_headers}Content-Length: {}\r\n\r\n{}",
        json_str.len(),
        json_str,
        content_type = content_type,
        server_header = server_header,
        cors_headers = CORS_HEADERS,
    )
}
