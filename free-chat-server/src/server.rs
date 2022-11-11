use std::net::{TcpListener, TcpStream};
use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use log::{debug, error, warn, log_enabled, info, Level};

// #[derive(Deserialize, Serialize)]
// enum ClientMessage {

// }

#[derive(Deserialize, Serialize)]
enum ServerMessage {
    Login(Login),
    Logout(Logout),
    RelayMsg(Message),
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

#[derive(Deserialize, Serialize)]
struct Message {
    sender: String,
    recipient: String,
    message: String,
}

#[derive(Deserialize, Serialize)]
struct Logout {

}

pub async fn handle_connection(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    match stream.read(&mut data) {
        Ok(_) => {
            let msg: ServerMessage = serde_json::from_slice(&data).unwrap();

            match msg {
                ServerMessage::Login(login) => {
                    handle_login(login);
                },
                ServerMessage::Logout(_logout) => {
                    error!("Unable to logout as first server message");
                },
                ServerMessage::RelayMsg(msg) => {
                    error!("Unable to send message: '{}' as first server message", msg.message);
                }
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

async fn handle_login(login: Login) {
    match login {
        Login::UserPass(credentials) => {
            info!("Login attempt with Username: {}, Password: {}", credentials.username, credentials.password);
        },
        Login::Token(token) => {
            info!("Login attempt with Token: {}", token.token);
        }
    }
}