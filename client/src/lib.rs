// lib.rs
pub mod env;
pub mod io_operations;
pub mod packet_capture;
pub mod packet_metadata;

// Re-export the modules for easier access
pub use env::CLIENT_ADDRESS;
pub use io_operations::io_operations;
pub use packet_capture::packet_capture;
