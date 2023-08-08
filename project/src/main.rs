mod database;
mod env;
mod packet_metadata;
mod tcp_server;
// mod tests;

use crate::database::connect_to_database;
use crate::packet_metadata::PacketMetadata;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

use crate::env::SERVER_ADDRESS;
use crate::tcp_server::handle_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_client = connect_to_database().await?; // Remove DATABASE_URL argument here

    let db_client = Arc::new(Mutex::new(db_client));

    let listener = TcpListener::bind(SERVER_ADDRESS).await?;
    println!("Server listening on {}", SERVER_ADDRESS);

    while let Ok((stream, _)) = listener.accept().await {
        let db_client_clone = Arc::clone(&db_client);
        tokio::spawn(handle_client(stream, db_client_clone));
    }

    Ok(())

    // let mut page = "".to_string();
    // let mut page_size = "".to_string();
    // println!("Enter the Page number you want to visit: ");
    // io::stdin().read_line(&mut page).expect("Failed to input");
    // println!("Enter the Page size : ");
    // io::stdin()
    //     .read_line(&mut page_size)
    //     .expect("Failed to input");
    // let page = page.trim().parse().expect("Failed to convert");
    // let page_size: i32 = page_size.trim().parse().expect("Filed to convert");
    // read(&pool, page, page_size).await;
}

