use colored::*;
use pnet::datalink::{self, Channel::Ethernet, NetworkInterface};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::Packet;
use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;

fn formatted_data(
    source_ip: String,
    source_port: i32,
    destination_ip: String,
    destination_port: i32,
    protocol: String,
    packet_size: i32,
) -> String {
    let formatted_data = format!(
        r#"{{
        "source_ip": "{}",
        "source_port": {},
        "destination_ip":"{}",
        "destination_port":{},
        "protocol":"{}",
        "packet_size":{}
        }}"#,
        source_ip, source_port, destination_ip, destination_port, protocol, packet_size
    );

    formatted_data
}

fn get_active_interface() -> NetworkInterface {
    // Retrieve the list of available network interfaces
    let mut interfaces = datalink::interfaces();

    let interface_name = interfaces
        .iter()
        .find(|e| e.is_up() && !e.is_loopback() && !e.ips.is_empty());

    let interface = interface_name.expect("Error").clone();

    interface
}

fn packet_capture(mut write_stream: TcpStream) {
    let title = " _  _       _                       _                    _   __   __           \n\
        | \\| | ___ | |_  _ __ __  ___  _ _ | |__       ___ _ _  (_) / _| / _| ___  _ _ \n\
        | .  |/ -_)|  _| \\ V  V // _ \\| '_|| / /      (_-/| ' \\ | ||  _||  _|/ -_)| '_|\n\
        |_|\\_|\\___| \\__|  \\_/\\_/ \\___/|_|  |_\\_\\     /__/|_||_||_||_|  |_|  \\___||_|  \n";
    println!("{}", title.purple());

    // Create a channel to receive packets on the selected Wi-Fi interface
    let mut channel = match datalink::channel(&get_active_interface(), Default::default()) {
        Ok(Ethernet(_, receiver)) => receiver,
        Ok(_) => panic!("Unsupported channel type"),
        Err(e) => panic!("Error creating channel: {}", e),
    };

    println!(" Start capturing packets");

    loop {
        match channel.next() {
            Ok(packet) => {
                // Convert the packet to a string representation
                let packet_type = pnet::packet::ethernet::EthernetPacket::new(packet).unwrap();

                if packet_type.get_ethertype() == pnet::packet::ethernet::EtherTypes::Ipv4 {
                    println!("\nIPV4 DATA");

                    let ipv4_packet = Ipv4Packet::new(packet_type.payload()).expect("error");

                    match ipv4_packet.get_next_level_protocol() {
                        pnet::packet::ip::IpNextHeaderProtocols::Tcp => {
                            let tcp_packet =
                                pnet::packet::tcp::TcpPacket::new(packet).expect("error");

                            let destination_port = tcp_packet.get_destination();
                            let source_port = tcp_packet.get_source();
                            let source_ip = ipv4_packet.get_source();
                            let destination_ip = ipv4_packet.get_destination();
                            let packet_size = packet_type.payload().len();
                            let payload = ipv4_packet.payload();
                            let check_sum = ipv4_packet.get_checksum();

                            let packet_string = formatted_data(
                                source_ip.to_string(),
                                source_port as i32,
                                destination_ip.to_string(),
                                destination_port as i32,
                                "Tcp".parse().unwrap(),
                                packet_size as i32,
                            );

                            write_stream
                                .write_all(packet_string.as_bytes())
                                .expect("Failed to se1nd packet");
                            write_stream.flush().expect("Failed to flush");

                            print!("source port -> {} ", source_port);
                            print!("destination port -> {} ", destination_port);
                            print!("SOURCE IP_ADDRESS -> {} ", source_ip);
                            println!("DESTINATION IP_ADDRESS -> {} ", destination_ip);
                            print!("PACKET_SIZE -> {} ", packet_size);
                            print!("GET_CHECKSUM -> {:?} ", check_sum);
                            print!("payload {:?}", payload);
                        }

                        pnet::packet::ip::IpNextHeaderProtocols::Udp => {
                            let udp_packet =
                                pnet::packet::udp::UdpPacket::new(packet).expect("error");

                            let destination_port = udp_packet.get_destination();
                            let source_port = udp_packet.get_source();
                            let source_ip = ipv4_packet.get_source();
                            let destination_ip = ipv4_packet.get_destination();
                            let packet_size = packet_type.payload().len();
                            let payload = ipv4_packet.payload();
                            let check_sum = ipv4_packet.get_checksum();
                            let packet_string = formatted_data(
                                source_ip.to_string(),
                                source_port as i32,
                                destination_ip.to_string(),
                                destination_port as i32,
                                "Udp".parse().unwrap(),
                                packet_size as i32,
                            );
                            write_stream
                                .write_all(packet_string.as_bytes())
                                .expect("Failed to send packet");
                            write_stream.flush().expect("Failed to flush");
                            print!("source port -> {} ", source_port);
                            print!("destination port -> {} ", destination_port);
                            print!("SOURCE IP_ADDRESS -> {} ", source_ip);
                            println!("DESTINATION IP_ADDRESS -> {} ", destination_ip);
                            print!("PACKET_SIZE -> {} ", packet_size);
                            print!("GET_CHECKSUM -> {:?} ", check_sum);
                            print!("payload {:?}", payload);
                        }

                        _ => {}
                    }
                } else if packet_type.get_ethertype() == pnet::packet::ethernet::EtherTypes::Ipv6 {
                    println!("\nIPV6 DATA");

                    let ipv6_packet =
                        pnet::packet::ipv6::Ipv6Packet::new(packet_type.payload()).expect("error");

                    match ipv6_packet.get_next_header() {
                        pnet::packet::ip::IpNextHeaderProtocols::Tcp => {
                            let tcp_packet =
                                pnet::packet::tcp::TcpPacket::new(packet).expect("error");
                            let destination_port = tcp_packet.get_destination();
                            let source_port = tcp_packet.get_source();
                            let source_ip = ipv6_packet.get_source();
                            let destination_ip = ipv6_packet.get_destination();
                            let packet_size = packet_type.payload().len();
                            let payload = ipv6_packet.payload();
                            let packet_string = formatted_data(
                                source_ip.to_string(),
                                source_port as i32,
                                destination_ip.to_string(),
                                destination_port as i32,
                                "Tcp".parse().unwrap(),
                                packet_size as i32,
                            );
                            write_stream
                                .write_all(packet_string.as_bytes())
                                .expect("Failed to send packet");
                            write_stream.flush().expect("Failed to flush");
                            print!("source port -> {} ", source_port);
                            print!("destination port -> {} ", destination_port);
                            println!("SOURCE IP_ADDRESS -> {} ", source_ip);
                            print!("DESTINATION IP_ADDRESS -> {} ", destination_ip);
                            print!("PACKET_SIZE -> {} ", packet_size);
                            print!("payload {:?}", payload);
                        }

                        pnet::packet::ip::IpNextHeaderProtocols::Udp => {
                            let udp_packet =
                                pnet::packet::udp::UdpPacket::new(packet).expect("error");
                            let destination_port = udp_packet.get_destination();
                            let source_port = udp_packet.get_source();
                            let source_ip = ipv6_packet.get_source();
                            let destination_ip = ipv6_packet.get_destination();
                            let packet_size = packet_type.payload().len();
                            let payload = ipv6_packet.payload();
                            let packet_string = formatted_data(
                                source_ip.to_string(),
                                source_port as i32,
                                destination_ip.to_string(),
                                destination_port as i32,
                                "Udp".parse().unwrap(),
                                packet_size as i32,
                            );

                            write_stream
                                .write_all(packet_string.as_bytes())
                                .expect("Failed to send packet");
                            write_stream.flush().expect("Failed to flush");
                            print!("source port -> {} ", source_port);
                            print!("destination port -> {} ", destination_port);
                            println!("SOURCE IP_ADDRESS -> {} ", source_ip);
                            print!("DESTINATION IP_ADDRESS -> {} ", destination_ip);
                            print!("PACKET_SIZE -> {} ", packet_size);
                            print!("payload {:?}", payload);
                        }
                        _ => {}
                    }
                }
            }

            _ => {}
        }
    }
}

fn main() {
    let mut stream = TcpStream::connect("localhost:8080").expect("Failed to connect to server");
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
