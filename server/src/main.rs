mod database;
mod http_methods_implementation;
mod query;
mod models;

use crate::database::*;
use crate::http_methods_implementation::*;
use crate::models::PacketStructure;
use async_std;
use async_std::task;
use crossbeam::epoch::Pointable;
use crossbeam::scope;
use dotenv::dotenv;
use mockall::*;
use serde::{Deserialize, Serialize};
use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::ops::Deref;
use warp::Filter;

async fn handle_client(mut stream: TcpStream, db_connection: &DatabaseConnection) {
    let mut buffer = [0; 1024];
    while match stream.read(&mut buffer) {
        Ok(size) => {
            if size == 0 {
                false
            } else {
                let packet_data = &buffer[..size];
                let packet_data = String::from_utf8_lossy(packet_data);
                // Parse the received JSON data into the PacketData struct
                let parsed_packet: Result<PacketStructure, _> = serde_json::from_str(&packet_data);
                match parsed_packet {
                    Ok(packet) => {
                        db_connection
                            .insert_items(packet)
                            .await
                            .expect("TODO: panic message");
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

async fn api_methods(db_connection: DatabaseConnection) {
    let db_filter = warp::any().map(move || db_connection.clone());

    let get_data_by_page = warp::get()
        .and(warp::path("api"))
        .and(warp::path("network"))
        .and(warp::path("page"))
        .and(warp::path::end())
        .and(db_filter.clone())
        .and(paginate())
        .and_then(get_networks_list_in_pages);

    let get_all_data = warp::get()
        .and(warp::path("api"))
        .and(warp::path("network"))
        .and(warp::path::end())
        .and(db_filter.clone())
        .and_then(get_all_networks_list);

    let get_traffic_by_source_ip = warp::get()
        .and(warp::path("api"))
        .and(warp::path!("get_source_ip" / String))
        .and(warp::path::end())
        .and(db_filter.clone())
        .and(paginate())
        .and_then(get_traffic_source_ip);


    let get_traffic_by_source_port = warp::get()
        .and(warp::path("api"))
        .and(warp::path!("get_source_port" / i64))
        .and(warp::path::end())
        .and(db_filter.clone())
        .and(paginate())
        .and_then(get_traffic_source_port);

    let get_traffic_by_protocol = warp::get()
        .and(warp::path("api"))
        .and(warp::path!("get_protocol" / String))
        .and(warp::path::end())
        .and(db_filter.clone())
        .and(paginate())
        .and_then(get_traffic_protocol);

    let routes = get_all_data
        .or(get_traffic_by_source_ip)
        .or(get_traffic_by_source_port)
        .or(get_traffic_by_protocol)
        .or(get_data_by_page);

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
async fn stream_connection(db_connection: DatabaseConnection) {
    let listener = TcpListener::bind("localhost:8080").expect("Failed to bind address");
    scope(|scope| {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    scope.spawn(|_| {
                        task::block_on(handle_client(stream, &db_connection));
                    });
                }
                Err(e) => {
                    eprintln!("An error occurred while accepting a connection: {}", e);
                }
            }
        }
    })
    .unwrap();
}

#[tokio::main]
async fn main() {
    dotenv().ok(); // This line loads the environment variable
    let db_url = env::var("DATABASE_URL").expect("Database not found");
    let db_connection = DatabaseConnection::new(&db_url).await;
    let db_clone2 = db_connection.clone();

    println!("Server listening on localhost:8080");
    tokio::spawn(async move {
        stream_connection(db_clone2).await;
    });

    api_methods(db_connection).await;
}
