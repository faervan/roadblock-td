use bevy::{color::palettes::css::RED, prelude::*};

use crate::{
    app_state::GameState,
    enemy::{Enemy, EnemyGoal, PathChangedEvent},
    game_loop::{Currency, GameStatistics},
    grid::{Grid, TILE_SIZE},
    health::Health,
};

use super::Tower;

pub struct TowerAttackPlugin;

impl Plugin for TowerAttackPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Projectile>();
        app.add_systems(
            Update,
            (shoot, move_projectile, projectile_damage)
                .run_if(in_state(GameState::Running)),
        );
    }
}

#[derive(Reflect, Component)]
#[reflect(Component)]
#[require(Transform)]
pub struct Projectile {
    speed: f32,
    damage: isize,
    target: Entity,
}

fn shoot(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut tower: Query<(&mut Tower, &Transform)>,
    goal: Single<&Transform, With<EnemyGoal>>,
    enemy: Query<(&Transform, Entity), With<Enemy>>,
    time: Res<Time>,
) {
    for (mut tower, tower_transform) in tower.iter_mut() {
        tower.attack_timer.tick(time.delta());
        if !tower.attack_timer.finished() {
            continue;
        }

        let mut closest_dist = None;
        let mut closest_enemy = None;

        for (enemy_transform, entity) in enemy.iter() {
            let goal_dist = goal
                .translation
                .distance_squared(enemy_transform.translation);

            let tower_dist = tower_transform
                .translation
                .distance(enemy_transform.translation);

            if tower_dist > tower.range() {
                continue;
            }

            if closest_dist.is_none_or(|x| x > goal_dist) {
                closest_dist = Some(goal_dist);
                closest_enemy = Some(entity);
            }
        }

        if let Some(closest) = closest_enemy {
            tower.attack_timer.reset();
            commands.spawn((
                Name::new("Projectile"),
                Mesh2d(meshes.add(Circle::new(5.0))),
                MeshMaterial2d(materials.add(ColorMaterial::from_color(RED))),
                Projectile {
                    speed: 500.0,
                    damage: tower.strength(),
                    target: closest,
                },
                Transform {
                    translation: Vec3 {
                        //will need to handle the rotation here at some point but I am lazy and the only attacking tower we have rn is symmetrical
                        x: tower_transform.translation.x
                            + tower.offset().0 as f32 * TILE_SIZE
                            + TILE_SIZE * 0.5,
                        y: tower_transform.translation.y
                            + tower.offset().1 as f32 * TILE_SIZE
                            + TILE_SIZE * 0.5,
                        z: 3.0,
                    },
                    ..default()
                },
            ));
        }
    }
}

fn move_projectile(
    mut commands: Commands,
    mut projectile: Query<(&mut Transform, &Projectile, Entity)>,
    enemy: Query<&Transform, (With<Enemy>, Without<Projectile>)>,
    time: Res<Time>,
) {
    for (mut transform, projectile, entity) in projectile.iter_mut() {
        if let Ok(target) = enemy.get(projectile.target) {
            transform.translation = transform
                .translation
                .move_towards(target.translation, projectile.speed * time.delta_secs());
        } else {
            warn!("target enemy no longer exists, despawning projectile");
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn projectile_damage(
    mut commands: Commands,
    projectile: Query<(&Transform, &Projectile, Entity)>,
    mut enemy: Query<(&Transform, &mut Health, Entity, &Enemy)>,
    mut stats: ResMut<GameStatistics>,
    mut currency: ResMut<Currency>,
    mut grid: ResMut<Grid>,
    mut path_change: EventWriter<PathChangedEvent>,
) {
    for (projectile_transform, projectile, projectile_entity) in projectile.iter() {
        for (enemy_transform, mut health, enemy_entity, enemy) in enemy.iter_mut() {
            if projectile_transform
                .translation
                .distance(enemy_transform.translation)
                < TILE_SIZE * 0.5
            {
                **health -= projectile.damage;
                if **health <= 0 {
                    commands.entity(enemy_entity).despawn_recursive();
                    **currency += enemy.reward();
                    stats.enemies_killed += 1;
                    stats.money_earned += enemy.reward();
                    match grid.death_count.get_mut(&enemy.current) {
                        Some(count) => {
                            *count += 1.;
                            path_change
                                .send(PathChangedEvent::now_blocked(vec![enemy.current]));
                        }
                        None => {
                            grid.death_count.insert(enemy.current, 1.);
                        }
                    }
                }
                commands.entity(projectile_entity).despawn_recursive();
            }
        }
    }
}
