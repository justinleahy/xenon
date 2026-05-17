#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    pub position: [f32; 2],
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}
