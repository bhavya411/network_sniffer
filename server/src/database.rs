use crate::models::PacketStructure;
use crate::query::*;
use sqlx::{Pool, Postgres, Row};
use std::error::Error;

#[derive(Clone)]
pub struct DatabaseConnection {
    pool: Pool<Postgres>,
}

impl DatabaseConnection {
    pub(crate) async fn new(url: &str) -> DatabaseConnection {
        let pool = sqlx::postgres::PgPool::connect(url).await.unwrap();
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Error");
        DatabaseConnection { pool }
    }
    pub async fn insert_items(&self, packet: PacketStructure) -> Result<(), Box<dyn Error>> {
        println!("inside insert item");
        sqlx::query(INSERT_QUERY)
            .bind(packet.source_ip)
            .bind(packet.source_port)
            .bind(packet.destination_ip)
            .bind(packet.destination_port)
            .bind(packet.protocol)
            .bind(packet.packet_size)
            .execute(&self.pool)
            .await?;
        println!("inserted");
        Ok(())
    }

    //
    pub async fn read_items(&self) -> Result<Vec<PacketStructure>, Box<dyn Error>> {
        let select_query = sqlx::query(READ_ALL_QUERY);
        let rows = select_query.fetch_all(&self.pool).await?;
        println!("inside read data ");
        let packet_structure: Vec<PacketStructure> = rows
            .into_iter()
            .map(|row| PacketStructure {
                source_ip: row.get("source_ip"),
                source_port: row.get("source_port"),
                destination_ip: row.get("destination_ip"),
                destination_port: row.get("destination_port"),
                packet_size: row.get("packet_size"),
                protocol: row.get("protocol"),
            })
            .collect();
        Ok(packet_structure)
    }

    // pub async fn get_item_by_serial_number(&self, serial_no: i32) -> Result<Option<Item>, Box<dyn Error>> {
    //     let select_query = sqlx::query(GET_BY_ID_QUERY).bind(serial_no);
    //     let rows = select_query.fetch_optional(&self.pool).await?;
    //
    //     let items = rows.map(|row| Item {
    //         ip_source: row.get("ip_source"),
    //         source_port: row.get("source_port"),
    //         ip_destination: row.get("ip_destination"),
    //         destination_port: row.get("destination_port"),
    //         packet_size: row.get("packet_size"),
    //         protocol: row.get("protocol"),
    //         start_date_time: row.get("start_date_time"),
    //         end_date_time: row.get("end_date_time"),
    //     });
    //     Ok(items)
    // }

    pub async fn get_traffic_of_ip_source(&self, source_ip: String) -> Result<Vec<PacketStructure>, Box<dyn Error>> {
        let select_query = sqlx::query(COUNT_TRAFFIC_QUERY)
            .bind(source_ip);

        let rows = select_query.fetch_all(&self.pool).await?;

            let packet_structure :Vec<PacketStructure> = rows.into_iter().map(|row| PacketStructure {
                source_ip: row.get("source_ip"),
                source_port: row.get("source_port"),
                destination_ip: row.get("destination_ip"),
                destination_port: row.get("destination_port"),
                packet_size: row.get("packet_size"),
                protocol: row.get("protocol"),
            }).collect();
        Ok(packet_structure)
    }
}
