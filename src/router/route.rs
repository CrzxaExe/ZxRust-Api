use std::{fs, path::PathBuf};

use axum::{ response::{Html, IntoResponse}};

pub async fn home() -> impl IntoResponse {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/views/home.html");

    let _html = fs::read_to_string(path).unwrap_or_else(|err| {
        eprintln!("Error on reading file: {}", err);
        "<h1>Error: HTML not found</h1>".to_string()
    });

    Html(_html)
}