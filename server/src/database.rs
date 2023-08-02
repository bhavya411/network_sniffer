use crate::models::{PacketStructure, PaginateStructure};
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

    pub async fn read_items(
        &self,
        paginate_structure: PaginateStructure,
    ) -> Result<Vec<PacketStructure>, Box<dyn Error>> {
        let offset = (paginate_structure.page_number - 1) * (paginate_structure.page_length);
        println!("offset  -> {}", offset);
        let select_query = sqlx::query(QUERY_OF_READ_BY_PAGES)
            .bind(paginate_structure.page_length)
            .bind(offset);

        let rows = select_query.fetch_all(&self.pool).await?;

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

    pub async fn read_all_items(&self) -> Result<Vec<PacketStructure>, Box<dyn Error>> {
        let select_query = sqlx::query(QUERY_OF_READ_ALL);
        let rows = select_query.fetch_all(&self.pool).await?;

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

    pub async fn get_traffic_of_ip_source(
        &self,
        source_ip: String,
        paginate_structure: PaginateStructure,
    ) -> Result<Vec<PacketStructure>, Box<dyn Error>> {
        let offset = (paginate_structure.page_number - 1) * (paginate_structure.page_length);

        let select_query = sqlx::query(QUERY_OF_GET_TRAFFIC__BY_IP_SOURCE)
            .bind(source_ip)
            .bind(paginate_structure.page_length)
            .bind(offset);

        let rows = select_query.fetch_all(&self.pool).await?;

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

    pub async fn get_traffic_of_source_port(
        &self,
        source_port: i64,
        paginate_structure: PaginateStructure,
    ) -> Result<Vec<PacketStructure>, Box<dyn Error>> {
        let offset = (paginate_structure.page_number - 1) * (paginate_structure.page_length);

        let select_query = sqlx::query(QUERY_OF_GET_TRAFFIC_BY_SOURCE_PORT)
            .bind(source_port)
            .bind(paginate_structure.page_length)
            .bind(offset);
        let rows = select_query.fetch_all(&self.pool).await?;
        println!("inside source port");
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

    pub async fn get_traffic_of_protocol(
        &self,
        protocol: String,
        paginate_structure: PaginateStructure,
    ) -> Result<Vec<PacketStructure>, Box<dyn Error>> {
        let offset = (paginate_structure.page_number - 1) * (paginate_structure.page_length);

        let select_query = sqlx::query(QUERY_OF_GET_TRAFFIC_BY_PROTOCOL)
            .bind(protocol)
            .bind(paginate_structure.page_length)
            .bind(offset);

        let rows = select_query.fetch_all(&self.pool).await?;
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
}
