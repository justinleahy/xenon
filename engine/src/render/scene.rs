#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RenderSprite {
    pub position: [f32; 2],
    pub size: [f32; 2],
    pub color: [f32; 4],
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RenderScene<'a> {
    pub sprites: &'a [RenderSprite],
}
