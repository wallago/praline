/// Global server routes.
fn routes() -> Router {
    Router::new()
        .route("/health", get(health))
        .nest_service("/api", ws::routes(state))
}

/// Start web services.
pub async fn start(address: &str, port: &str) -> Result<()> {
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", address, port)).await?;
    let make_service = app.into_make_service_with_connect_info::<SocketAddr>();
    Ok(axum::serve(listener, make_service).await?)
}
