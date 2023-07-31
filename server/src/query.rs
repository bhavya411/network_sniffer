pub static INSERT_QUERY: &str = "insert into packet_information (source_ip,source_port,destination_ip,destination_port,protocol,packet_size)\
 values ($1,$2,$3,$4,$5,$6)";
pub static READ_ALL_QUERY: &str = "select * from packet_information";
// pub static GET_BY_ID_QUERY: &str = "select * from network_information where serial_no = $1";
pub static COUNT_TRAFFIC_QUERY: &str = "select * from packet_information where source_ip = $1";
