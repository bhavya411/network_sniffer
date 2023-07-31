use crate::database::DatabaseConnection;
use crate::models::PacketStructure;
use warp::Filter;

pub async fn get_all_networks_list(
    db: DatabaseConnection,
) -> Result<impl warp::Reply, warp::Rejection> {
    let data = db.read_items().await.expect("Failed to read data");
    println!("Data -> {:?}", data);
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

pub async fn get_traffic(
    ip_source: String,
    db: DatabaseConnection,
) -> Result<impl warp::Reply, warp::Rejection> {
    let item = db
        .get_traffic_of_ip_source(ip_source)
        .await
        .expect("Ip Source not found");
    Ok(warp::reply::json(&item))
}

pub fn json_body() -> impl Filter<Extract = (PacketStructure,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
