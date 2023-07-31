use crate::database::DatabaseConnection;
use crate::information::*;
use warp::{Filter, Rejection};
use crate::pagination::pagination_data;

pub async fn post_networks_list(
    item: PacketData,
    db: DatabaseConnection,
) -> Result<impl warp::Reply, Rejection> {
    let item_clone = item.clone();
    db.insert_items(item_clone)
        .await
        .expect("Failed to insert data");
    Ok(warp::reply::json(&item))
}
pub async fn get_all_networks_list(
    db: DatabaseConnection,
    pagination_params: PaginationParams,
) -> Result<impl warp::Reply, Rejection> {
    // Read all items from the database
    let all_data = db.read_items().await.expect("Failed to read data");
    let api_response = pagination_data(all_data,pagination_params);
    Ok(warp::reply::json(&api_response))
}
pub async fn get_network_list_by_serial_number(
    serial_no: i32,
    db: DatabaseConnection,
) -> Result<impl warp::Reply,Rejection> {
    let item = db.get_item_by_serial_number(serial_no).await.expect("Id not found");
    Ok(warp::reply::json(&item))
}
pub async fn get_traffic(
    ip_source: String,
    db: DatabaseConnection,
    pagination_params: PaginationParams,
) -> Result<impl warp::Reply, Rejection> {
    let item = db.get_traffic_of_ip_source(ip_source).await.expect("Ip Source not found");
    let api_response = pagination_data(item, pagination_params);
    Ok(warp::reply::json(&api_response))
}
pub async fn filter_by_protocol(
    protocol: String,
    db: DatabaseConnection,
    pagination_params: PaginationParams,
) -> Result<impl warp::Reply, Rejection> {
    let item = db.get_network_list_by_protocol(protocol).await.expect("Protocol not found");
    let api_response = pagination_data(item, pagination_params);
    Ok(warp::reply::json(&api_response))

}
pub fn json_body() -> impl Filter<Extract = (PacketData,), Error = Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
