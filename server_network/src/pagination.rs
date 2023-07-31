use crate::information::*;

pub fn pagination_data(all_data: Vec<PacketData>, pagination_params: PaginationParams) -> ApiResponse<PacketData> {
    let total_items = all_data.len();
    let start_index = (pagination_params.page - 1) * pagination_params.per_page;

    // Extract the data for the requested page
    let page_data = all_data
        .into_iter()
        .skip(start_index)
        .take(pagination_params.per_page)
        .collect::<Vec<_>>();

    let total_pages = (total_items as f32 / pagination_params.per_page as f32).ceil() as usize;
    let is_empty_page = page_data.is_empty();

    // Prepare the API response with data and pagination information
    let api_response = ApiResponse {
        data: if is_empty_page {
            Vec::new() // Return an empty vector if no data is found
        } else {
            page_data
        },
        pagination: PaginationInfoResponse {
            total_items: if is_empty_page { 0 } else { total_items },
            per_page: pagination_params.per_page,
            current_page: pagination_params.page,
            total_pages: if is_empty_page { 0 } else { total_pages },
        },
    };
    api_response
}