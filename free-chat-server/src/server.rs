use std::net::{TcpListener, TcpStream};
use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use log::{debug, error, warn, log_enabled, info, Level};
use tokio::time::{Duration, sleep};
use mini_redis::{client};

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
                    match handle_login(login).await {
                        Ok(_) => {
                            info!("Login successful");
                        },
                        Err(e) => {
                            error!("Login failed: {}", e);
                        }
                    }
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

async fn handle_login(login: Login) -> Result<i32, String> {
    sleep(Duration::from_millis(100)).await; // simulated database lookup
    match login {
        Login::UserPass(user_pass_login) => {
            if user_pass_login.username == "user" && user_pass_login.password == "pass" {
                Ok(123) // returns session ID
            } else {
                Err("invalid username or password".to_string())
            }
        },
        Login::Token(token_login) => {
            if token_login.token == "token" {
                Ok(123) // returns session ID
            } else {
                Err("invalid token".to_string())
            }
        }
    }
}