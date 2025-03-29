use bevy::{color::palettes::css::RED, prelude::*};

use crate::enemy::Enemy;

use super::Tower;

pub struct TowerAttackPlugin;

impl Plugin for TowerAttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (shoot, move_projectile));
        app.register_type::<Projectile>();
    }
}

#[derive(Reflect, Component)]
#[reflect(Component)]
#[require(Transform)]
struct Projectile {
    speed: f32,
    damage: isize,
    target: Entity,
}

fn shoot(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut tower: Query<(&mut Tower, &Transform)>,
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
            let dist = tower_transform
                .translation
                .distance_squared(enemy_transform.translation);

            if closest_dist.is_none_or(|x| x > dist) {
                closest_dist = Some(dist);
                closest_enemy = Some(entity);
            }
        }

        if closest_dist.is_none_or(|x| x.sqrt() > tower.range()) {
            continue;
        }

        if let Some(closest) = closest_enemy {
            tower.attack_timer.reset();
            commands.spawn((
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
                        x: tower_transform.translation.x + tower.offset().0 as f32,
                        y: tower_transform.translation.y + tower.offset().1 as f32,
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
            error!("target enemy no longer exists, despawning projectile");
            commands.get_entity(entity).unwrap().despawn();
        }
    }
}
