use tokio_postgres::NoTls;
use crate::handlers::DatabaseRecord;

pub async fn get_data_from_db(database_url: &str) -> Result<Vec<DatabaseRecord>, actix_web::Error> {
    let (client, connection) = tokio_postgres::connect(database_url, NoTls)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    tokio::spawn(connection);

    let rows = client
        .query("SELECT * FROM sniffer", &[])
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let mut result = Vec::new();

    for row in rows {
        let src_ip: Option<String> = row.get("src_ip");
        let dst_ip: Option<String> = row.get("dst_ip");
        let src_port: i32 = row.get("src_port");
        let dst_port: i32 = row.get("dst_port");
        let seq_number: i32 = row.get("seq_number");
        let ack_number: i32 = row.get("ack_number");
        let flags: i32 = row.get("flags");
        let window_size: i32 = row.get("window_size");

        result.push(DatabaseRecord {
            src_ip,
            dst_ip,
            src_port,
            dst_port,
            seq_number,
            ack_number,
            flags,
            window_size,
        });
    }

    Ok(result)
}
