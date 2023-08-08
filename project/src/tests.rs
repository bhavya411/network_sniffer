// tests.rs
use tokio::sync::Mutex;
use std::sync::Arc;
use crate::handle_client;

mod tcp_server_test {
    use super::*;
    use tokio::net::TcpListener;
    use tokio::net::TcpStream;
    use tokio::io::AsyncWriteExt;

    #[tokio::test]
    async fn test_handle_client() {
        // Set up a mock TcpListener
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("Failed to bind to address");
        let server_addr = listener.local_addr().unwrap();

        // Set up a mock TcpStream
        let client_stream = TcpStream::connect(&server_addr).await.expect("Failed to connect to server");

        // Set up the shared database client state
        let db_client = Arc::new(Mutex::new("".to_string()));

        // Call the handle_client function with the mock TcpStream and db_client
        let handle_client_task = handle_client(client_stream, Arc::clone(&db_client));

        // Wait for the handle_client task to complete
        let result = handle_client_task.await;

        // Assert that the handle_client function completed successfully
        assert!(result.is_ok());

        // Lock the database client to read the updated message
        let db_client_guard = db_client.lock().await;

        // Assert that the message in the database client was updated as expected
        assert_eq!(*db_client_guard, "Hello from handle_client!");
    }
}

