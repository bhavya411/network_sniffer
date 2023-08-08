// lib.rs
pub mod handlers;
pub mod static_files;
pub mod env;
pub mod db;

// Re-export the modules for easier access
pub use db::get_data_from_db;
pub use static_files::{webapp, configure_static_files};
pub use handlers::{index, configure_routes};

pub use env::DATABASE_URL;
