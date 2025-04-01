use bevy::prelude::*;

use crate::{
    Health,
    app_state::{AppState, GameState},
    grid::Grid,
    tower::Tower,
};

use super::{Enemy, PathChangedEvent, goal::EnemyGoal};

pub struct EnemyAttackPlugin;

impl Plugin for EnemyAttackPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Attacking>().add_systems(
            Update,
            (
                advance_enemy_attack_timers,
                enemy_attacking,
                enemy_attacking_goal,
            )
                .run_if(in_state(AppState::Game)),
        );
    }
}

#[derive(Component, Reflect, Deref)]
#[reflect(Component)]
pub struct Attacking(pub Entity);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct AttackingGoal;

fn advance_enemy_attack_timers(mut enemies: Query<&mut Enemy>, time: Res<Time>) {
    for mut enemy in &mut enemies {
        enemy.attack_timer.tick(time.delta());
    }
}

fn enemy_attacking(
    mut enemies: Query<(&mut Enemy, &Attacking, Entity)>,
    mut towers: Query<(&mut Health, &Tower)>,
    mut commands: Commands,
    mut grid: ResMut<Grid>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut event_writer: EventWriter<PathChangedEvent>,
) {
    for (mut enemy, target, entity) in &mut enemies {
        if !enemy.attack_timer.finished() {
            continue;
        }
        enemy.attack_timer.reset();

        if let Ok((mut health, tower)) = towers.get_mut(**target) {
            health.0 -= enemy.damage();

            if health.0 <= 0 {
                commands.entity(**target).despawn_recursive();
                event_writer.send(PathChangedEvent::now_free(
                    tower.clear_grid(&mut grid, **target),
                ));
            }
        }

        commands
            .entity(entity)
            .remove::<Attacking>()
            .insert((
                enemy.walk_animation_config(),
                Sprite {
                    image: asset_server.load(enemy.walk_sprites()),
                    texture_atlas: Some(enemy.walk_layout(&mut texture_atlas_layouts)),
                    ..Default::default()
                },
            ))
            .despawn_descendants();
    }
}

fn enemy_attacking_goal(
    mut enemies: Query<(&mut Enemy, &mut Health, Entity), With<AttackingGoal>>,
    mut commands: Commands,
    mut goal: Single<(&EnemyGoal, &mut Health), Without<Enemy>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let (goal, goal_health) = (goal.0, &mut goal.1);
    for (mut enemy, mut enemy_health, entity) in &mut enemies {
        if !enemy.attack_timer.finished() {
            continue;
        }
        enemy.attack_timer.reset();

        **enemy_health -= goal.thorn_damage();
        if **enemy_health <= 0 {
            commands.entity(entity).despawn_recursive();
        }

        ***goal_health -= enemy.damage();
        if ***goal_health <= 0 {
            next_state.set(GameState::GameOver);
            info!("GAMEOVER!!");
        }
    }
}
