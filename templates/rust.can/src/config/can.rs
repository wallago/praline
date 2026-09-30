use std::collections::HashSet;

use serde::Deserialize;

/// CAN config.
#[derive(Clone, Debug, Deserialize, Default)]
#[serde(default, deny_unknown_fields)]
pub struct CanConfig {
    /// Frames ID (HEX) to listen.
    pub target_frames_id: HashSet<String>,
    /// Bus name.
    pub bus: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserializes_toml_list() {
        let can_config: CanConfig = toml::from_str(
            r#"
            target_frames_id = ["501", "502"]
            bus = "can0"
            "#,
        )
        .expect("failed to deserialize string");
        assert_eq!(can_config.target_frames_id.len(), 2);
        assert_eq!(can_config.bus, "can0");
        assert!(
            toml::from_str::<CanConfig>("target_frames_id = \"test\"\nbus = [\"test\"]").is_err()
        );
    }
}
