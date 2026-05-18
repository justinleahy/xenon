use super::{
    AssetError, AssetId, AssetManifest, Handle, ShaderAsset, ShaderAssetEntry, TextureAsset,
    TextureAssetEntry,
};
use std::{fs, path::PathBuf};

pub struct AssetManager {
    manifest: AssetManifest,
    asset_root: PathBuf,
}

impl AssetManager {
    pub fn new(manifest: AssetManifest) -> Self {
        Self::with_root(manifest, ".")
    }

    pub fn with_root(manifest: AssetManifest, asset_root: impl Into<PathBuf>) -> Self {
        Self {
            manifest,
            asset_root: asset_root.into(),
        }
    }

    pub fn load_manifest(path: impl Into<PathBuf>) -> Result<Self, AssetError> {
        let path = path.into();
        let manifest = AssetManifest::load_from_file(&path)?;
        let asset_root = path
            .parent()
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."));

        Ok(Self::with_root(manifest, asset_root))
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

    pub fn texture_file_path(&self, id: &AssetId) -> Option<PathBuf> {
        self.texture_path(id).map(|path| self.asset_root.join(path))
    }

    pub fn shader_file_path(&self, id: &AssetId) -> Option<PathBuf> {
        self.shader_path(id).map(|path| self.asset_root.join(path))
    }

    pub fn load_shader_source(&self, id: &AssetId) -> Result<String, AssetError> {
        let path = self
            .shader_file_path(id)
            .ok_or_else(|| AssetError::MissingAsset { id: id.clone() })?;

        fs::read_to_string(&path).map_err(|source| AssetError::AssetRead {
            id: id.clone(),
            path,
            source,
        })
    }

    pub fn load_texture_bytes(&self, id: &AssetId) -> Result<Vec<u8>, AssetError> {
        let path = self
            .texture_file_path(id)
            .ok_or_else(|| AssetError::MissingAsset { id: id.clone() })?;

        fs::read(&path).map_err(|source| AssetError::AssetRead {
            id: id.clone(),
            path,
            source,
        })
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

    fn temp_asset_root(name: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        std::env::temp_dir().join(format!("xenon-assets-manager-{name}-{nanos}"))
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
    fn test_texture_file_path_resolves_against_asset_root() {
        let manager = AssetManager::with_root(test_manifest(), PathBuf::from("assets"));
        let id = AssetId::new("textures/player");

        assert_eq!(
            manager.texture_file_path(&id),
            Some(PathBuf::from("assets").join("textures/player.png"))
        );
    }

    #[test]
    fn test_shader_file_path_resolves_against_asset_root() {
        let manager = AssetManager::with_root(test_manifest(), PathBuf::from("assets"));
        let id = AssetId::new("shaders/colored");

        assert_eq!(
            manager.shader_file_path(&id),
            Some(PathBuf::from("assets").join("shaders/colored.wgsl"))
        );
    }

    #[test]
    fn test_load_shader_source_reads_known_shader() {
        let asset_root = temp_asset_root("shader-source");
        let shader_dir = asset_root.join("shaders");
        fs::create_dir_all(&shader_dir).unwrap();
        fs::write(shader_dir.join("colored.wgsl"), "fn vertex_main() {}\n").unwrap();

        let manager = AssetManager::with_root(test_manifest(), asset_root);

        let source = manager
            .load_shader_source(&AssetId::new("shaders/colored"))
            .unwrap();

        assert_eq!(source, "fn vertex_main() {}\n");
    }

    #[test]
    fn test_load_shader_source_returns_missing_asset_error() {
        let manager = AssetManager::new(test_manifest());

        let error = manager
            .load_shader_source(&AssetId::new("shaders/missing"))
            .unwrap_err();

        match error {
            AssetError::MissingAsset { id } => {
                assert_eq!(id, AssetId::new("shaders/missing"));
            }
            _ => panic!("expected missing asset error"),
        }
    }

    #[test]
    fn test_load_shader_source_returns_read_error_for_missing_file() {
        let asset_root = temp_asset_root("missing-shader-file");
        let manager = AssetManager::with_root(test_manifest(), asset_root.clone());

        let error = manager
            .load_shader_source(&AssetId::new("shaders/colored"))
            .unwrap_err();

        match error {
            AssetError::AssetRead { id, path, source } => {
                assert_eq!(id, AssetId::new("shaders/colored"));
                assert_eq!(path, asset_root.join("shaders/colored.wgsl"));
                assert_eq!(source.kind(), std::io::ErrorKind::NotFound);
            }
            _ => panic!("expected asset read error"),
        }
    }

    #[test]
    fn test_load_texture_bytes_reads_known_texture() {
        let asset_root = temp_asset_root("texture-bytes");
        let texture_dir = asset_root.join("textures");
        fs::create_dir_all(&texture_dir).unwrap();
        fs::write(texture_dir.join("player.png"), &[0x89, b'P', b'N', b'G']).unwrap();

        let manager = AssetManager::with_root(test_manifest(), asset_root);

        let bytes = manager
            .load_texture_bytes(&AssetId::new("textures/player"))
            .unwrap();

        assert_eq!(bytes, vec![0x89, b'P', b'N', b'G']);
    }

    #[test]
    fn test_load_texture_bytes_returns_missing_asset_error() {
        let manager = AssetManager::new(test_manifest());

        let error = manager
            .load_texture_bytes(&AssetId::new("textures/missing"))
            .unwrap_err();

        match error {
            AssetError::MissingAsset { id } => {
                assert_eq!(id, AssetId::new("textures/missing"));
            }
            _ => panic!("expected missing asset error"),
        }
    }

    #[test]
    fn test_load_texture_bytes_returns_read_error_for_missing_file() {
        let asset_root = temp_asset_root("missing-texture-file");
        let manager = AssetManager::with_root(test_manifest(), asset_root.clone());

        let error = manager
            .load_texture_bytes(&AssetId::new("textures/player"))
            .unwrap_err();

        match error {
            AssetError::AssetRead { id, path, source } => {
                assert_eq!(id, AssetId::new("textures/player"));
                assert_eq!(path, asset_root.join("textures/player.png"));
                assert_eq!(source.kind(), std::io::ErrorKind::NotFound);
            }
            _ => panic!("expected asset read error"),
        }
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
