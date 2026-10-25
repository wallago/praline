//! Streamable HTTP transport, mounted by the server at `/mcp`.

use std::sync::Arc;

use rmcp::transport::streamable_http_server::{
    StreamableHttpServerConfig, StreamableHttpService, session::local::LocalSessionManager,
};

use super::Server;

/// Env var listing the `Host` values accepted, comma-separated.
const ALLOWED_HOSTS_VAR: &str = "MCP_ALLOWED_HOSTS";

/// MCP over Streamable HTTP, ready for `Router::nest_service`.
///
/// Only loopback `Host` headers are accepted by default, against DNS
/// rebinding. Behind a public URL (e.g. a claude.ai custom connector), set
/// `MCP_ALLOWED_HOSTS` to that hostname.
pub(crate) fn service() -> StreamableHttpService<Server, LocalSessionManager> {
    let mut config = StreamableHttpServerConfig::default();
    if let Ok(hosts) = std::env::var(ALLOWED_HOSTS_VAR) {
        config = config.with_allowed_hosts(hosts.split(',').map(str::trim));
    }
    StreamableHttpService::new(|| Ok(Server), Arc::default(), config)
}
