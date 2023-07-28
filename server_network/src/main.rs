mod database;
mod http_methods_implementation;
mod information;
mod query;

use crate::database::DatabaseConnection;
use dotenv::dotenv;
use std::env;
use warp::Filter;
use http_methods_implementation::*;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use crate::information::PacketData;
use async_std::task;
use crossbeam::scope;


async fn handle_client(mut stream: TcpStream, db: &DatabaseConnection) {
    let mut buffer = [0; 1024];
    while match stream.read(&mut buffer) {
        Ok(size) => {
            if size == 0 {
                false
            } else {
                let packet_data = &buffer[..size];
                let packet_data = String::from_utf8_lossy(packet_data);

                // Parse the received JSON data into the PacketData struct
                let parsed_packet: Result<PacketData, _> = serde_json::from_str(&packet_data);

                match parsed_packet {
                    Ok(packet) => {
                        // Do whatever you need to do with the parsed data
                        let packet_clone = packet.clone();
                        db.insert_items(packet_clone).await
                            .expect("Failed to insert data");
                        println!("Received packet data: {:?}", packet);

                        // Echo the received data back to the client
                        if let Err(e) = stream.write_all(packet_data.as_bytes()) {
                            eprintln!("Error writing to socket: {}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error parsing packet data: {}", e);
                    }
                }

                true
            }
        }
        Err(_) => {
            eprintln!("An error occurred while reading from stream");
            false
        }
    } {}
}

async fn server_network(db_clone: DatabaseConnection) {
    let listener = TcpListener::bind("localhost:8081").expect("Failed to bind address");
    println!("Server listening on localhost:8081");

    scope(|scope| {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    // Spawn a new async task using async-std to handle each client connection
                    scope.spawn(|_| {
                        task::block_on(handle_client(stream, &db_clone));
                    });
                }
                Err(e) => {
                    eprintln!("Error accepting client connection: {}", e);
                }
            }
        }
    })
        .unwrap(); // Ensure all threads are joined before the function exits
}

async fn api_calls(db_connection: DatabaseConnection) {

    let db_filter = warp::any().map(move || db_connection.clone());
    let add_items = warp::post()
        .and(warp::path("api"))
        .and(warp::path("network"))
        .and(warp::path::end())
        .and(json_body())
        .and(db_filter.clone())
        .and_then(post_networks_list);

    let get_items = warp::get()
        .and(warp::path("api"))
        .and(warp::path("network"))
        .and(warp::path::end())
        .and(db_filter.clone())
        .and_then(get_all_networks_list);

    let get_items_by_serial_number = warp::get()
        .and(warp::path("api"))
        .and(warp::path!("network" / "get_ip" / i32))
        .and(warp::path::end())
        .and(db_filter.clone())
        .and_then(get_network_list_by_serial_number);

    let get_traffic_by_ip_source = warp::get()
        .and(warp::path("api"))
        .and(warp::path!("network" / "get_ip" / String))
        .and(warp::path::end())
        .and(db_filter.clone())
        .and_then(get_traffic);

    let routes = add_items.or(get_items).or(get_items_by_serial_number).or(get_traffic_by_ip_source);
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await
}
#[tokio::main]
async fn main() {

    dotenv().ok(); // This line loads the environment variable
    let db_url = env::var("DATABASE_URL").expect("Database not found");
    let db_connection = DatabaseConnection::new(&db_url).await;
    let db_clone = db_connection.clone();

    tokio::spawn(async move {
        server_network(db_clone).await;
    });

    // Start the API server in the main thread
    api_calls(db_connection).await;
}
