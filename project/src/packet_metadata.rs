use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PacketMetadata {
    pub src_ip: Option<String>,
    pub dst_ip: Option<String>,
    pub src_port: i32,
    pub dst_port: i32,
    pub seq_number: i32,
    pub ack_number: i32,
    pub flags: i32,
    pub window_size: i32,
}