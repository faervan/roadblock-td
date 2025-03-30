use bevy::prelude::*;

use crate::grid::{COLUMNS, Grid, GridPos, ROWS, grid_to_world_coords};

pub struct EnemyGoalPlugin;

impl Plugin for EnemyGoalPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<EnemyGoal>()
            .add_systems(Startup, spawn_enemy_goal);
    }
}

#[derive(Reflect, Component, Clone, Copy)]
#[reflect(Component)]
enum EnemyGoal {
    Heart,
}

impl EnemyGoal {
    /// Returns all the tiles that belong to the goal, relative to the "origin tile"
    fn other_tiles(&self, origin: &GridPos) -> Vec<GridPos> {
        match self {
            EnemyGoal::Heart => {
                vec![
                    origin + GridPos::new(1, 0),
                    origin + GridPos::new(0, 1),
                    origin + GridPos::new(1, 1),
                ]
            }
        }
    }

    fn sprite(&self) -> &str {
        match self {
            EnemyGoal::Heart => "sprites/goals/heart.png",
        }
    }

    fn offset(&self) -> Vec3 {
        match self {
            EnemyGoal::Heart => Vec3::new(11., 13., 0.),
        }
    }

    fn scale(&self) -> Vec3 {
        match self {
            EnemyGoal::Heart => Vec3::splat(0.75),
        }
    }
}

pub fn spawn_enemy_goal(
    mut commands: Commands,
    mut grid: ResMut<Grid>,
    asset_server: Res<AssetServer>,
) {
    let grid_pos = GridPos::new(ROWS / 2, COLUMNS - 2);
    let goal = EnemyGoal::Heart;
    let entity = commands
        .spawn((
            Sprite::from_image(asset_server.load(goal.sprite())),
            Transform {
                translation: grid_to_world_coords(grid_pos).extend(1.0) + goal.offset(),
                scale: goal.scale(),
                ..default()
            },
            goal,
        ))
        .id();
    grid.enemy_goals.insert(grid_pos, entity);
    for pos in goal.other_tiles(&grid_pos) {
        grid.enemy_goals.insert(pos, entity);
    }
}
