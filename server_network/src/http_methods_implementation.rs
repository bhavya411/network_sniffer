use crate::database::DatabaseConnection;
use crate::information::*;
use warp::{Filter, Rejection};
use warp::reject::custom;

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
    let limit = pagination_params.per_page;
    let offset = (pagination_params.page - 1) * limit;
    // Read all items from the database
    let api_response = match db.read_items(limit as i32, offset as i32).await {
        Ok(data) => {
            if data.is_empty() {
                // If the result is empty, return a custom rejection indicating no data found.
                return Err(custom(NoDataFound));
            }
            data
        }
        Err(err) => {
            // Handle the database error by returning a rejection with the error message.
            return Err(custom(DatabaseError(err.to_string())));
        }
    };
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
    let limit = pagination_params.per_page;
    let offset = (pagination_params.page - 1) * limit;
    let api_response = db.get_traffic_of_ip_source(ip_source,limit as i32, offset as i32).await.expect("Ip Source not found");
    Ok(warp::reply::json(&api_response))
}
pub async fn filter_by_protocol(
    protocol: String,
    db: DatabaseConnection,
    pagination_params: PaginationParams,
) -> Result<impl warp::Reply, Rejection> {
    let limit = pagination_params.per_page;
    let offset = (pagination_params.page - 1) * limit;
    let api_response = db.get_network_list_by_protocol(protocol,limit as i32, offset as i32).await.expect("Protocol not found");
    Ok(warp::reply::json(&api_response))

}
pub fn json_body() -> impl Filter<Extract = (PacketData,), Error = Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
