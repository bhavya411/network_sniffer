use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PacketData {
    pub(crate) ip_source: String,
    pub(crate) source_port: i64,
    pub(crate) ip_destination: String,
    pub(crate) destination_port: i64,
    pub(crate) packet_size: i32,
    pub(crate) protocol: String,
}
