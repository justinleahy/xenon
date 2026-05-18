use super::{
    AssetError, AssetId, AssetManifest, Handle, ShaderAsset, ShaderAssetEntry, TextureAsset,
    TextureAssetEntry,
};
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
        self.texture_entry(id).is_some()
    }

    pub fn has_shader(&self, id: &AssetId) -> bool {
        self.shader_entry(id).is_some()
    }

    pub fn texture_entry(&self, id: &AssetId) -> Option<&TextureAssetEntry> {
        self.manifest
            .textures
            .iter()
            .find(|texture| texture.id == *id)
    }

    pub fn shader_entry(&self, id: &AssetId) -> Option<&ShaderAssetEntry> {
        self.manifest.shaders.iter().find(|shader| shader.id == *id)
    }

    pub fn texture_path(&self, id: &AssetId) -> Option<&str> {
        self.texture_entry(id).map(|texture| texture.path.as_str())
    }

    pub fn shader_path(&self, id: &AssetId) -> Option<&str> {
        self.shader_entry(id).map(|shader| shader.path.as_str())
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
    fn test_texture_entry_returns_known_texture_metadata() {
        let manager = AssetManager::new(test_manifest());
        let id = AssetId::new("textures/player");

        let entry = manager.texture_entry(&id).unwrap();

        assert_eq!(&entry.id, &id);
        assert_eq!(entry.path, "textures/player.png");
    }

    #[test]
    fn test_shader_entry_returns_known_shader_metadata() {
        let manager = AssetManager::new(test_manifest());
        let id = AssetId::new("shaders/colored");

        let entry = manager.shader_entry(&id).unwrap();

        assert_eq!(&entry.id, &id);
        assert_eq!(entry.path, "shaders/colored.wgsl");
    }

    #[test]
    fn test_texture_path_returns_known_texture_path() {
        let manager = AssetManager::new(test_manifest());
        let id = AssetId::new("textures/player");

        assert_eq!(manager.texture_path(&id), Some("textures/player.png"));
    }

    #[test]
    fn test_shader_path_returns_known_shader_path() {
        let manager = AssetManager::new(test_manifest());
        let id = AssetId::new("shaders/colored");

        assert_eq!(manager.shader_path(&id), Some("shaders/colored.wgsl"));
    }

    #[test]
    fn test_asset_paths_return_none_for_missing_assets() {
        let manager = AssetManager::new(test_manifest());

        assert_eq!(
            manager.texture_path(&AssetId::new("textures/missing")),
            None
        );
        assert_eq!(manager.shader_path(&AssetId::new("shaders/missing")), None);
    }

    #[test]
    fn test_texture_handle_and_path_resolve_same_asset_id() {
        let manager = AssetManager::new(test_manifest());
        let id = AssetId::new("textures/player");

        let handle = manager.texture(id.as_str()).unwrap();

        assert_eq!(handle.id(), &id);
        assert_eq!(
            manager.texture_path(handle.id()),
            Some("textures/player.png")
        );
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
