// main.rs
mod packet_metadata;
mod database;
mod tcp_server;
mod env;

use tokio::net::TcpListener;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::database::connect_to_database;
use crate::packet_metadata::PacketMetadata;

use crate::tcp_server::handle_client;
use crate::env::{SERVER_ADDRESS}; 

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_client = connect_to_database().await?;

    let db_client = Arc::new(Mutex::new(db_client));

    let listener = TcpListener::bind(SERVER_ADDRESS).await?;
    println!("Server listening on {}", SERVER_ADDRESS);

    while let Ok((stream, _)) = listener.accept().await {
        let db_client_clone = Arc::clone(&db_client);
        tokio::spawn(handle_client(stream, db_client_clone));
    }

    Ok(())
}
