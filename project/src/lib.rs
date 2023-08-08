pub mod database;
pub mod env;
pub mod packet_metadata;
pub mod tcp_server;

// Re-export the functions for easier access
pub use database::connect_to_database;
pub use database::store_metadata;
pub use env::{DATABASE_URL, SERVER_ADDRESS};
pub use packet_metadata::PacketMetadata;
pub use tcp_server::handle_client;
