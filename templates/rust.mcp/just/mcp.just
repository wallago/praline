# {if:just}
# ── MCP ─────────────────────────────────

# {ifnot:rust.mcp.http}
# Open the MCP Inspector on the server (stdio)
[group('mcp')]
mcp-inspect:
    npx @modelcontextprotocol/inspector cargo run --quiet -- --config config.toml
# {endif:rust.mcp.http}
# {if:rust.mcp.http}
# Open the MCP Inspector; point it at the URL below (Streamable HTTP)
[group('mcp')]
mcp-inspect:
    @echo "MCP endpoint: {{ endpoint }}/mcp"
    npx @modelcontextprotocol/inspector
# {endif:rust.mcp.http}
# {endif:just}
