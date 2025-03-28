use bevy::{input::common_conditions::input_just_pressed, prelude::*, window::PrimaryWindow};
use goal::EnemyGoalPlugin;
pub use path_finding::PathChangedEvent;
use path_finding::PathfindingPlugin;
use spawner::EnemySpawnerPlugin;

use crate::{
    Orientation,
    animation::AnimationConfig,
    grid::{Grid, GridPos, grid_to_world_coords, world_to_grid_coords},
};

mod goal;
mod path_finding;
mod spawner;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Enemy>()
            .add_plugins((PathfindingPlugin, EnemySpawnerPlugin, EnemyGoalPlugin))
            .add_systems(
                Update,
                spawn_enemies_manual.run_if(input_just_pressed(MouseButton::Right)),
            );
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

/// Only for development purposes
fn spawn_enemies_manual(
    mut commands: Commands,
    window: Single<&Window, With<PrimaryWindow>>,
    cam: Single<(&Camera, &GlobalTransform)>,
    grid: Res<Grid>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
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
