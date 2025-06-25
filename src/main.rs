use warp::{Filter, Reply};
use serde::Serialize;
use std::convert::Infallible;

#[derive(Serialize)]
struct ApiResponse<'a> {
    message: &'a str,
    data: &'a str,
}

// i dont like rust.......
async fn create_response<'a>(message: &'a str, data: &'a str) -> Result<impl Reply + use<>, Infallible> {
    let response = ApiResponse { message, data };
    Ok(warp::reply::json(&response))
}

#[tokio::main]
async fn main() {
    let api_route = warp::path!("api" / "data" / String)
        .and_then(|data: String| async move {
            let message = "Hello from the API!";
            create_response(message, &data).await
        });

    warp::serve(api_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
