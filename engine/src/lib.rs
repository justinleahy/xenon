#[cfg(feature = "assets")]
pub mod assets;
#[cfg(feature = "render")]
pub mod render;
#[cfg(feature = "runtime")]
pub mod runtime;
#[cfg(feature = "scene")]
pub mod scene;

#[cfg(feature = "assets")]
pub use assets::{
    AssetError, AssetId, AssetManager, AssetManifest, Handle, ShaderAsset, ShaderAssetEntry,
    TextureAsset, TextureAssetEntry, TextureData,
};
#[cfg(feature = "render")]
pub use render::{
    Material, RenderCamera, RenderError, RenderScene, RenderSprite, Renderer, TextureResource,
};
#[cfg(feature = "runtime")]
pub use runtime::{
    EngineConfig, FixedStep, FixedSteps, FixedTimestep, FpsCounter, FrameClock, FrameTiming,
    LifecycleEvent, RuntimeError,
};
#[cfg(feature = "scene")]
pub use scene::{EntityId, SceneObjectId, Sprite, Transform};

#[cfg(all(test, feature = "assets"))]
mod asset_feature_tests {
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

#[cfg(all(test, feature = "render"))]
mod render_feature_tests {
    use super::*;

    #[test]
    fn test_texture_resource_is_exported_from_crate_root() {
        fn accepts_texture_resource(_resource: Option<TextureResource>) {}

        accepts_texture_resource(None);
    }
}
