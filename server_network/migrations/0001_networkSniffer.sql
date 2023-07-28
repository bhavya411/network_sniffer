create table network_information (
   serial_no serial primary key not null,
   ip_source text not null ,
   source_port bigint not null,
   ip_destination text not null,
   destination_port bigint not null,
   packet_size integer not null,
   protocol text not null
);

--"ip_source": "172.20.10.14",
--    "source_port": 54990,
--    "ip_destination": "172.20.10.1",
--    "destination_port": 53,
--    "packet_size": 88,
--    "protocol": "ethernet-ipv4-UDP-DNS",
--    "start_date_time": "24/07/2023 16:13",
--    "end_date_time": "24/07/2023 16:13"

--"ip_source": "172.20.10.1",
--        "source_port": 53,
--        "ip_destination": "172.20.10.14",
--        "destination_port": 63851,
--        "packet_size": 162,
--        "protocol": "ethernet-ipv4-UDP-DNS",
--        "start_date_time": "24/07/2023 16:15",
--        "end_date_time": "24/07/2023 16:15"

--"ip_source": "172.20.10.14",
--    "source_port": 59126,
--    "ip_destination": "172.20.10.1",
--    "destination_port": 53,
--    "packet_size": 80,
--    "protocol": "ethernet-ipv4-UDP-DNS",
--    "start_date_time": "24/07/2023 16:16",
--    "end_date_time": "24/07/2023 16:16"

-- "ip_source": "172.20.10.1",
--    "source_port": 53,
--    "ip_destination": "172.20.10.14",
--    "destination_port": 51512,
--    "packet_size": 294,
--    "protocol": "ethernet-ipv4-UDP-DNS",
--    "start_date_time": "24/07/2023 16:17",
--    "end_date_time": "24/07/2023 16:17"
