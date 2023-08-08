use async_std::io::WriteExt;
use bincode::serialize;
use pnet::datalink::{self, Channel::Ethernet};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
// use tokio::io::AsyncWriteExt;
use crate::packet_metadata::PacketMetadata;
use async_std::net::TcpStream;
// use tokio::net::TcpStream;
use async_std::sync::Mutex; // Import Mutex from async_std
use std::sync::Arc;

pub async fn packet_capture(write_stream: Arc<Mutex<TcpStream>>) {
    // Retrieve the list of available network interfaces
    let interfaces = datalink::interfaces();

    // Find the desired network interface by name
    let interface = interfaces
        .into_iter()
        .find(|iface| iface.name == "wlp3s0")
        .expect("Wi-Fi interface not found");

    // Create a channel to receive packets on the selected Wi-Fi interface
    let mut channel = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(_, receiver)) => receiver,
        Ok(_) => panic!("Unsupported channel type"),
        Err(e) => panic!("Error creating channel: {}", e),
    };

    // Start capturing packets
    loop {
        match channel.next() {
            Ok(packet) => {
                // Access Ethernet packet metadata
                if let Some(transport_packet) = TcpPacket::new(packet) {
                    let src_port = transport_packet.get_source();
                    let dst_port = transport_packet.get_destination();
                    let seq_number = transport_packet.get_sequence();
                    let ack_number = transport_packet.get_acknowledgement();
                    let flags = transport_packet.get_flags();
                    let window_size = transport_packet.get_window();

                    // Access IP packet metadata
                    if let Some(ip_packet) = Ipv4Packet::new(packet) {
                        let src_ip = ip_packet.get_source().to_string();
                        let dst_ip = ip_packet.get_destination().to_string();

                        println!("Source Ip: {:?}", src_ip);
                        println!("Destination Ip: {:?}", dst_ip);
                        println!("Source Port: {}", src_port);
                        println!("Destination Port: {}", dst_port);
                        println!("Sequence Number: {}", seq_number);
                        println!("Acknowledgment Number: {}", ack_number);
                        println!("TCP Flags: {:?}", flags);
                        println!("TCP Window Size: {}", window_size);

                        // Create a struct with the relevant packet metadata
                        let packet_metadata = PacketMetadata {
                            src_ip: Some(src_ip),
                            dst_ip: Some(dst_ip),
                            src_port: src_port.into(),
                            dst_port: dst_port.into(),
                            seq_number,
                            ack_number,
                            flags: flags.into(),
                            window_size: window_size.into(),
                        };

                        // Serialize the metadata to bytes using bincode
                        let metadata_bytes =
                            serialize(&packet_metadata).expect("Failed to serialize metadata");

                        // Send the metadata as a message to the server
                        let mut stream = write_stream.lock().await; // Acquire the lock before writing
                        stream
                            .write_all(&metadata_bytes)
                            .await
                            .expect("Failed to send metadata");
                        stream.flush().await.expect("Failed to flush");
                        // Release the lock here
                    }
                }
            }
            Err(_) => todo!(),
        }
    }
}
