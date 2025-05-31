use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse {
    pub message: String,
}
 #[derive(Serialize)]
pub struct AppResponse {
    pub name: String,
    pub version: String,
    pub endpoint: String,
    pub result: ResultResponse,
}

#[derive(Serialize)]
pub struct ResultResponse {
    pub res: String,
}