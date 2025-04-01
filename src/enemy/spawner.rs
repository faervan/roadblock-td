use std::time::Duration;

use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};

use crate::{
    Health, RngResource,
    app_state::{AppState, GameState},
    grid::{Grid, GridPos, grid_to_world_coords},
};

use super::{Enemy, EnemyType, goal::spawn_enemy_goal};

pub struct EnemySpawnerPlugin;

impl Plugin for EnemySpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<EnemySpawn>()
            .add_systems(
                OnEnter(AppState::Game),
                spawn_enemy_spawners.after(spawn_enemy_goal),
            )
            .add_systems(Update, spawn_enemies.run_if(in_state(GameState::Running)));
    }
}

#[derive(Reflect, Component, Debug)]
#[reflect(Component)]
struct EnemySpawn {
    variant: EnemySpawnType,
    pos: GridPos,
    timer: Timer,
}

#[derive(Reflect, Debug)]
enum EnemySpawnType {
    RedTower,
}

impl EnemySpawn {
    fn new(variant: EnemySpawnType, pos: GridPos) -> Self {
        Self {
            variant,
            pos,
            timer: Timer::new(Duration::from_secs(8), TimerMode::Repeating),
        }
    }

    /// Returns all the tiles that belong to the spawner, relative to the "origin tile"
    fn other_tiles(&self) -> Vec<GridPos> {
        match self.variant {
            EnemySpawnType::RedTower => {
                vec![
                    self.pos + GridPos::new(1, 0),
                    self.pos + GridPos::new(0, 1),
                    self.pos + GridPos::new(1, 1),
                ]
            }
        }
    }

    fn add_unbuildable_surroundings(&self, grid: &mut Grid) {
        grid.unbuildable.extend(
            match self.variant {
                EnemySpawnType::RedTower => [
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
            .map(|offset| self.pos + offset)
            .filter(|pos| pos.inside_grid_bounds()),
        );
    }

    fn spawn_point(&self) -> Vec2 {
        grid_to_world_coords(self.pos)
            + match self.variant {
                EnemySpawnType::RedTower => Vec2::new(10., 0.),
            }
    }

    fn sprite(&self) -> &str {
        match self.variant {
            EnemySpawnType::RedTower => "sprites/spawners/red_spawner.png",
        }
    }

    fn offset(&self) -> Vec3 {
        match self.variant {
            EnemySpawnType::RedTower => Vec3::new(13., 15., 0.),
        }
    }

    fn scale(&self) -> Vec3 {
        match self.variant {
            EnemySpawnType::RedTower => Vec3::splat(0.8),
        }
    }
}

fn spawn_enemy_spawners(
    mut commands: Commands,
    mut grid: ResMut<Grid>,
    asset_server: Res<AssetServer>,
    mut rng: ResMut<RngResource>,
) {
    let mut origin_tiles = HashMap::new();
    let mut other_tiles = HashSet::new();

    while origin_tiles.len() != 5 {
        let grid_pos = GridPos::random(&mut rng);

        let spawner = EnemySpawn::new(EnemySpawnType::RedTower, grid_pos);
        let other = spawner.other_tiles();

        if spawner.pos.distance_to_closest(&grid.enemy_goals) >= 20
            && !other.iter().any(|pos| other_tiles.contains(pos))
            && !other_tiles.contains(&spawner.pos)
        {
            origin_tiles.insert(grid_pos, spawner);
            other_tiles.extend(other);
        }
    }

    for (pos, spawner) in origin_tiles.into_iter() {
        let other = spawner.other_tiles();
        spawner.add_unbuildable_surroundings(&mut grid);

        let entity = commands
            .spawn((
                Sprite::from_image(asset_server.load(spawner.sprite())),
                Transform {
                    translation: grid_to_world_coords(pos).extend(1.) + spawner.offset(),
                    scale: spawner.scale(),
                    ..Default::default()
                },
                spawner,
            ))
            .id();

        grid.enemy_spawners.insert(pos, entity);
        for tile in other.into_iter() {
            grid.enemy_spawners.insert(tile, entity);
        }
    }
}

fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut spawners: Query<&mut EnemySpawn>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for mut spawner in &mut spawners {
        spawner.timer.tick(time.delta());
        if !spawner.timer.finished() {
            continue;
        }

        let enemy = Enemy::new(spawner.pos, EnemyType::Skeleton);

        commands.spawn((
            Health(enemy.max_hp()),
            Sprite {
                image: asset_server.load(enemy.walk_sprites()),
                texture_atlas: Some(enemy.walk_layout(&mut texture_atlas_layouts)),
                ..Default::default()
            },
            Transform {
                translation: spawner.spawn_point().extend(2.) + enemy.offset(),
                scale: enemy.scale(),
                ..default()
            },
            enemy.walk_animation_config(),
            enemy,
        ));
    }
}
