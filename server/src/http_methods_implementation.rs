use crate::database::DatabaseConnection;
use crate::models::{DatabaseError, NoDataFound, PaginateStructure};
use warp::reject::custom;
use warp::Filter;

pub async fn get_all_networks_list(
    db: DatabaseConnection,
) -> Result<impl warp::Reply, warp::Rejection> {
    let data = db.read_all_items().await.expect("Failed to read data");
    Ok(warp::reply::json(&data))
}

pub async fn get_networks_list_in_pages(
    db: DatabaseConnection,
    paginate_structure: PaginateStructure,
) -> Result<impl warp::Reply, warp::Rejection> {
    let data = match db.read_items(paginate_structure).await {
        Ok(vec_data) => {
            if vec_data.is_empty() {
                return Err(custom(NoDataFound));
            }
            vec_data
        }
        Err(err) => return Err(custom(DatabaseError(err.to_string()))),
    };
    for page in &data {
        println!("page data  -> {:?}", page);
    }
    Ok(warp::reply::json(&data))
}

//
// pub async fn get_network_list_by_serial_number(
//     serial_no: i32,
//     db: DatabaseConnection,
// ) -> Result<impl warp::Reply, warp::Rejection> {
//     let item = db.get_item_by_serial_number(serial_no).await.expect("Id not found");
//     match item {
//         Some(result) => {
//             println!("Data -> {:?}", result);
//             Ok(warp::reply::json(&result))
//         }
//         None => Err(warp::reject::not_found()),
//     }
// }

pub async fn get_traffic_source_ip(
    ip_source: String,
    db: DatabaseConnection,
    paginate_structure: PaginateStructure,
) -> Result<impl warp::Reply, warp::Rejection> {
    let packet = db
        .get_traffic_of_ip_source(ip_source, paginate_structure)
        .await
        .expect("Ip Source not found");
    Ok(warp::reply::json(&packet))
}

pub async fn get_traffic_source_port(
    source_port: i64,
    db: DatabaseConnection,
    paginate_structure: PaginateStructure,
) -> Result<impl warp::Reply, warp::Rejection> {
    let packet_structure = db
        .get_traffic_of_source_port(source_port, paginate_structure)
        .await
        .expect("Ip Source_Port not found");

    Ok(warp::reply::json(&packet_structure))
}

pub async fn get_traffic_protocol(
    protocol: String,
    db: DatabaseConnection,
    paginate_structure: PaginateStructure,
) -> Result<impl warp::Reply, warp::Rejection> {
    let packet_structure = db
        .get_traffic_of_protocol(protocol, paginate_structure)
        .await
        .expect("Ip protocol not found");
    Ok(warp::reply::json(&packet_structure))
}

/*
pub fn json_body() -> impl Filter<Extract = (PacketStructure,), Error = warp::Rejection> + Clone {
warp::body::content_length_limit(1024*16)
.and(warp::body::json())
}
*/

pub fn paginate() -> impl Filter<Extract = (PaginateStructure,), Error = warp::Rejection> + Clone {
    warp::query::<PaginateStructure>()
}
