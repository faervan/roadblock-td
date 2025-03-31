use std::time::Duration;

use attack::TowerAttackPlugin;
use bevy::prelude::*;
use placing::TowerPlacingPlugin;

pub use placing::{SelectedTower, place_tower};

use crate::{
    Orientation,
    grid::{Grid, GridPos, TILE_SIZE},
};

mod attack;
mod placing;

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Tower>();
        app.add_plugins((TowerPlacingPlugin, TowerAttackPlugin));
    }
}

#[derive(Reflect, Component, Clone, Deref, DerefMut)]
#[reflect(Component)]
pub struct Tower {
    #[deref]
    pub variant: TowerType,
    attack_timer: Timer,
    pub orientation: Orientation,
}

#[derive(Reflect, Clone, Copy)]
pub enum TowerType {
    Wall,
    SpikedWall,
    Canon,
}

impl Tower {
    pub fn new(variant: TowerType, orientation: Orientation) -> Self {
        Self {
            variant,
            orientation,
            attack_timer: Timer::new(variant.fire_cooldown(), TimerMode::Once),
        }
    }

    fn fill_grid(&self, origin: &GridPos, grid: &mut Grid, entity: Entity) -> Vec<GridPos> {
        let mut blocked = vec![];
        grid.tower_origins.insert(entity, *origin);
        // Add entity to every coordinate it covers
        let (rows, cols) = self.size();
        for i in 0..rows {
            for j in 0..cols {
                let pos = GridPos::new(origin.row + j, origin.col + i);
                grid.towers.insert(pos, entity);
                blocked.push(pos);
            }
        }
        blocked
    }

    pub fn clear_grid(&self, grid: &mut Grid, entity: Entity) -> Vec<GridPos> {
        let mut freed = vec![];
        let Some(origin) = grid.tower_origins.remove(&entity) else {
            return vec![];
        };
        let (rows, cols) = self.size();
        for i in 0..rows {
            for j in 0..cols {
                let pos = GridPos::new(origin.row + j, origin.col + i);
                grid.towers.remove(&pos);
                freed.push(pos);
            }
        }
        freed
    }

    pub fn size(&self) -> (isize, isize) {
        let size = self.variant.size();
        // Flip Dimensions of the tower in case of rotation
        match self.orientation.is_horizontal() {
            true => (size.1, size.0),
            false => size,
        }
    }
}

impl TowerType {
    //temp values as balancing cannot happen until a basic gameplay loop is in place
    fn max_hp(&self) -> isize {
        match self {
            TowerType::Wall => 100,
            TowerType::SpikedWall => 100,
            TowerType::Canon => 80,
        }
    }

    pub fn size(&self) -> (isize, isize) {
        match self {
            TowerType::Wall => (4, 1),
            TowerType::SpikedWall => (4, 1),
            TowerType::Canon => (3, 3),
        }
    }

    fn offset(&self) -> (isize, isize) {
        match self {
            TowerType::Wall => (1, 0),
            TowerType::SpikedWall => (1, 0),
            TowerType::Canon => (1, 1),
        }
    }

    fn range(&self) -> f32 {
        match self {
            TowerType::Canon => TILE_SIZE * 10.0,
            _ => 0.0,
        }
    }

    fn strength(&self) -> isize {
        match self {
            TowerType::Canon => 15,
            _ => 0,
        }
    }

    fn fire_cooldown(&self) -> Duration {
        match self {
            TowerType::Canon => Duration::from_secs(1),
            _ => Duration::ZERO,
        }
    }

    fn _contact_damage(&self) -> isize {
        match self {
            TowerType::SpikedWall => 5,
            _ => 0,
        }
    }

    fn _contact_damage_cooldown(&self) -> Duration {
        match self {
            TowerType::SpikedWall => Duration::from_secs(1),
            _ => Duration::ZERO,
        }
    }
}
