mod router;
mod models;

use axum::{response::IntoResponse, routing::get, Json, Router};
use models::response::ApiResponse;
use mongodb::{bson::doc, options::ClientOptions, Client};
use router::basic::home;
use tokio::time::Instant;
use dotenv::dotenv;
use std::env;

async fn test() -> impl IntoResponse {
    let formated_string: String = format!("Hello world {:?}", Instant::now().to_owned());
    let json = ApiResponse{
        message: formated_string.to_string(),
    };

    Json(json)
}

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    dotenv().ok();

    // Mongodb Connection
    let db_uri: String = env::var("MONGO_URI").expect("Error MONGO_URI not found");
    let database_name = env::var("DATABASE_NAME").unwrap_or_else(|_| "mydatabase".to_string());

    let client_options = ClientOptions::parse(&db_uri).await?;
    let client = Client::with_options(client_options)?;
    let db = client.database(&database_name);

    match db.run_command(doc! { "ping": 1 }).await {
        Ok(_) => println!("Succerfull connect to database"),
        Err(e) => {
            eprintln!("Failed connect to database {}", e);
            return Err(e);
        }
    }

    // App Data
    let app_name: String = env::var("APP_NAME").unwrap_or_else(|_| "ZxRust Api".to_string());
    let address: String = env::var("ADDRESS").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port: String = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    let ip: String = format!("{}:{}", address, port);

    println!("{} started", app_name);
    println!("Listening to {}", ip);

    let app: Router = Router::new()
        .route("/", get(home))
        .route("/test", get(test));

    let listener = tokio::net::TcpListener::bind(ip).await.unwrap();


    axum::serve(listener, app).await.unwrap();

    Ok(())
}