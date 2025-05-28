mod router;
mod models;

use axum::{response::IntoResponse, routing::get, Json, Router};
use models::response::ApiResponse;
use router::route::home;
use tokio::time::Instant;

async fn test() -> impl IntoResponse {
    let formated_string: String = format!("Hello world {:?}", Instant::now().to_owned());
    let json = ApiResponse{
        message: formated_string.to_string(),
    };

    Json(json)
}

#[tokio::main]
async fn main() {

    let address: String = String::from("0.0.0.0");
    let port: String = String::from("3000");

    let ip: String = format!("{}:{}", address, port);

    println!("Listening to {}", ip);

    let app: Router = Router::new()
        .route("/", get(home))
        .route("/test", get(test));

    let listener = tokio::net::TcpListener::bind(ip).await.unwrap();


    axum::serve(listener, app).await.unwrap();
}