#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RenderCamera {
    pub position: [f32; 2],
    pub pixels_per_world_unit: f32,
}

impl Default for RenderCamera {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0],
            pixels_per_world_unit: 32.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RenderSprite {
    pub position: [f32; 2],
    pub size: [f32; 2],
    pub color: [f32; 4],
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RenderScene<'a> {
    pub camera: RenderCamera,
    pub sprites: &'a [RenderSprite],
}
