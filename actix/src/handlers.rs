use actix_web::{web, HttpResponse};
use crate::db::get_data_from_db;

const DATABASE_URL: &str = "postgres://postgres:1234567@localhost:5432/new"; // Replace with your database URL

#[derive(Debug, serde::Serialize)]
pub struct DatabaseRecord {
    pub src_ip: Option<String>,
    pub dst_ip: Option<String>,
    pub src_port: i32,
    pub dst_port: i32,
    pub seq_number: i32,
    pub ack_number: i32,
    pub flags: i32,
    pub window_size: i32,
}

pub async fn index() -> HttpResponse {
    // Fetch data from the database
    let data = match get_data_from_db(DATABASE_URL).await {
        Ok(data) => data,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    // Render the data as JSON and return it as the API response
    HttpResponse::Ok()
        .content_type("application/json")
        .json(data)
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/json", web::get().to(index));
}
