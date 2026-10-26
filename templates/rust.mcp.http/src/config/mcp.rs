use serde::Deserialize;

/// MCP config.
#[derive(Clone, Debug, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(crate) struct McpConfig {
    /// `Host` headers accepted on `/mcp`; anything else gets a 403.
    pub(crate) allowed_hosts: Vec<String>,
}

impl Default for McpConfig {
    /// Loopback only: safe until a public hostname is listed.
    fn default() -> Self {
        Self {
            allowed_hosts: ["localhost", "127.0.0.1", "::1"].map(String::from).to_vec(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserializes_toml() -> Result<(), toml::de::Error> {
        let config = toml::from_str::<McpConfig>(r#"allowed_hosts = ["mcp.example.com"]"#)?;
        assert_eq!(config.allowed_hosts, ["mcp.example.com"]);
        Ok(())
    }

    #[test]
    fn test_defaults_to_loopback() -> Result<(), toml::de::Error> {
        let config = toml::from_str::<McpConfig>("")?;
        assert_eq!(config.allowed_hosts, ["localhost", "127.0.0.1", "::1"]);
        Ok(())
    }
}
