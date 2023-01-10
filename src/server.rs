use std::net::SocketAddr;
use std::path::PathBuf;
use axum::Router;
use axum::routing::get;
use log::info;
use tower_http::cors;
use tower_http::cors::CorsLayer;
use anyhow::{Context, Result};
use axum_server::tls_rustls::RustlsConfig;
use crate::Config;
use crate::endpoints::check_ip_endpoint;

/// Start the API to query for mails and subjects
pub async fn start_http() -> Result<String> {
    let socket_addr: SocketAddr = format!(
        "{}:{}",
        Config::global().listen.ip,
        Config::global().listen.port,
    ).parse()?;
    let cors = CorsLayer::new().allow_origin(cors::AllowOrigin::any());
    let app = Router::new().route("/check_ip", get(check_ip_endpoint)).layer(cors);
    info!("Server listening on {}", socket_addr);
    match &Config::global().tls {
        Some(c) => {
            let rustls_config = RustlsConfig::from_pem_file(
                PathBuf::from(&c.cert),
                PathBuf::from(&c.key),
            ).await
                .with_context(||format!("reading pem files at configured paths: {} {}", &c.cert, &c.key))?;
            axum_server::bind_rustls(socket_addr, rustls_config)
                .serve(app.into_make_service())
                .await
                .with_context(||format!("binding axum_server to: {}", socket_addr))?
        }
        None => {
            axum::Server::bind(&socket_addr)
                .serve(app.into_make_service())
                .await?;
        }
    }
    Ok(String::from("HTTP server stopped"))
}