use std::net::{TcpListener, TcpStream};
use log::{debug, error, warn, log_enabled, info, Level};
use env_logger::Env;
use mini_redis::{client, Result};

mod server;

#[tokio::main]
async fn main() -> Result<()>  {
    env_logger::Builder::from_env(Env::default()
        .default_filter_or("debug")) // set to "info" level for production
        .init();

    info!("Starting server...");
    let listener = TcpListener::bind("127.0.0.1:9123").unwrap();
    info!("Server started on port 9123");

    for stream in listener.incoming() {
        tokio::spawn(async move {
            let mut stream: TcpStream = stream.unwrap();
            info!("New connection: {}", stream.peer_addr().unwrap());
            server::handle_connection(stream).await;
        });
    }

    Ok(())
}
