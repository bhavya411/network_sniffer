use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]

pub struct PacketStructure {
    pub source_ip: String,
    pub source_port: i64,
    pub destination_ip: String,
    pub destination_port: i64,
    pub protocol: String,
    pub packet_size: i64,
}
