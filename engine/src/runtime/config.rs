use crate::RuntimeError;
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
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
    pub fn load_from_file(path: impl AsRef<Path>) -> Result<Self, RuntimeError> {
        let path = path.as_ref();
        let contents =
            std::fs::read_to_string(path).map_err(|source| RuntimeError::ConfigRead {
                path: path.to_path_buf(),
                source,
            })?;

        let config: Self =
            toml::from_str(&contents).map_err(|source| RuntimeError::ConfigParse {
                path: path.to_path_buf(),
                source,
            })?;

        config.validate()?;

        Ok(config)
    }

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
    use std::{
        fs,
        path::PathBuf,
        time::{SystemTime, UNIX_EPOCH},
    };

    fn temp_config_path(name: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        std::env::temp_dir().join(format!("xenon-{name}-{nanos}.toml"))
    }

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

    #[test]
    fn test_load_from_file_loads_valid_toml() {
        let path = temp_config_path("valid");
        fs::write(
            &path,
            r#"
app_name = "Test App"
window_width = 800
window_height = 600
clear_color = [1, 2, 3, 255]
            "#,
        )
        .unwrap();

        let config = EngineConfig::load_from_file(&path).unwrap();

        assert_eq!(config.app_name, "Test App");
        assert_eq!(config.window_width, 800);
        assert_eq!(config.window_height, 600);
        assert_eq!(config.clear_color, [1, 2, 3, 255]);

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_load_from_file_returns_read_error_for_missing_file() {
        let path = temp_config_path("missing");

        let result = EngineConfig::load_from_file(&path);

        assert!(matches!(result, Err(RuntimeError::ConfigRead { .. })));
    }

    #[test]
    fn test_load_from_file_returns_parse_error_for_malformed_toml() {
        let path = temp_config_path("malformed");

        fs::write(&path, "app_name = [").unwrap();

        let result = EngineConfig::load_from_file(&path);

        assert!(matches!(result, Err(RuntimeError::ConfigParse { .. })));

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_load_from_file_validates_loaded_config() {
        let path = temp_config_path("invalid");
        fs::write(
            &path,
            r#"
app_name = "Test App"
window_width = 0
window_height = 600
clear_color = [1, 2, 3, 255]
            "#,
        )
        .unwrap();

        let result = EngineConfig::load_from_file(&path);

        assert!(matches!(
            result,
            Err(RuntimeError::InvalidWindowWidth { width: 0 })
        ));

        fs::remove_file(path).unwrap();
    }
}
