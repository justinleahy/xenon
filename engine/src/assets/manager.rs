use super::{AssetError, AssetId, AssetManifest, Handle, ShaderAsset, TextureAsset};
use std::path::Path;

pub struct AssetManager {
    manifest: AssetManifest,
}

impl AssetManager {
    pub fn new(manifest: AssetManifest) -> Self {
        Self { manifest }
    }

    pub fn load_manifest(path: impl AsRef<Path>) -> Result<Self, AssetError> {
        let manifest = AssetManifest::load_from_file(path)?;

        Ok(Self::new(manifest))
    }

    pub fn texture(&self, id: impl Into<String>) -> Option<Handle<TextureAsset>> {
        let id = AssetId::new(id);

        self.has_texture(&id).then(|| Handle::new(id))
    }

    pub fn shader(&self, id: impl Into<String>) -> Option<Handle<ShaderAsset>> {
        let id = AssetId::new(id);

        self.has_shader(&id).then(|| Handle::new(id))
    }

    pub fn has_texture(&self, id: &AssetId) -> bool {
        self.manifest
            .textures
            .iter()
            .any(|texture| texture.id == *id)
    }

    pub fn has_shader(&self, id: &AssetId) -> bool {
        self.manifest.shaders.iter().any(|shader| shader.id == *id)
    }

    pub fn manifest(&self) -> &AssetManifest {
        &self.manifest
    }
}

#[cfg(test)]
mod tests {
    use super::super::{ShaderAssetEntry, TextureAssetEntry};
    use super::*;
    use std::{
        fs,
        path::PathBuf,
        time::{SystemTime, UNIX_EPOCH},
    };

    fn test_manifest() -> AssetManifest {
        AssetManifest {
            textures: vec![TextureAssetEntry {
                id: AssetId::new("textures/player"),
                path: "textures/player.png".to_string(),
            }],
            shaders: vec![ShaderAssetEntry {
                id: AssetId::new("shaders/colored"),
                path: "shaders/colored.wgsl".to_string(),
            }],
        }
    }

    fn temp_manifest_path(name: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        std::env::temp_dir().join(format!("xenon-assets-manager-{name}-{nanos}.toml"))
    }

    #[test]
    fn test_texture_returns_handle_for_known_texture() {
        let manager = AssetManager::new(test_manifest());

        let handle = manager.texture("textures/player").unwrap();

        assert_eq!(handle.id(), &AssetId::new("textures/player"));
    }

    #[test]
    fn test_texture_returns_none_for_missing_texture() {
        let manager = AssetManager::new(test_manifest());

        assert!(manager.texture("textures/missing").is_none());
    }

    #[test]
    fn test_shader_returns_handle_for_known_shader() {
        let manager = AssetManager::new(test_manifest());

        let handle = manager.shader("shaders/colored").unwrap();

        assert_eq!(handle.id(), &AssetId::new("shaders/colored"));
    }

    #[test]
    fn test_shader_returns_none_for_missing_shader() {
        let manager = AssetManager::new(test_manifest());

        assert!(manager.shader("shaders/missing").is_none());
    }

    #[test]
    fn test_load_manifest_builds_manager_from_file() {
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

        let manager = AssetManager::load_manifest(&path).unwrap();

        assert!(manager.texture("textures/player").is_some());
        assert!(manager.shader("shaders/colored").is_some());
    }
}
