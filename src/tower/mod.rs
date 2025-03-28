use std::time::Duration;

use bevy::prelude::*;
use placing::TowerPlacingPlugin;

pub use placing::{SelectedTower, TowerPlaceState, place_tower};

use crate::grid::TILE_SIZE;

mod placing;

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<TowerPlaceState>();
        app.register_type::<Tower>();
        app.add_plugins(TowerPlacingPlugin);
    }
}

#[derive(Reflect, Component, Clone, Copy)]
#[reflect(Component)]
pub enum Tower {
    Wall,
    SpikedWall,
    Canon,
}

impl Tower {
    //temp values as balancing cannot happen until a basic gameplay loop is in place
    fn _max_hp(&self) -> u32 {
        match self {
            Self::Wall => 100,
            Self::SpikedWall => 100,
            Self::Canon => 80,
        }
    }

    pub fn size(&self) -> (isize, isize) {
        match self {
            Self::Wall => (4, 1),
            Self::SpikedWall => (4, 1),
            Self::Canon => (3, 3),
        }
    }

    fn offset(&self) -> (isize, isize) {
        match self {
            Self::Wall => (1, 0),
            Self::SpikedWall => (1, 0),
            Self::Canon => (1, 1),
        }
    }

    fn _range(&self) -> f32 {
        match self {
            Self::Canon => TILE_SIZE * 10.0,
            _ => 0.0,
        }
    }

    fn _strength(&self) -> u32 {
        match self {
            Self::Canon => 15,
            _ => 0,
        }
    }

    fn _fire_cooldown(&self) -> Duration {
        match self {
            Self::Canon => Duration::from_secs(1),
            _ => Duration::ZERO,
        }
    }

    fn _contact_damage(&self) -> u32 {
        match self {
            Self::SpikedWall => 5,
            _ => 0,
        }
    }

    fn _contact_damage_cooldown(&self) -> Duration {
        match self {
            Self::SpikedWall => Duration::from_secs(1),
            _ => Duration::ZERO,
        }
    }
}
