use super::components::PickupReward;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnemyKind {
    Basic,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EnemyDefinition {
    pub health: f32,
    pub speed: f32,
    pub contact_damage_per_second: f32,
    pub collider_radius: f32,
    pub sprite_size: [f32; 2],
    pub sprite_color: [f32; 4],
    pub death_reward: PickupReward,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WeaponKind {
    Wand,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WeaponDefinition {
    pub cooldown_secs: f32,
    pub projectile_speed: f32,
    pub projectile_damage: f32,
    pub projectile_size: [f32; 2],
    pub projectile_color: [f32; 4],
    pub projectile_collider_radius: f32,
    pub projectile_lifetime_secs: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GameCatalog {
    pub basic_enemy: EnemyDefinition,
    pub wand: WeaponDefinition,
}

impl Default for GameCatalog {
    fn default() -> Self {
        Self {
            basic_enemy: EnemyDefinition {
                health: 20.0,
                speed: 1.5,
                contact_damage_per_second: 10.0,
                collider_radius: 0.35,
                sprite_size: [1.0, 1.0],
                sprite_color: [0.9, 0.35, 0.55, 1.0],
                death_reward: PickupReward::Experience(1),
            },
            wand: WeaponDefinition {
                cooldown_secs: 0.5,
                projectile_speed: 8.0,
                projectile_damage: 10.0,
                projectile_size: [0.25, 0.25],
                projectile_color: [0.35, 0.75, 1.0, 1.0],
                projectile_collider_radius: 0.15,
                projectile_lifetime_secs: 2.0,
            },
        }
    }
}

impl GameCatalog {
    pub fn enemy(&self, kind: EnemyKind) -> &EnemyDefinition {
        match kind {
            EnemyKind::Basic => &self.basic_enemy,
        }
    }

    pub fn weapon(&self, kind: WeaponKind) -> &WeaponDefinition {
        match kind {
            WeaponKind::Wand => &self.wand,
        }
    }
}
