use serde::{Deserialize, Serialize};
use std::{fmt, marker::PhantomData};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AssetId(pub String);

impl AssetId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for AssetId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Handle<T> {
    id: AssetId,
    #[serde(skip)]
    _marker: PhantomData<T>,
}

impl<T> Handle<T> {
    pub fn new(id: AssetId) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }

    pub fn id(&self) -> &AssetId {
        &self.id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextureAsset;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShaderAsset;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AssetKind {
    Shader,
    Texture,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asset_id_exposes_string_value() {
        let id = AssetId::new("textures/player");

        assert_eq!(id.as_str(), "textures/player");
        assert_eq!(id.to_string(), "textures/player");
    }

    #[test]
    fn test_handle_preserves_asset_id() {
        let handle = Handle::<TextureAsset>::new(AssetId::new("textures/player"));

        assert_eq!(handle.id().as_str(), "textures/player");
    }

    #[test]
    fn test_typed_handles_do_not_mix_asset_types() {
        let texture = Handle::<TextureAsset>::new(AssetId::new("shared/id"));
        let shader = Handle::<ShaderAsset>::new(AssetId::new("shared/id"));

        assert_eq!(texture.id(), shader.id());
    }
}
