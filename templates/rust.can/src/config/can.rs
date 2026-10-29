use std::collections::HashSet;

use serde::Deserialize;

/// CAN config.
#[derive(Clone, Debug, Deserialize, Default)]
#[serde(default, deny_unknown_fields)]
pub(crate) struct CanConfig {
    /// Frames ID (HEX) to listen.
    pub(crate) target_frames_id: HashSet<String>,
    /// Bus name.
    pub(crate) bus: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserializes_toml_list() -> Result<(), toml::de::Error> {
        let can_config: CanConfig = toml::from_str(
            r#"
            target_frames_id = ["501", "502"]
            bus = "can0"
            "#,
        )?;
        assert_eq!(can_config.target_frames_id.len(), 2);
        assert_eq!(can_config.bus, "can0");
        assert!(
            toml::from_str::<CanConfig>("target_frames_id = \"test\"\nbus = [\"test\"]").is_err()
        );
        Ok(())
    }
}
