use super::components::PickupReward;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnemyKind {
    Basic,
    Fast,
    Tank,
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
    Pistol,
    Smg,
    Shotgun,
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
    pub fast_enemy: EnemyDefinition,
    pub tank_enemy: EnemyDefinition,
    pub pistol: WeaponDefinition,
    pub smg: WeaponDefinition,
    pub shotgun: WeaponDefinition,
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
            fast_enemy: EnemyDefinition {
                health: 12.0,
                speed: 2.6,
                contact_damage_per_second: 8.0,
                collider_radius: 0.28,
                sprite_size: [0.8, 0.8],
                sprite_color: [0.95, 0.75, 0.25, 1.0],
                death_reward: PickupReward::Experience(1),
            },
            tank_enemy: EnemyDefinition {
                health: 60.0,
                speed: 0.85,
                contact_damage_per_second: 18.0,
                collider_radius: 0.55,
                sprite_size: [1.4, 1.4],
                sprite_color: [0.55, 0.45, 0.95, 1.0],
                death_reward: PickupReward::Experience(3),
            },
            pistol: WeaponDefinition {
                cooldown_secs: 0.5,
                projectile_speed: 10.0,
                projectile_damage: 10.0,
                projectile_size: [0.2, 0.12],
                projectile_color: [0.95, 0.9, 0.35, 1.0],
                projectile_collider_radius: 0.12,
                projectile_lifetime_secs: 1.5,
            },
            smg: WeaponDefinition {
                cooldown_secs: 0.16,
                projectile_speed: 11.5,
                projectile_damage: 5.0,
                projectile_size: [0.16, 0.1],
                projectile_color: [0.45, 0.8, 1.0, 1.0],
                projectile_collider_radius: 0.1,
                projectile_lifetime_secs: 1.25,
            },
            shotgun: WeaponDefinition {
                cooldown_secs: 0.9,
                projectile_speed: 8.0,
                projectile_damage: 28.0,
                projectile_size: [0.38, 0.22],
                projectile_color: [1.0, 0.55, 0.25, 1.0],
                projectile_collider_radius: 0.24,
                projectile_lifetime_secs: 0.9,
            },
        }
    }
}

impl GameCatalog {
    pub fn enemy(&self, kind: EnemyKind) -> &EnemyDefinition {
        match kind {
            EnemyKind::Basic => &self.basic_enemy,
            EnemyKind::Fast => &self.fast_enemy,
            EnemyKind::Tank => &self.tank_enemy,
        }
    }

    pub fn weapon(&self, kind: WeaponKind) -> &WeaponDefinition {
        match kind {
            WeaponKind::Pistol => &self.pistol,
            WeaponKind::Smg => &self.smg,
            WeaponKind::Shotgun => &self.shotgun,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enemy_returns_basic_enemy_definition() {
        let catalog = GameCatalog::default();

        assert_eq!(catalog.enemy(EnemyKind::Basic), &catalog.basic_enemy);
    }

    #[test]
    fn test_enemy_returns_fast_enemy_definition() {
        let catalog = GameCatalog::default();

        assert_eq!(catalog.enemy(EnemyKind::Fast), &catalog.fast_enemy);
    }

    #[test]
    fn test_enemy_returns_tank_enemy_definition() {
        let catalog = GameCatalog::default();

        assert_eq!(catalog.enemy(EnemyKind::Tank), &catalog.tank_enemy);
    }

    #[test]
    fn test_weapon_returns_pistol_definition() {
        let catalog = GameCatalog::default();

        assert_eq!(catalog.weapon(WeaponKind::Pistol), &catalog.pistol);
    }

    #[test]
    fn test_weapon_returns_smg_definition() {
        let catalog = GameCatalog::default();

        assert_eq!(catalog.weapon(WeaponKind::Smg), &catalog.smg);
    }

    #[test]
    fn test_weapon_returns_shotgun_definition() {
        let catalog = GameCatalog::default();

        assert_eq!(catalog.weapon(WeaponKind::Shotgun), &catalog.shotgun);
    }
}
