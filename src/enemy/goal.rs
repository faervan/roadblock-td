use bevy::prelude::*;

use crate::grid::{COLUMNS, Grid, GridPos, ROWS, TILE_SIZE, grid_to_world_coords};

pub struct EnemyGoalPlugin;

impl Plugin for EnemyGoalPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<EnemyGoal>()
            .add_systems(Startup, spawn_enemy_goal);
    }
}

#[derive(Reflect, Component)]
#[reflect(Component)]
struct EnemyGoal;

pub fn spawn_enemy_goal(mut commands: Commands, mut grid: ResMut<Grid>) {
    let grid_pos = GridPos::new(ROWS / 2, COLUMNS - 1);
    let entity = commands
        .spawn((
            EnemyGoal,
            Sprite::from_color(Color::hsl(360., 1., 0.5), Vec2::splat(TILE_SIZE)),
            Transform {
                translation: grid_to_world_coords(grid_pos).extend(1.0),
                ..default()
            },
        ))
        .id();
    grid.enemy_goal.insert(grid_pos, entity);
}
