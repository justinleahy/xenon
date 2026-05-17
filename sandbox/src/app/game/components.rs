#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    pub position: [f32; 2],
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sprite {
    pub size: [f32; 2],
    pub color: [f32; 4],
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Projectile {
    pub previous_position: [f32; 2],
    pub velocity: [f32; 2],
    pub lifetime_secs: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CircleCollider {
    pub radius: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Damage {
    pub amount: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PickupReward {
    Experience(u32),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DeathDrop {
    pub reward: PickupReward,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pickup {
    pub reward: PickupReward,
}
