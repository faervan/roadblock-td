use bevy::prelude::*;

use crate::grid::GridPos;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemies)
            .add_systems(Update, move_enemies);
    }
}

#[derive(Component)]
pub struct Enemy {
    pub current: GridPos,
    pub goal: GridPos,
}

#[derive(Component)]
pub struct EnemyPath(pub Vec<GridPos>);

fn spawn_enemies() {}

fn move_enemies() {}
