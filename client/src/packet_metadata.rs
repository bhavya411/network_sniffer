use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PacketMetadata {
    pub src_ip: Option<String>,
    pub dst_ip: Option<String>,
    pub src_port: u32,
    pub dst_port: u32,
    pub seq_number: u32,
    pub ack_number: u32,
    pub flags: u32,
    pub window_size: u32,
}
