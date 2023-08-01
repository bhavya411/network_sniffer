create table network_information (
   serial_no serial primary key not null,
   ip_source text not null ,
   source_port bigint not null,
   ip_destination text not null,
   destination_port bigint not null,
   packet_size integer not null,
   protocol text not null
);
