//! Server core.

use axum::{Router, http::StatusCode, routing::get, serve};

use crate::{config::Config, prelude::*};

// {slot:rust.server.mods}

/// Liveness probe: answers as long as the server is up.
async fn health() -> StatusCode {
    StatusCode::OK
}

/// Global server routes.
fn routes() -> Router {
    Router::new().route("/health", get(health))
    // {slot:rust.server.routes}
}

/// Start web services.
pub(super) async fn start(config: &Config) -> Result<()> {
    let (address, port) = (&config.server.address, config.server.port);
    let listener = tokio::net::TcpListener::bind(format!("{address}:{port}")).await?;
    tracing::info!("Server running");
    let routes = routes()
        // {slot:rust.server.routes.config}
        // {slot:rust.server.serve}
    ;
    Ok(serve(listener, routes).await?)
}
