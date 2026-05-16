use crate::RuntimeError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EngineConfig {
    pub app_name: String,
    pub window_width: u32,
    pub window_height: u32,
    pub clear_color: [u8; 4],
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            app_name: "Xenon Sandbox".to_string(),
            window_width: 1280,
            window_height: 720,
            clear_color: [0x20, 0x30, 0x40, 0xff],
        }
    }
}

impl EngineConfig {
    pub fn validate(&self) -> Result<(), RuntimeError> {
        if self.app_name.trim().is_empty() {
            return Err(RuntimeError::InvalidConfig(
                "app_name cannot be empty".to_string(),
            ));
        }

        if self.window_width == 0 {
            return Err(RuntimeError::InvalidWindowWidth {
                width: self.window_width,
            });
        }

        if self.window_height == 0 {
            return Err(RuntimeError::InvalidWindowHeight {
                height: self.window_height,
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_valid_config() {
        let config = EngineConfig::default();

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validate_invalid_app_name() {
        let config = EngineConfig {
            app_name: "".to_string(),
            ..EngineConfig::default()
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validate_invalid_window_width() {
        let config = EngineConfig {
            window_width: 0,
            ..EngineConfig::default()
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validate_invalid_window_height() {
        let config = EngineConfig {
            window_height: 0,
            ..EngineConfig::default()
        };

        assert!(config.validate().is_err());
    }
}
