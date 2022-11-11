use std::net::{TcpListener, TcpStream};
use serde::{Deserialize, Serialize};
use std::io::prelude::*;

#[derive(Deserialize, Serialize)]
enum Message {
    Login(Login),
    Logout(String),
    SendMsg(String),
}

#[derive(Deserialize, Serialize)]
enum Login {
    UserPass(UserPassLogin),
    Token(TokenLogin),
}

#[derive(Deserialize, Serialize)]
struct UserPassLogin {
    username: String,
    password: String,
}

#[derive(Deserialize, Serialize)]
struct TokenLogin {
    token: String,
}

pub fn handle_connection(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    match stream.read(&mut data) {
        Ok(_) => {
            let msg: Message = serde_json::from_slice(&data).unwrap();

        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

/*

Steps after client connects to server:
1.) send 

*/