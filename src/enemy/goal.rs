use bevy::prelude::*;

use crate::{
    Health,
    app_state::InGame,
    grid::{COLUMNS, Grid, GridPos, ROWS, grid_to_world_coords, spawn_grid},
};

pub struct EnemyGoalPlugin;

impl Plugin for EnemyGoalPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<EnemyGoal>()
            .add_systems(OnEnter(InGame), spawn_enemy_goal.after(spawn_grid));
    }
}

#[derive(Reflect, Component, Clone, Copy)]
#[reflect(Component)]
pub enum EnemyGoal {
    Heart,
}

impl EnemyGoal {
    fn max_hp(&self) -> isize {
        match self {
            EnemyGoal::Heart => 200,
        }
    }

    pub fn thorn_damage(&self) -> isize {
        match self {
            EnemyGoal::Heart => 10,
        }
    }

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

    fn add_unbuildable_surroundings(&self, origin: &GridPos, grid: &mut Grid) {
        grid.unbuildable.extend(
            match self {
                EnemyGoal::Heart => [
                    [0, -1],
                    [1, -1],
                    [0, 2],
                    [1, 2],
                    [-1, -1],
                    [-1, 0],
                    [-1, 1],
                    [-1, 2],
                    [2, -1],
                    [2, 0],
                    [2, 1],
                    [2, 2],
                ],
            }
            .into_iter()
            .map(|offset| origin + offset)
            .filter(|pos| pos.inside_grid_bounds()),
        );
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
            Health(goal.max_hp()),
            Sprite::from_image(asset_server.load(goal.sprite())),
            Transform {
                translation: grid_to_world_coords(grid_pos).extend(1.0) + goal.offset(),
                scale: goal.scale(),
                ..default()
            },
            goal,
        ))
        .id();
    goal.add_unbuildable_surroundings(&grid_pos, &mut grid);
    grid.enemy_goals.insert(grid_pos, entity);
    for pos in goal.other_tiles(&grid_pos) {
        grid.enemy_goals.insert(pos, entity);
    }
}
