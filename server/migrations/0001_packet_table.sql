create table packet_information(
serial_no serial primary key not null,
source_ip text not null ,
source_port bigint not null,
destination_ip text not null,
destination_port bigint not null,
protocol text not null,
packet_size bigint not null

);