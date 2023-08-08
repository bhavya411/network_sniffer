use async_std::net::TcpStream;
use std::io;
use async_std::sync::Arc;
use tokio::runtime::Runtime;
use async_std::sync::Mutex;

mod io_operations;
mod packet_capture;
mod packet_metadata;

use io_operations::io_operations;
use packet_capture::packet_capture;

#[tokio::main]
async fn main() {
    // Establish the tokio runtime
    let rt = Runtime::new().unwrap();

    // Connect to the server
    let stream = TcpStream::connect("localhost:8000")
        .await
        .expect("Failed to connect to server");
    println!("Connected to server");

    // Wrap the TCP stream in an Arc and Mutex for shared access
    let shared_stream = Arc::new(Mutex::new(stream));

    // Clone the stream for packet capture
    let write_stream = shared_stream.clone();

    // Start packet capture in a separate tokio task
    rt.spawn(async move {
        packet_capture(write_stream).await;
    });

    // Main loop for reading input and interacting with the server
    let mut stream_clone = shared_stream.clone();
    loop {
        let mut input = String::new();
        println!("Enter metadata (KEY:VALUE):");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if io_operations(&rt, &mut stream_clone, &input).await {
            break; // Exit the main loop if the I/O operations signal to break
        }
    }
}
