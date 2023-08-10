// lib.rs
pub mod packet_metadata;
pub mod database;
pub mod tcp_server;
pub mod env;

// Re-export the functions for easier access
pub use database::connect_to_database;
pub use database::store_metadata;
pub use packet_metadata::PacketMetadata;
pub use tcp_server::handle_client;
pub use env::{DATABASE_URL, SERVER_ADDRESS};
