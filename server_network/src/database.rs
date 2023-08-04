use crate::information::PacketData;
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
            .expect("error");
        DatabaseConnection { pool }
    }
    pub async fn insert_items(&self, item: PacketData) -> Result<(), Box<dyn Error>> {
        sqlx::query(INSERT_QUERY)
            .bind(item.ip_source)
            .bind(item.source_port)
            .bind(item.ip_destination)
            .bind(item.destination_port)
            .bind(item.packet_size)
            .bind(item.protocol)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn read_items(&self,limit: i32,offset: i32) -> Result<Vec<PacketData>, Box<dyn Error>> {
        let select_query = sqlx::query(READ_ALL_QUERY).bind(limit).bind(offset);
        let rows = select_query.fetch_all(&self.pool).await?;

        let items: Vec<PacketData> = rows
            .into_iter()
            .map(|row| PacketData {
                ip_source: row.get("ip_source"),
                source_port: row.get("source_port"),
                ip_destination: row.get("ip_destination"),
                destination_port: row.get("destination_port"),
                packet_size: row.get("packet_size"),
                protocol: row.get("protocol"),
            })
            .collect();
        Ok(items)
    }
    pub async fn get_item_by_serial_number(&self, serial_no: i32) -> Result<Option<PacketData>, Box<dyn Error>> {
        let select_query = sqlx::query(GET_BY_ID_QUERY).bind(serial_no);
        let rows = select_query.fetch_optional(&self.pool).await?;

        let items = rows.map(|row| PacketData {
            ip_source: row.get("ip_source"),
            source_port: row.get("source_port"),
            ip_destination: row.get("ip_destination"),
            destination_port: row.get("destination_port"),
            packet_size: row.get("packet_size"),
            protocol: row.get("protocol"),
        });
         Ok(items)
     }
    pub async fn get_traffic_of_ip_source(&self, ip_source: String, limit: i32, offset: i32) -> Result<Vec<PacketData>, Box<dyn Error>> {
        let select_query = sqlx::query(COUNT_TRAFFIC_QUERY).bind(ip_source) .bind(limit)
            .bind(offset);
        let rows = select_query.fetch_all(&self.pool).await?;

        let items: Vec<PacketData> = rows
            .into_iter()
            .map(|row| PacketData {
                ip_source: row.get("ip_source"),
                source_port: row.get("source_port"),
                ip_destination: row.get("ip_destination"),
                destination_port: row.get("destination_port"),
                packet_size: row.get("packet_size"),
                protocol: row.get("protocol"),
            })
            .collect();
        Ok(items)
    }
    pub async fn get_network_list_by_protocol(&self, protocol: String, limit: i32, offset: i32) -> Result<Vec<PacketData>, Box<dyn Error>> {
        let select_query = sqlx::query(FILTER_BY_PROTOCOL)
            .bind(protocol).bind(limit).bind(offset);
        let rows = select_query.fetch_all(&self.pool).await?;

        let items: Vec<PacketData> = rows
            .into_iter()
            .map(|row| PacketData {
                ip_source: row.get("ip_source"),
                source_port: row.get("source_port"),
                ip_destination: row.get("ip_destination"),
                destination_port: row.get("destination_port"),
                packet_size: row.get("packet_size"),
                protocol: row.get("protocol"),
            })
            .collect();
        Ok(items)
    }
}
