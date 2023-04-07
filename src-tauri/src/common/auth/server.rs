use std::borrow::Cow;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use tauri::Manager;

use crate::auth::{bearer_token, login};

async fn handle_read(mut stream: &TcpStream, app: &tauri::AppHandle) {
    let mut buf: [u8; 4096] = [0u8; 4096];
    match stream.read(&mut buf) {
        Ok(_) => {
            let req_str: Cow<str> = String::from_utf8_lossy(&buf);
            let code: &str = req_str
                .split("code=")
                .nth(1)
                .and_then(|s| s.split("&").next())
                .unwrap_or("");
            app.emit_all(
                "auth",
                login::LoginEventPayload {
                    message: format!("Obtaining bearer token."),
                    status: String::from("Loading"),
                },
            )
            .unwrap();
            bearer_token::get_bearer_token(code, app).await;
        }
        Err(e) => {
            println!("Unable to read stream: {}", e);
        }
    }
}

fn handle_write(mut stream: TcpStream) {
    let response: &[u8; 111]= b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Wait to be redirected...</body></html>\r\n";
    match stream.write(response) {
        Ok(_) => println!("Response sent"),
        Err(e) => println!("Failed sending response: {}", e),
    }
}

async fn handle_client(stream: TcpStream, app: &tauri::AppHandle) {
    app.get_window("auth").unwrap().close().unwrap();
    handle_read(&stream, app).await;
    handle_write(stream)
}

pub async fn start_server(app: &tauri::AppHandle) {
    // Check if the server is already running
    match TcpListener::bind("0.0.0.0:7222") {
        Ok(_) => (),
        Err(_) => return,
    }
    let listener: TcpListener = TcpListener::bind("0.0.0.0:7222").unwrap();
    println!("Server listening  on port 7222",);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream, app).await;
                return;
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}
