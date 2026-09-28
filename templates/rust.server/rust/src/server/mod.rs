pub fn router() -> Router {
    Router::new().route("/health", get(health))
    // {slot:server.routes}
}
