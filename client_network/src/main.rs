use lazy_static::lazy_static;
use pnet::datalink::{self, Channel::Ethernet, NetworkInterface};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ipv6::Ipv6Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
use pnet::packet::*;
use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;

lazy_static! {
    // Retrieve the list of available network interfaces
    static ref INTERFACES: Vec<NetworkInterface> = datalink::interfaces();
}

fn get_active_interface() -> &'static NetworkInterface {
    let mut active_interface = None;

    for interface in &*INTERFACES {
        if interface.is_running() && interface.is_broadcast() {
            println!("Active Interface: {}", interface.name);
            active_interface = Some(interface);
            break; // We found an active interface, no need to continue the loop
        }
    }

    let active_interface = match active_interface {
        Some(interface) => interface,
        None => {
            panic!("No active network interface found.");
        }
    };
    active_interface
}
// fn get_readable_payload(payload_bytes: &[u8]) -> String {
//
//     // Convert payload to a readable ASCII string
//     let mut payload_readable = String::new();
//     for byte in payload_bytes.iter() {
//         if is_printable(*byte) {
//             payload_readable.push(*byte as char);
//         }
//     }
//     payload_readable
// }r
fn get_json_data(
    source_ip: String,
    destination_ip: String,
    source_port: u16,
    destination_port: u16,
    packet_size: usize,
    protocol: &str,
) -> String {
    // Send the packet data as a message to the server
    let json_data = format!(
        r#"{{
        "ip_source": "{}",
        "ip_destination": "{}",
        "source_port": {},
        "destination_port": {},
        "packet_size": {},
        "protocol": "{}"
        }}"#,
        source_ip,
        destination_ip,
        source_port,
        destination_port,
        packet_size,
        protocol.to_string()
    );
    json_data
}
fn process_ipv4_tcp_packet(ethernet: &TcpPacket, ipv4: &Ipv4Packet) -> String {
    if let Some(tcp) = TcpPacket::new(ipv4.payload()) {
        let source_ip = ipv4.get_source();
        let destination_ip = ipv4.get_destination();
        let source_port = tcp.get_source();
        let destination_port = tcp.get_destination();
        let packet_size = ethernet.packet().len();
        let protocol = "TCP";

        // let payload_readable = get_readable_payload(ethernet.payload());
        get_json_data(
            source_ip.to_string(),
            destination_ip.to_string(),
            source_port,
            destination_port,
            packet_size,
            protocol,
        )
    } else {
        String::new()
    }
}

fn process_ipv4_udp_packet(ethernet: &UdpPacket, ipv4: &Ipv4Packet) -> String {
    if let Some(udp) = UdpPacket::new(ipv4.payload()) {
        let source_ip = ipv4.get_source();
        let destination_ip = ipv4.get_destination();
        let source_port = udp.get_source();
        let destination_port = udp.get_destination();
        let packet_size = ethernet.packet().len();
        let protocol = "UDP";

        //let payload_readable = get_readable_payload(ethernet.payload());
        get_json_data(
            source_ip.to_string(),
            destination_ip.to_string(),
            source_port,
            destination_port,
            packet_size,
            protocol,
        )
    } else {
        String::new()
    }
}

fn process_ipv6_tcp_packet(ethernet: &TcpPacket, ipv6: &Ipv6Packet) -> String {
    if let Some(tcp) = TcpPacket::new(ipv6.payload()) {
        let source_ip = ipv6.get_source();
        let destination_ip = ipv6.get_destination();
        let source_port = tcp.get_source();
        let destination_port = tcp.get_destination();
        let packet_size = ethernet.packet().len();
        let protocol = "TCP";

        //let payload_readable = get_readable_payload(ethernet.payload());
        get_json_data(
            source_ip.to_string(),
            destination_ip.to_string(),
            source_port,
            destination_port,
            packet_size,
            protocol,
        )
    } else {
        String::new()
    }
}

fn process_ipv6_udp_packet(ethernet: &UdpPacket, ipv6: &Ipv6Packet) -> String {
    if let Some(udp) = UdpPacket::new(ipv6.payload()) {
        let source_ip = ipv6.get_source();
        let destination_ip = ipv6.get_destination();
        let source_port = udp.get_source();
        let destination_port = udp.get_destination();
        let packet_size = ethernet.packet().len();
        let protocol = "UDP";

        //let payload_readable = get_readable_payload(ethernet.payload());
        get_json_data(
            source_ip.to_string(),
            destination_ip.to_string(),
            source_port,
            destination_port,
            packet_size,
            protocol,
        )
    } else {
        String::new()
    }
}

fn packet_capture(mut write_stream: TcpStream) {
    let mut channel = match datalink::channel(get_active_interface(), Default::default()) {
        Ok(Ethernet(_tx, rx)) => rx,
        Ok(_) => panic!("Unsupported channel type"),
        Err(e) => panic!("Error creating channel: {}", e),
    };
    // Start capturing packets
    loop {
        match channel.next() {
            Ok(packet) => {
                // Parse the Ethernet packet
                let ethernet = ethernet::EthernetPacket::new(packet).unwrap();
                // Check if the EtherType indicates IPv4
                if ethernet.get_ethertype() == ethernet::EtherTypes::Ipv4 {
                    // Extract the IPv4 packet
                    if let Some(ipv4) = Ipv4Packet::new(ethernet.payload()) {
                        // Check if the protocol is TCP or UDP
                        match ipv4.get_next_level_protocol() {
                            ip::IpNextHeaderProtocols::Tcp => {
                                if let Some(tcp) = TcpPacket::new(ipv4.payload()) {
                                    let json_data = process_ipv4_tcp_packet(&tcp, &ipv4);
                                    write_stream
                                        .write_all(json_data.as_bytes())
                                        .expect("Failed to send packet");
                                    write_stream.flush().expect("Failed to flush");
                                }
                            }
                            ip::IpNextHeaderProtocols::Udp => {
                                if let Some(udp) = UdpPacket::new(ipv4.payload()) {
                                    let json_data = process_ipv4_udp_packet(&udp, &ipv4);
                                    write_stream
                                        .write_all(json_data.as_bytes())
                                        .expect("Failed to send packet");
                                    write_stream.flush().expect("Failed to flush");
                                }
                            }
                            _ => {
                                //panic!("Other protocols encountered");
                            }
                        }
                    }
                } else if ethernet.get_ethertype() == ethernet::EtherTypes::Ipv6 {
                    // Extract the IPv4 packet
                    if let Some(ipv6) = Ipv6Packet::new(ethernet.payload()) {
                        // Check if the protocol is TCP or UDP
                        match ipv6.get_next_header() {
                            ip::IpNextHeaderProtocols::Tcp => {
                                if let Some(tcp) = TcpPacket::new(ipv6.payload()) {
                                    let json_data = process_ipv6_tcp_packet(&tcp, &ipv6);
                                    write_stream
                                        .write_all(json_data.as_bytes())
                                        .expect("Failed to send packet");
                                    write_stream.flush().expect("Failed to flush");
                                }
                            }
                            ip::IpNextHeaderProtocols::Udp => {
                                if let Some(udp) = UdpPacket::new(ipv6.payload()) {
                                    let json_data = process_ipv6_udp_packet(&udp, &ipv6);
                                    write_stream
                                        .write_all(json_data.as_bytes())
                                        .expect("Failed to send packet");
                                    write_stream.flush().expect("Failed to flush");
                                }
                            }
                            _ => {
                                //panic!("Other protocols encountered");
                            }
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error receiving packet: {}", e);
            }
        }
    }
}

fn main() {
    let mut stream = TcpStream::connect("localhost:8081").expect("Failed to connect to server");
    println!("Connected to server");

    let write_stream = stream.try_clone().expect("Failed to clone stream");

    // Start packet capture in a separate thread
    std::thread::spawn(move || {
        packet_capture(write_stream);
    });

    loop {
        let mut message = String::new();
        io::stdin()
            .read_line(&mut message)
            .expect("Failed to read line");

        stream.write_all(message.as_bytes()).unwrap();
        stream.flush().unwrap();

        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(size) => {
                let echo = String::from_utf8_lossy(&buffer[..size]);
                println!("Received echo: {}", echo);
            }
            Err(e) => {
                eprintln!("An error occurred while reading from server: {}", e);
                break;
            }
        }
    }
}
