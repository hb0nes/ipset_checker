use axum::extract::Query;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::{Serialize, Deserialize};
use crate::ipset::check_ip;

#[derive(Debug, Deserialize)]
pub struct IpCheckQuery {
    ip: String,
}

#[derive(Serialize)]
pub struct Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    results: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

pub async fn check_ip_endpoint(query: Query<IpCheckQuery>) -> impl IntoResponse {
    let a = check_ip(&query.ip);
    match a {
        Ok(val) => (StatusCode::OK, Json(Response {
            results: Some(val),
            error: None,
        })),
        Err(why) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(Response {
                results: None,
                error: Some(format!("{:#}", why))
            }))
        }
    }
}
