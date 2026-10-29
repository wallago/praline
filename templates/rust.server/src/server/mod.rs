//! Server core.

use axum::{Router, http::StatusCode, routing::get, serve};

use crate::{config::Config, prelude::*};

// {slot:rust.server.mods}

/// Liveness probe: answers as long as the server is up.
async fn health() -> StatusCode {
    StatusCode::OK
}

/// Routes that sit behind auth, when an auth option is on.
fn protected() -> Router {
    Router::new()
        // {slot:rust.server.routes}
}

/// Start web services.
pub(super) async fn start(config: &Config) -> Result<()> {
    let (address, port) = (&config.server.address, config.server.port);
    let listener = tokio::net::TcpListener::bind(format!("{address}:{port}")).await?;
    tracing::info!("Server running");
    let protected = protected()
        // {slot:rust.server.routes.config}
    ;
    // {slot:rust.server.protect}
    let routes = Router::new()
        .route("/health", get(health))
        .merge(protected)
        // {slot:rust.server.serve}
    ;
    Ok(serve(listener, routes).await?)
}
