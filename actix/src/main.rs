mod handlers;
mod db;
mod static_files;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(handlers::configure_routes)
            .configure(static_files::configure_static_files)
    })
    .bind("127.0.0.1:2000")?
    .run()
    .await
}
