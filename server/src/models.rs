use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PacketStructure {
    pub source_ip: String,
    pub source_port: i64,
    pub destination_ip: String,
    pub destination_port: i64,
    pub protocol: String,
    pub packet_size: i64,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct PaginateStructure {
    pub page_number: i32,
    pub page_length: i32,
}

#[derive(Debug)]
pub struct NoDataFound;
impl warp::reject::Reject for NoDataFound {}

#[derive(Debug)]
pub struct DatabaseError(pub String);
impl warp::reject::Reject for DatabaseError {}
