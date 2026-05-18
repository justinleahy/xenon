use super::{AssetError, AssetId};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssetManifest {
    #[serde(default)]
    pub textures: Vec<TextureAssetEntry>,

    #[serde(default)]
    pub shaders: Vec<ShaderAssetEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextureAssetEntry {
    pub id: AssetId,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShaderAssetEntry {
    pub id: AssetId,
    pub path: String,
}

impl AssetManifest {
    pub fn load_from_file(path: impl AsRef<Path>) -> Result<Self, AssetError> {
        let path = path.as_ref();

        let contents =
            std::fs::read_to_string(path).map_err(|source| AssetError::ManifestRead {
                path: path.to_path_buf(),
                source,
            })?;

        toml::from_str(&contents).map_err(|source| AssetError::ManifestParse {
            path: path.to_path_buf(),
            source,
        })
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

    fn temp_manifest_path(name: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        std::env::temp_dir().join(format!("xenon-assets-{name}-{nanos}.toml"))
    }

    #[test]
    fn test_load_from_file_loads_valid_manifest() {
        let path = temp_manifest_path("valid");
        fs::write(
            &path,
            r#"
[[textures]]
id = "textures/player"
path = "textures/player.png"

[[shaders]]
id = "shaders/colored"
path = "shaders/colored.wgsl"
            "#,
        )
        .unwrap();

        let manifest = AssetManifest::load_from_file(&path).unwrap();

        assert_eq!(
            manifest.textures,
            vec![TextureAssetEntry {
                id: AssetId::new("textures/player"),
                path: "textures/player.png".to_string(),
            }]
        );
        assert_eq!(
            manifest.shaders,
            vec![ShaderAssetEntry {
                id: AssetId::new("shaders/colored"),
                path: "shaders/colored.wgsl".to_string(),
            }]
        );
    }

    #[test]
    fn test_load_from_file_defaults_missing_sections_to_empty() {
        let path = temp_manifest_path("empty");
        fs::write(&path, "").unwrap();

        let manifest = AssetManifest::load_from_file(&path).unwrap();

        assert!(manifest.textures.is_empty());
        assert!(manifest.shaders.is_empty());
    }

    #[test]
    fn test_load_from_file_returns_read_error_for_missing_file() {
        let path = temp_manifest_path("missing");

        let result = AssetManifest::load_from_file(&path);

        assert!(matches!(result, Err(AssetError::ManifestRead { .. })));
    }

    #[test]
    fn test_load_from_file_returns_parse_error_for_malformed_toml() {
        let path = temp_manifest_path("malformed");
        fs::write(&path, "[[textures]").unwrap();

        let result = AssetManifest::load_from_file(&path);

        assert!(matches!(result, Err(AssetError::ManifestParse { .. })));
    }
}
