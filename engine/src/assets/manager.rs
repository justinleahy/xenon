use super::{
    AssetError, AssetId, AssetKind, AssetManifest, Handle, ShaderAsset, ShaderAssetEntry,
    TextureAsset, TextureAssetEntry, TextureData,
};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    time::SystemTime,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ReloadedAsset {
    pub id: AssetId,
    pub kind: AssetKind,
}

struct CachedAsset<T> {
    value: T,
    modified: SystemTime,
}

pub struct AssetManager {
    manifest: AssetManifest,
    asset_root: PathBuf,
    shader_cache: HashMap<AssetId, CachedAsset<String>>,
    texture_cache: HashMap<AssetId, CachedAsset<TextureData>>,
}

impl AssetManager {
    pub fn new(manifest: AssetManifest) -> Self {
        Self::with_root(manifest, ".")
    }

    pub fn with_root(manifest: AssetManifest, asset_root: impl Into<PathBuf>) -> Self {
        Self {
            manifest,
            asset_root: asset_root.into(),
            shader_cache: HashMap::new(),
            texture_cache: HashMap::new(),
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

    pub fn reload_changed_assets(&mut self) -> Result<Vec<ReloadedAsset>, AssetError> {
        let mut reloaded = Vec::new();

        for id in self.shader_cache.keys().cloned().collect::<Vec<_>>() {
            let Some(path) = self.shader_file_path(&id) else {
                continue;
            };

            let modified = modified_time(&path).map_err(|source| AssetError::AssetRead {
                id: id.clone(),
                path: path.clone(),
                source,
            })?;

            if modified > self.shader_cache.get(&id).unwrap().modified {
                let value = self.read_shader_source(&id)?;

                self.shader_cache
                    .insert(id.clone(), CachedAsset { value, modified });
                reloaded.push(ReloadedAsset {
                    id,
                    kind: AssetKind::Shader,
                })
            }
        }

        for id in self.texture_cache.keys().cloned().collect::<Vec<_>>() {
            let Some(path) = self.texture_file_path(&id) else {
                continue;
            };

            let modified = modified_time(&path).map_err(|source| AssetError::AssetRead {
                id: id.clone(),
                path: path.clone(),
                source,
            })?;

            if modified > self.texture_cache.get(&id).unwrap().modified {
                let value = self.read_texture_data(&id)?;

                self.texture_cache
                    .insert(id.clone(), CachedAsset { value, modified });
                reloaded.push(ReloadedAsset {
                    id,
                    kind: AssetKind::Texture,
                })
            }
        }

        Ok(reloaded)
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

    pub fn load_shader_source(&mut self, id: &AssetId) -> Result<&str, AssetError> {
        if !self.shader_cache.contains_key(id) {
            let path = self
                .shader_file_path(id)
                .ok_or_else(|| AssetError::MissingAsset { id: id.clone() })?;

            let value = self.read_shader_source(id)?;
            let modified = modified_time(&path).map_err(|source| AssetError::AssetRead {
                id: id.clone(),
                path,
                source,
            })?;

            self.shader_cache
                .insert(id.clone(), CachedAsset { value, modified });
        }

        Ok(self.shader_cache.get(id).unwrap().value.as_str())
    }

    pub fn load_texture_data(&mut self, id: &AssetId) -> Result<&TextureData, AssetError> {
        if !self.texture_cache.contains_key(id) {
            let path = self
                .texture_file_path(id)
                .ok_or_else(|| AssetError::MissingAsset { id: id.clone() })?;

            let value = self.read_texture_data(id)?;
            let modified = modified_time(&path).map_err(|source| AssetError::AssetRead {
                id: id.clone(),
                path,
                source,
            })?;

            self.texture_cache
                .insert(id.clone(), CachedAsset { value, modified });
        }

        Ok(&self.texture_cache.get(id).unwrap().value)
    }

    fn read_texture_data(&self, id: &AssetId) -> Result<TextureData, AssetError> {
        let path = self
            .texture_file_path(id)
            .ok_or_else(|| AssetError::MissingAsset { id: id.clone() })?;

        let bytes = fs::read(&path).map_err(|source| AssetError::AssetRead {
            id: id.clone(),
            path: path.clone(),
            source,
        })?;

        let image =
            image::load_from_memory(&bytes).map_err(|source| AssetError::TextureDecode {
                id: id.clone(),
                path,
                source,
            })?;

        let rgba = image.to_rgba8();

        Ok(TextureData {
            width: rgba.width(),
            height: rgba.height(),
            rgba: rgba.into_raw(),
        })
    }

    fn read_shader_source(&self, id: &AssetId) -> Result<String, AssetError> {
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

    pub fn unload_shader_source(&mut self, id: &AssetId) -> bool {
        self.shader_cache.remove(id).is_some()
    }

    pub fn unload_texture_data(&mut self, id: &AssetId) -> bool {
        self.texture_cache.remove(id).is_some()
    }

    pub fn unload_all(&mut self) {
        self.shader_cache.clear();
        self.texture_cache.clear();
    }

    pub fn cached_shader_count(&self) -> usize {
        self.shader_cache.len()
    }

    pub fn cached_texture_count(&self) -> usize {
        self.texture_cache.len()
    }

    pub fn manifest(&self) -> &AssetManifest {
        &self.manifest
    }
}

fn modified_time(path: &Path) -> Result<SystemTime, std::io::Error> {
    fs::metadata(path)?.modified()
}

#[cfg(test)]
mod tests {
    use super::super::{ShaderAssetEntry, TextureAssetEntry};
    use super::*;
    use std::{
        fs,
        path::{Path, PathBuf},
        thread,
        time::{Duration, SystemTime, UNIX_EPOCH},
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

    fn save_test_texture(path: &Path, rgba: [u8; 4]) {
        let image = image::RgbaImage::from_raw(1, 1, rgba.to_vec()).unwrap();
        image.save(path).unwrap();
    }

    fn rewrite_until_modified_after(
        path: &Path,
        previous_modified: SystemTime,
        mut rewrite: impl FnMut(),
    ) {
        for _ in 0..100 {
            thread::sleep(Duration::from_millis(20));
            rewrite();

            let modified = fs::metadata(path).unwrap().modified().unwrap();
            if modified > previous_modified {
                return;
            }
        }

        panic!("failed to produce a newer modification timestamp for {path:?}");
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

        let mut manager = AssetManager::with_root(test_manifest(), asset_root);

        let source = manager
            .load_shader_source(&AssetId::new("shaders/colored"))
            .unwrap();

        assert_eq!(source, "fn vertex_main() {}\n");
    }

    #[test]
    fn test_load_shader_source_returns_missing_asset_error() {
        let mut manager = AssetManager::new(test_manifest());

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
        let mut manager = AssetManager::with_root(test_manifest(), asset_root.clone());

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
    fn test_load_texture_data_decodes_png_to_rgba() {
        let asset_root = temp_asset_root("texture-data");
        let texture_dir = asset_root.join("textures");
        fs::create_dir_all(&texture_dir).unwrap();

        let image = image::RgbaImage::from_raw(2, 1, vec![255, 0, 0, 255, 0, 255, 0, 128]).unwrap();
        image.save(texture_dir.join("player.png")).unwrap();

        let mut manager = AssetManager::with_root(test_manifest(), asset_root);

        let texture = manager
            .load_texture_data(&AssetId::new("textures/player"))
            .unwrap();

        assert_eq!(texture.width, 2);
        assert_eq!(texture.height, 1);
        assert_eq!(texture.rgba, vec![255, 0, 0, 255, 0, 255, 0, 128]);
    }

    #[test]
    fn test_load_texture_data_returns_missing_asset_error() {
        let mut manager = AssetManager::new(test_manifest());

        let error = manager
            .load_texture_data(&AssetId::new("textures/missing"))
            .unwrap_err();

        match error {
            AssetError::MissingAsset { id } => {
                assert_eq!(id, AssetId::new("textures/missing"));
            }
            _ => panic!("expected missing asset error"),
        }
    }

    #[test]
    fn test_load_texture_data_returns_decode_error_for_invalid_image_bytes() {
        let asset_root = temp_asset_root("invalid-texture-data");
        let texture_dir = asset_root.join("textures");
        fs::create_dir_all(&texture_dir).unwrap();
        fs::write(texture_dir.join("player.png"), b"not an image").unwrap();

        let mut manager = AssetManager::with_root(test_manifest(), asset_root.clone());

        let error = manager
            .load_texture_data(&AssetId::new("textures/player"))
            .unwrap_err();

        match error {
            AssetError::TextureDecode { id, path, .. } => {
                assert_eq!(id, AssetId::new("textures/player"));
                assert_eq!(path, asset_root.join("textures/player.png"));
            }
            _ => panic!("expected texture decode error"),
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

    #[test]
    fn test_load_shader_source_uses_cache_until_unloaded() {
        let asset_root = temp_asset_root("shader-cache");
        let shader_dir = asset_root.join("shaders");
        fs::create_dir_all(&shader_dir).unwrap();

        let shader_path = shader_dir.join("colored.wgsl");
        fs::write(&shader_path, "first").unwrap();

        let mut manager = AssetManager::with_root(test_manifest(), asset_root);
        let id = AssetId::new("shaders/colored");

        assert_eq!(manager.load_shader_source(&id).unwrap(), "first");
        assert_eq!(manager.cached_shader_count(), 1);

        fs::write(&shader_path, "second").unwrap();

        assert_eq!(manager.load_shader_source(&id).unwrap(), "first");
        assert!(manager.unload_shader_source(&id));
        assert_eq!(manager.cached_shader_count(), 0);
        assert_eq!(manager.load_shader_source(&id).unwrap(), "second");
    }

    #[test]
    fn test_load_texture_data_uses_cache_until_unloaded() {
        let asset_root = temp_asset_root("texture-cache");
        let texture_dir = asset_root.join("textures");
        fs::create_dir_all(&texture_dir).unwrap();

        let texture_path = texture_dir.join("player.png");
        let first_image = image::RgbaImage::from_raw(1, 1, vec![255, 0, 0, 255]).unwrap();
        first_image.save(&texture_path).unwrap();

        let mut manager = AssetManager::with_root(test_manifest(), asset_root);
        let id = AssetId::new("textures/player");

        assert_eq!(
            manager.load_texture_data(&id).unwrap().rgba,
            vec![255, 0, 0, 255]
        );
        assert_eq!(manager.cached_texture_count(), 1);

        let second_image = image::RgbaImage::from_raw(1, 1, vec![0, 255, 0, 255]).unwrap();
        second_image.save(&texture_path).unwrap();

        assert_eq!(
            manager.load_texture_data(&id).unwrap().rgba,
            vec![255, 0, 0, 255]
        );
        assert!(manager.unload_texture_data(&id));
        assert_eq!(manager.cached_texture_count(), 0);
        assert_eq!(
            manager.load_texture_data(&id).unwrap().rgba,
            vec![0, 255, 0, 255]
        );
    }

    #[test]
    fn test_unload_missing_assets_returns_false() {
        let mut manager = AssetManager::new(test_manifest());

        assert!(!manager.unload_shader_source(&AssetId::new("shaders/missing")));
        assert!(!manager.unload_texture_data(&AssetId::new("textures/missing")));
    }

    #[test]
    fn test_unload_all_clears_loaded_assets() {
        let asset_root = temp_asset_root("unload-all");
        let shader_dir = asset_root.join("shaders");
        let texture_dir = asset_root.join("textures");
        fs::create_dir_all(&shader_dir).unwrap();
        fs::create_dir_all(&texture_dir).unwrap();
        fs::write(shader_dir.join("colored.wgsl"), "shader").unwrap();

        let image = image::RgbaImage::from_raw(1, 1, vec![255, 255, 255, 255]).unwrap();
        image.save(texture_dir.join("player.png")).unwrap();

        let mut manager = AssetManager::with_root(test_manifest(), asset_root);

        manager
            .load_shader_source(&AssetId::new("shaders/colored"))
            .unwrap();
        manager
            .load_texture_data(&AssetId::new("textures/player"))
            .unwrap();

        assert_eq!(manager.cached_shader_count(), 1);
        assert_eq!(manager.cached_texture_count(), 1);

        manager.unload_all();

        assert_eq!(manager.cached_shader_count(), 0);
        assert_eq!(manager.cached_texture_count(), 0);
    }

    #[test]
    fn test_reload_unchanged_assets_returns_empty_list() {
        let asset_root = temp_asset_root("reload-unchanged");
        let shader_dir = asset_root.join("shaders");
        let texture_dir = asset_root.join("textures");
        fs::create_dir_all(&shader_dir).unwrap();
        fs::create_dir_all(&texture_dir).unwrap();

        fs::write(shader_dir.join("colored.wgsl"), "shader").unwrap();
        save_test_texture(&texture_dir.join("player.png"), [255, 255, 255, 255]);

        let mut manager = AssetManager::with_root(test_manifest(), asset_root);

        manager
            .load_shader_source(&AssetId::new("shaders/colored"))
            .unwrap();
        manager
            .load_texture_data(&AssetId::new("textures/player"))
            .unwrap();

        assert!(manager.reload_changed_assets().unwrap().is_empty());
    }

    #[test]
    fn test_reload_changed_shader_updates_cache() {
        let asset_root = temp_asset_root("reload-shader");
        let shader_dir = asset_root.join("shaders");
        fs::create_dir_all(&shader_dir).unwrap();

        let shader_path = shader_dir.join("colored.wgsl");
        fs::write(&shader_path, "first").unwrap();

        let mut manager = AssetManager::with_root(test_manifest(), asset_root);
        let id = AssetId::new("shaders/colored");

        assert_eq!(manager.load_shader_source(&id).unwrap(), "first");

        let previous_modified = fs::metadata(&shader_path).unwrap().modified().unwrap();
        rewrite_until_modified_after(&shader_path, previous_modified, || {
            fs::write(&shader_path, "second").unwrap();
        });

        assert_eq!(
            manager.reload_changed_assets().unwrap(),
            vec![ReloadedAsset {
                id: id.clone(),
                kind: AssetKind::Shader,
            }]
        );
        assert_eq!(manager.load_shader_source(&id).unwrap(), "second");
    }

    #[test]
    fn test_reload_changed_texture_updates_cache() {
        let asset_root = temp_asset_root("reload-texture");
        let texture_dir = asset_root.join("textures");
        fs::create_dir_all(&texture_dir).unwrap();

        let texture_path = texture_dir.join("player.png");
        save_test_texture(&texture_path, [255, 0, 0, 255]);

        let mut manager = AssetManager::with_root(test_manifest(), asset_root);
        let id = AssetId::new("textures/player");

        assert_eq!(
            manager.load_texture_data(&id).unwrap().rgba,
            vec![255, 0, 0, 255]
        );

        let previous_modified = fs::metadata(&texture_path).unwrap().modified().unwrap();
        rewrite_until_modified_after(&texture_path, previous_modified, || {
            save_test_texture(&texture_path, [0, 255, 0, 255]);
        });

        assert_eq!(
            manager.reload_changed_assets().unwrap(),
            vec![ReloadedAsset {
                id: id.clone(),
                kind: AssetKind::Texture,
            }]
        );
        assert_eq!(
            manager.load_texture_data(&id).unwrap().rgba,
            vec![0, 255, 0, 255]
        );
    }
}
