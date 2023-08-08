use async_std::io::ReadExt; // Import ReadExt from async_std
use async_std::io::WriteExt; // Import WriteExt from async_std
use async_std::net::TcpStream;
use async_std::sync::Mutex; // Import Mutex from async_std
use std::sync::Arc;
use tokio::runtime::Runtime;
// use tokio::net::TcpStream;

pub async fn io_operations(
    rt: &Runtime,
    stream_clone: &Arc<Mutex<TcpStream>>,
    input: &str,
) -> bool {

    let mut stream = stream_clone.lock().await; // Acquire the lock before writing
    stream.write_all(input.as_bytes()).await.expect("Failed to send metadata");
    stream.flush().await.expect("Failed to flush");

    // Read response from the server
    let mut buffer = [0; 1024];
    let size = stream.read(&mut buffer).await.expect("Failed to read from server");
    if size == 0 {
        return true; // Signal the main loop to break
    }

    let response = String::from_utf8_lossy(&buffer[..size]);
    println!("Received response: {}", response);

    false 
}
