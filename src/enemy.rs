use std::time::Duration;

use rand_core::RngCore;

use bevy::{prelude::*, utils::HashSet, window::PrimaryWindow};
use bevy_rand::{global::GlobalEntropy, prelude::WyRand};

use crate::{
    Orientation,
    animation::AnimationConfig,
    grid::{COLUMNS, Grid, GridPos, ROWS, TILE_SIZE, grid_to_world_coords, world_to_grid_coords},
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Enemy>()
            .register_type::<EnemyPath>()
            .register_type::<EnemyGoal>()
            .register_type::<EnemySpawn>()
            .add_systems(
                Startup,
                (
                    spawn_enemy_goal,
                    spawn_enemy_spawners.after(spawn_enemy_goal),
                ),
            )
            .add_systems(Update, (spawn_enemies, move_enemies));
    }
}

#[derive(Reflect, Component)]
#[reflect(Component)]
pub struct Enemy {
    pub current: GridPos,
    pub goal: GridPos,
    variant: EnemyType,
    orientation: Orientation,
}

#[derive(Reflect)]
enum EnemyType {
    Skeleton,
}

impl Enemy {
    fn new(current: GridPos, goal: GridPos, variant: EnemyType) -> Self {
        Self {
            current,
            goal,
            variant,
            orientation: Orientation::default(),
        }
    }

    fn sprite_sheet(&self) -> &str {
        match self.variant {
            EnemyType::Skeleton => "sprites/enemies/BODY_skeleton.png",
        }
    }

    fn layout(&self, layouts: &mut Assets<TextureAtlasLayout>) -> TextureAtlas {
        match self.variant {
            EnemyType::Skeleton => TextureAtlas {
                layout: layouts.add(TextureAtlasLayout::from_grid(
                    UVec2::splat(64),
                    9,
                    4,
                    None,
                    None,
                )),
                index: self.sprite_indices().0,
            },
        }
    }

    fn offset(&self) -> Vec3 {
        match self.variant {
            EnemyType::Skeleton => Vec3::new(0., 10., 0.),
        }
    }

    fn scale(&self) -> Vec3 {
        match self.variant {
            EnemyType::Skeleton => Vec3::splat(0.6),
        }
    }

    fn animation_config(&self) -> AnimationConfig {
        match self.variant {
            EnemyType::Skeleton => {
                let (first, last) = self.sprite_indices();
                AnimationConfig::new(first, last, 10)
            }
        }
    }

    /// Returns (first_sprite_index, last_sprite_index)
    fn sprite_indices(&self) -> (usize, usize) {
        match self.variant {
            EnemyType::Skeleton => match self.orientation {
                Orientation::Up => (0, 8),
                Orientation::Down => (18, 26),
                Orientation::Left => (9, 17),
                Orientation::Right => (27, 35),
            },
        }
    }
}

#[derive(Reflect, Component)]
#[reflect(Component)]
pub struct EnemyPath {
    pub steps: Vec<GridPos>,
    next: Option<Vec3>,
}

impl EnemyPath {
    pub fn new(steps: Vec<GridPos>) -> Self {
        Self { steps, next: None }
    }
}

#[derive(Reflect, Component)]
#[reflect(Component)]
struct EnemySpawn(Timer);

#[derive(Reflect, Component)]
#[reflect(Component)]
struct EnemyGoal;

fn spawn_enemy_spawners(
    mut commands: Commands,
    mut grid: ResMut<Grid>,
    asset_server: Res<AssetServer>,
    mut rng: GlobalEntropy<WyRand>,
) {
    let mut positions = HashSet::new();
    let goal = grid.enemy_goal.iter().next().unwrap().0;
    while positions.len() != 5 {
        let [row, col] = [
            //rng.random_range(0..(ROWS - 1) as usize) as isize,
            //rng.random_range(0..(COLUMNS - 1) as usize) as isize,
            (rand::random::<u32>() as isize % ROWS),
            (rand::random::<u32>() as isize % COLUMNS),
        ];
        if ((goal.row - row).abs().pow(2) + (goal.col - col).abs().pow(2)).isqrt()
            >= TILE_SIZE as isize * 20
        {
            positions.insert(GridPos::new(row, col));
        }
    }
    for pos in &positions {
        let entity = commands
            .spawn((
                EnemySpawn(Timer::new(Duration::from_secs(1), TimerMode::Repeating)),
                Sprite::from_image(asset_server.load("sprites/spawners/Asset 24.png")),
                Transform::from_translation(grid_to_world_coords(*pos).extend(1.)),
            ))
            .id();
        grid.enemy_spawn.insert(*pos, entity);
    }
}

fn spawn_enemy_goal(mut commands: Commands, mut grid: ResMut<Grid>) {
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

fn spawn_enemies(
    mut commands: Commands,
    window: Single<&Window, With<PrimaryWindow>>,
    cam: Single<(&Camera, &GlobalTransform)>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    grid: Res<Grid>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    if !mouse_input.just_pressed(MouseButton::Right) {
        return;
    }

    let mouse_pos = window.cursor_position();

    if let Some(mouse_pos) = mouse_pos {
        let (camera, cam_transform) = *cam;

        let world_pos = camera.viewport_to_world_2d(cam_transform, mouse_pos);
        if let Ok(world_pos) = world_pos {
            if let Some(grid_pos) = world_to_grid_coords(world_pos) {
                if grid.is_free(&grid_pos) {
                    let enemy = Enemy::new(
                        grid_pos,
                        *grid.enemy_goal.iter().next().unwrap().0,
                        EnemyType::Skeleton,
                    );
                    commands.spawn((
                        Sprite {
                            image: asset_server.load(enemy.sprite_sheet()),
                            texture_atlas: Some(enemy.layout(&mut texture_atlas_layouts)),
                            ..Default::default()
                        },
                        Transform {
                            translation: grid_to_world_coords(grid_pos).extend(2.) + enemy.offset(),
                            scale: enemy.scale(),
                            ..default()
                        },
                        enemy.animation_config(),
                        enemy,
                    ));
                }
            }
        } else {
            warn!("Unable to get Cursor Position {:?}", world_pos.unwrap_err())
        }
    }
}

fn move_enemies(
    mut query: Query<(
        &mut EnemyPath,
        &mut Enemy,
        &mut AnimationConfig,
        &mut Sprite,
        &mut Transform,
        Entity,
    )>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (mut path, mut enemy, mut animation, mut sprite, mut pos, entity) in &mut query {
        let next = match path.next {
            Some(tile) => tile,
            None => {
                if let Some(tile) = path.steps.pop() {
                    let orientation =
                        match (tile.row > enemy.current.row, tile.col > enemy.current.col) {
                            (true, false) => Orientation::Up,
                            (false, true) => Orientation::Right,
                            _ => match tile.row < enemy.current.row {
                                true => Orientation::Down,
                                false => Orientation::Left,
                            },
                        };
                    if orientation != enemy.orientation {
                        enemy.orientation = orientation;
                        *animation = enemy.animation_config();
                        if let Some(atlas) = &mut sprite.texture_atlas {
                            atlas.index = enemy.sprite_indices().0;
                        }
                    }
                    enemy.current = tile;
                    let next = grid_to_world_coords(tile).extend(2.) + enemy.offset();
                    path.next = Some(next);
                    next
                } else {
                    commands.entity(entity).despawn();
                    return;
                }
            }
        };
        let direction = next - pos.translation;
        pos.translation += direction.normalize() * time.delta_secs() * 150.;
        if pos.translation.distance(next) >= direction.length() {
            path.next = None;
        }
    }
}
