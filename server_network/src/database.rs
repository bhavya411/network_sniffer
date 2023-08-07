use crate::information::PacketData;
use crate::query::*;
use sqlx::{Pool, Postgres, Row};
use std::error::Error;

#[derive(Clone)]
pub struct DatabaseConnection {
    pool: Pool<Postgres>,
}

impl DatabaseConnection {
    pub async fn new(url: &str) -> DatabaseConnection {
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

    pub async fn read_items(&self, limit: i32, offset: i32) -> Result<Vec<PacketData>, Box<dyn Error>> {
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
        let select_query = sqlx::query(COUNT_TRAFFIC_QUERY).bind(ip_source).bind(limit)
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

#[cfg(test)]
mod tests {
    use super::*;

#[faux::create]
pub struct MockDatabaseConnection;

#[faux::methods]
impl MockDatabaseConnection {
    pub fn new(_: &str) -> MockDatabaseConnection {
        MockDatabaseConnection
    }
    pub fn dummy_database(&self) -> Vec<PacketData> {
        let packet_data = vec![PacketData {
            ip_source: "127.0.0.1".to_string(),
            source_port: 8080,
            ip_destination: "192.168.0.1".to_string(),
            destination_port: 80,
            packet_size: 1024,
            protocol: "TCP".to_string(),
        },PacketData {
            ip_source: "127.0.0.1".to_string(),
            source_port: 8010,
            ip_destination: "199.168.0.1".to_string(),
            destination_port: 80,
            packet_size: 724,
            protocol: "UDP".to_string(),
        },PacketData {
            ip_source: "255.0.0.1".to_string(),
            source_port: 8080,
            ip_destination: "191.168.0.1".to_string(),
            destination_port: 443,
            packet_size: 45,
            protocol: "TCP".to_string(),
        }];
        packet_data
    }
    pub fn insert_items(&self, data: PacketData) -> Result<(), Box<dyn Error>> {
        let mut items = self.dummy_database();
        items.push(data);
        Ok(())
    }

    pub fn read_items(&self, _: i32, _: i32) -> Result<Vec<PacketData>, Box<dyn Error>> {
        Ok(self.dummy_database())
    }

    pub  fn get_item_by_serial_number(&self, serial_no: i32) -> Result<PacketData, Box<dyn Error>> {
        let packet_data = self.dummy_database();
        Ok(packet_data[serial_no as usize].clone())
    }

    pub fn get_traffic_of_ip_source(&self, ip_source: String, _: i32, _: i32) -> Result<Vec<PacketData>, Box<dyn Error>> {
        let packet_data = self.dummy_database();
        let mut vector: Vec<PacketData> = vec![];
        for item in packet_data {
            if item.ip_source == ip_source {
                vector.push(item);
            }
        }
        Ok(vector)
    }

    pub fn get_network_list_by_protocol(&self, protocol: String, _: i32, _: i32) -> Result<Vec<PacketData>, Box<dyn Error>> {
        let packet_data = self.dummy_database();
        let mut vector: Vec<PacketData> = vec![];
        for item in packet_data {
            if item.protocol == protocol {
                vector.push(item);
            }
        }
        Ok(vector)
    }
}

    #[tokio::test]
    async fn test_read_items() {
        let db_url = "DATABASE_URL";
        let mock_db = MockDatabaseConnection::new(db_url);
        let limit = 3;
        let offset = 0;
        let result = mock_db.read_items(limit, offset);
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.len(), 3);
    }

    #[tokio::test]
    async fn test_insert_items() {
        let db_url = "DATABASE_URL";
        let mock_db = MockDatabaseConnection::new(db_url);
        let packet_data = PacketData {
            ip_source: "127.0.0.1".to_string(),
            source_port: 8080,
            ip_destination: "192.168.0.1".to_string(),
            destination_port: 80,
            packet_size: 1024,
            protocol: "TCP".to_string(),
        };
        let result = mock_db.insert_items(packet_data);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_items_by_serial_no() {
        let db_url = "DATABASE_URL";
        let mock_db = MockDatabaseConnection::new(db_url);
        let result = mock_db.get_item_by_serial_number(1);
        assert!(result.is_ok());
        let data = result.unwrap();
        let expected_data = PacketData {
            ip_source: "127.0.0.1".to_string(),
            source_port: 8010,
            ip_destination: "199.168.0.1".to_string(),
            destination_port: 80,
            packet_size: 724,
            protocol: "UDP".to_string(),
        };
        assert_eq!(data,expected_data);
    }

    #[tokio::test]
    async fn test_get_items_by_ip_source() {
        let db_url = "DATABASE_URL";
        let mock_db = MockDatabaseConnection::new(db_url);
        let limit = 1;
        let offset = 1;
        let result = mock_db.get_traffic_of_ip_source("127.0.0.1".to_string(),limit,
                                                      offset, );
        assert!(result.is_ok());
        let data = result.unwrap();
        let expected_data = vec![PacketData {
            ip_source: "127.0.0.1".to_string(),
            source_port: 8080,
            ip_destination: "192.168.0.1".to_string(),
            destination_port: 80,
            packet_size: 1024,
            protocol: "TCP".to_string(),
        },PacketData {
            ip_source: "127.0.0.1".to_string(),
            source_port: 8010,
            ip_destination: "199.168.0.1".to_string(),
            destination_port: 80,
            packet_size: 724,
            protocol: "UDP".to_string(),
        }];
        assert_eq!(data,expected_data);
    }

    #[tokio::test]
    async fn test_get_items_by_protocol() {
        let db_url = "DATABASE_URL";
        let mock_db = MockDatabaseConnection::new(db_url);
        let limit = 1;
        let offset = 1;
        let result = mock_db.get_network_list_by_protocol("TCP".to_string(),limit,
                                                          offset, );
        assert!(result.is_ok());
        let data = result.unwrap();
        let expected_data = vec![PacketData {
            ip_source: "127.0.0.1".to_string(),
            source_port: 8080,
            ip_destination: "192.168.0.1".to_string(),
            destination_port: 80,
            packet_size: 1024,
            protocol: "TCP".to_string(),
        },PacketData {
            ip_source: "255.0.0.1".to_string(),
            source_port: 8080,
            ip_destination: "191.168.0.1".to_string(),
            destination_port: 443,
            packet_size: 45,
            protocol: "TCP".to_string(),
        }];
        assert_eq!(data,expected_data);
    }
}


