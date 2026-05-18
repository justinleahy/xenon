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
    pub material: Material,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RenderScene<'a> {
    pub camera: RenderCamera,
    pub sprites: &'a [RenderSprite],
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Material {
    pub base_color: [f32; 4],
}

impl Material {
    pub const WHITE: Self = Self {
        base_color: [1.0, 1.0, 1.0, 1.0],
    };

    pub const BLACK: Self = Self {
        base_color: [0.0, 0.0, 0.0, 1.0],
    };

    pub const fn from_color(base_color: [f32; 4]) -> Self {
        Self { base_color }
    }
}

impl Default for Material {
    fn default() -> Self {
        Self::WHITE
    }
}
