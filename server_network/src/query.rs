pub static INSERT_QUERY: &str = "insert into network_information (ip_source,source_port,ip_destination,destination_port,packet_size,protocol)\
 values ($1,$2,$3,$4,$5,$6)";
pub static READ_ALL_QUERY: &str = "select * from network_information limit $1 offset $2";
pub static GET_BY_ID_QUERY: &str = "select * from network_information where serial_no = $1";
pub static COUNT_TRAFFIC_QUERY: &str = "select * from network_information where ip_source = $1 order by serial_no limit $2 offset $3";
pub static FILTER_BY_PROTOCOL: &str = "select * from network_information where protocol = $1 order by serial_no limit $2 offset $3 ";
