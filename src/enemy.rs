use bevy::{prelude::*, window::PrimaryWindow};

use crate::grid::{
    COLUMNS, Grid, GridPos, ROWS, TILE_SIZE, Tile, TileType, grid_to_world_coords,
    world_to_grid_coords,
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Enemy>()
            .register_type::<EnemyPath>()
            .add_systems(Startup, spawn_enemy_goal)
            .add_systems(Update, (spawn_enemies, move_enemies));
    }
}

#[derive(Reflect, Component)]
#[reflect(Component)]
pub struct Enemy {
    pub current: GridPos,
    pub goal: GridPos,
}

#[derive(Reflect, Component)]
#[reflect(Component)]
pub struct EnemyPath {
    steps: Vec<GridPos>,
    next: Option<Vec3>,
}

impl EnemyPath {
    pub fn new(steps: Vec<GridPos>) -> Self {
        Self { steps, next: None }
    }
}

#[derive(Reflect, Component)]
#[reflect(Component)]
#[require(Tile(enemy_goal))]
struct EnemyGoal;

fn enemy_goal() -> Tile {
    Tile {
        pos: GridPos::default(),
        tile_type: TileType::EnemyGoal,
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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
                    commands.spawn((
                        Enemy {
                            current: grid_pos,
                            goal: *grid.enemy_goal.iter().next().unwrap().0,
                        },
                        Mesh2d(meshes.add(Circle::new(5.))),
                        MeshMaterial2d(materials.add(Color::hsl(0., 1., 0.5))),
                        Transform {
                            translation: grid_to_world_coords(grid_pos).extend(1.0),
                            ..default()
                        },
                    ));
                }
            }
        } else {
            warn!("Unable to get Cursor Position {:?}", world_pos.unwrap_err())
        }
    }
}

fn move_enemies(
    mut query: Query<(&mut EnemyPath, &mut Enemy, &mut Transform, Entity)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (mut path, mut enemy, mut pos, entity) in &mut query {
        let next = match path.next {
            Some(tile) => tile,
            None => {
                if let Some(tile) = path.steps.pop() {
                    enemy.current = tile;
                    let next = grid_to_world_coords(tile).extend(1.);
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
