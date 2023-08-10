// database.rs
use tokio_postgres::Client;
use tokio::sync::Mutex;
use std::sync::Arc;
use crate::PacketMetadata;
use crate::env::DATABASE_URL;

pub async fn connect_to_database() -> Result<Client, tokio_postgres::Error> {
    let (client, connection) = tokio_postgres::connect(DATABASE_URL, tokio_postgres::NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Database connection error: {}", e);
        }
    });

    Ok(client)
}

pub async fn store_metadata(db_client: &Arc<Mutex<Client>>, metadata: &PacketMetadata) -> Result<(), tokio_postgres::Error> {
    let db_client_lock = db_client.lock().await;
    db_client_lock.query(
        "INSERT INTO sniffer (src_ip, dst_ip, src_port, dst_port, seq_number, ack_number, flags, window_size) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        &[
            &metadata.src_ip,
            &metadata.dst_ip,
            &metadata.src_port,
            &metadata.dst_port,
            &metadata.seq_number,
            &metadata.ack_number,
            &metadata.flags,
            &metadata.window_size,
        ],
    ).await?;

    Ok(())
}
