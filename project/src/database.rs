use crate::env::DATABASE_URL;
use crate::PacketMetadata;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Client; // Import the constant DATABASE_URL from env.rs

pub async fn connect_to_database() -> Result<Client, tokio_postgres::Error> {
    let (client, connection) = tokio_postgres::connect(DATABASE_URL, tokio_postgres::NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Database connection error: {}", e);
        }
    });

    Ok(client)
}

pub async fn store_metadata(
    db_client: &Arc<Mutex<Client>>,
    metadata: &PacketMetadata,
) -> Result<(), tokio_postgres::Error> {
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

// async fn read(pool: &sqlx::PgPool, page: i32, page_size: i32) {
//     let offset = (page - 1) * page_size;
//     let query = format!(
//         "SELECT * FROM student ORDER BY roll ASC LIMIT {} OFFSET {}",
//         page_size, offset
//     );

//     let data = sqlx::query_as::<_, Student>(&query)
//         .fetch_all(pool)
//         .await
//         .expect("Error fetching data from the database.");

//     for student in data {
//         println!("Name: {}  \t\t City: {}", student.name, student.city);
//     }
// }
