pub mod assets;
pub mod render;
pub mod runtime;
pub mod scene;

pub use assets::{
    AssetError, AssetId, AssetManager, AssetManifest, Handle, ShaderAsset, ShaderAssetEntry,
    TextureAsset, TextureAssetEntry, TextureData,
};
pub use render::{RenderCamera, RenderError, RenderScene, RenderSprite, Renderer};
pub use runtime::{
    EngineConfig, FixedStep, FixedSteps, FixedTimestep, FpsCounter, FrameClock, FrameTiming,
    LifecycleEvent, RuntimeError,
};
pub use scene::{EntityId, SceneObjectId, Sprite, Transform};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_texture_data_is_exported_from_crate_root() {
        let texture = TextureData {
            width: 1,
            height: 1,
            rgba: vec![255, 255, 255, 255],
        };

        assert_eq!(texture.width, 1);
        assert_eq!(texture.height, 1);
        assert_eq!(texture.rgba, vec![255, 255, 255, 255]);
    }
}
