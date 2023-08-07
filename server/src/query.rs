pub static INSERT_QUERY: &str = "insert into packet_information (source_ip,source_port,destination_ip,destination_port,protocol,packet_size)\
 values ($1,$2,$3,$4,$5,$6)";
pub static QUERY_OF_READ_BY_PAGES: &str =
    "select * from packet_information order by serial_no limit $1 offset $2";

pub static QUERY_OF_READ_ALL: &str = "select * from packet_information ";

// pub static GET_BY_ID_QUERY: &str = "select * from network_information where serial_no = $1";
pub static QUERY_OF_GET_TRAFFIC__BY_IP_SOURCE: &str =
    "select * from packet_information where source_ip = $1  order by serial_no limit $2 offset $3";

pub static QUERY_OF_GET_TRAFFIC_BY_SOURCE_PORT: &str =
    "select * from packet_information where source_port = $1 order by serial_no limit $2 offset $3";

pub static QUERY_OF_GET_TRAFFIC_BY_PROTOCOL: &str =
    "select * from packet_information where protocol = $1 order by serial_no limit $2 offset $3";
