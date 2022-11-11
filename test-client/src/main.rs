use std::io::prelude::*;
use std::net::TcpStream;
use serde::{Deserialize, Serialize};

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

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:34254")?;

    let serialized_login = 

    stream.write(&[1])?;
    stream.read(&mut [0; 128])?;
    Ok(())
}