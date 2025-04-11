use std::time::Duration;

use bevy::prelude::*;

use crate::{
    RngResource,
    app_state::GameState,
    game_loop::{SpawnerInfo, WaveInfo, WaveStart},
    grid::{Grid, GridPos, grid_to_world_coords},
    health::Health,
};

use super::{Enemy, EnemyType};

pub struct EnemySpawnerPlugin;

impl Plugin for EnemySpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<EnemySpawn>()
            .register_type::<SpawnQueue>()
            .add_systems(
                Update,
                (
                    spawn_enemy_spawners.run_if(on_event::<WaveStart>),
                    spawn_enemies.run_if(in_state(GameState::Running)),
                ),
            );
    }
}

#[derive(Reflect, Component)]
#[reflect(Component)]
struct EnemySpawn {
    variant: EnemySpawnType,
    pos: GridPos,
    #[reflect(ignore)]
    info: SpawnerInfo,
}

#[derive(Reflect, Component)]
#[reflect(Component)]
struct SpawnQueue {
    enemies: Vec<EnemyType>,
    timer: Timer,
}

impl SpawnQueue {
    fn new(info: &SpawnerInfo, wave: usize) -> Self {
        Self {
            enemies: (info.enemies)(wave),
            timer: Timer::new(
                Duration::from_secs_f32((info.interval)(wave)),
                TimerMode::Repeating,
            ),
        }
    }
}

#[derive(Reflect, Debug)]
enum EnemySpawnType {
    RedTower,
}

impl EnemySpawn {
    fn new(variant: EnemySpawnType, pos: GridPos, info: SpawnerInfo) -> Self {
        Self { variant, pos, info }
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
    mut event: EventReader<WaveStart>,
    mut commands: Commands,
    mut grid: ResMut<Grid>,
    asset_server: Res<AssetServer>,
    mut rng: ResMut<RngResource>,
    spawner_query: Query<(Entity, &EnemySpawn)>,
) {
    let Some(wave) = event.read().next() else {
        error!("Failed to read WaveStart event!");
        return;
    };

    for info in &wave.new_spawners {
        loop {
            let grid_pos = GridPos::random(&mut rng);

            let spawner = EnemySpawn::new(EnemySpawnType::RedTower, grid_pos, *info);
            let other = spawner.other_tiles();

            if spawner.pos.distance_to_closest(&grid.enemy_goals) < 35
                || other.iter().any(|pos| !grid.is_free(pos))
            {
                continue;
            }

            spawner.add_unbuildable_surroundings(&mut grid);

            let entity = commands
                .spawn((
                    Name::new(format!("Spawner: {:?}", spawner.variant)),
                    Sprite::from_image(asset_server.load(spawner.sprite())),
                    Transform {
                        translation: grid_to_world_coords(grid_pos).extend(1.)
                            + spawner.offset(),
                        scale: spawner.scale(),
                        ..Default::default()
                    },
                    spawner,
                    SpawnQueue::new(info, **wave),
                ))
                .id();

            grid.enemy_spawners.insert(grid_pos, entity);
            for tile in other.into_iter() {
                grid.enemy_spawners.insert(tile, entity);
            }
            break;
        }
    }

    for (entity, spawner) in &spawner_query {
        commands
            .entity(entity)
            .insert(SpawnQueue::new(&spawner.info, **wave));
    }
}

fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut spawners: Query<(Entity, &EnemySpawn, &mut SpawnQueue)>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut wave: ResMut<WaveInfo>,
) {
    for (entity, spawner, mut queue) in &mut spawners {
        queue.timer.tick(time.delta());
        if !queue.timer.just_finished() {
            continue;
        }

        let Some(enemy_ty) = queue.enemies.pop() else {
            commands.entity(entity).remove::<SpawnQueue>();
            wave.done_this_wave += 1;
            return;
        };
        let enemy = Enemy::new(spawner.pos, enemy_ty);

        commands.spawn((
            Name::new(format!("Enemy: {:?}", enemy.variant)),
            Health::new(enemy.max_hp(), enemy.health_bar_offset()),
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
