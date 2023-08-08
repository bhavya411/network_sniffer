// tcp_server.rs
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
// use tokio::net::TcpListener;
// use std::io::Write;
use crate::database::store_metadata;
use crate::packet_metadata::PacketMetadata;
use bincode;
use std::sync::Arc;

pub async fn handle_client(mut stream: TcpStream, db_client: Arc<Mutex<tokio_postgres::Client>>) {
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer).await {
            Ok(size) => {
                if size == 0 {
                    // Connection closed by client
                    break;
                } else {
                    let message = String::from_utf8_lossy(&buffer[..size]);
                    println!("Received message: {}", message);

                    // Deserialize the received data into a PacketMetadata struct
                    let packet_metadata: Result<PacketMetadata, bincode::Error> =
                        bincode::deserialize(&buffer[..size]);
                    println!("data:{:?}", packet_metadata);
                    match packet_metadata {
                        Ok(metadata) => {
                            // Store the metadata in the database
                            if let Err(err) = store_metadata(&db_client, &metadata).await {
                                eprintln!("Failed to insert into database: {}", err);
                            }
                        }
                        Err(err) => {
                            eprintln!("Failed to deserialize packet metadata: {}", err);
                        }
                    }

                    // Send the message back to the client
                    if let Err(err) = stream.write_all(&buffer[..size]).await {
                        eprintln!("Failed to write response: {}", err);
                    }
                }
            }
            Err(_) => {
                eprintln!("An error occurred while reading from stream");
                break;
            }
        }
    }
}
