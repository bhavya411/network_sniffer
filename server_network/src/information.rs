use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PacketData {
    pub ip_source: String,
    pub source_port: i64,
    pub ip_destination: String,
    pub destination_port: i64,
    pub packet_size: i32,
    pub protocol: String,
}
#[derive(Deserialize,Serialize)]
pub struct PaginationParams {
    pub page: usize,
    pub per_page: usize,
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub data: Vec<T>,
    pub pagination: PaginationInfoResponse,
}

#[derive(Serialize)]
pub struct PaginationInfoResponse {
    pub total_items: usize,
    pub per_page: usize,
    pub current_page: usize,
    pub total_pages: usize,
}

