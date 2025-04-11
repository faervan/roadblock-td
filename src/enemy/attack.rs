use bevy::prelude::*;

use crate::{
    app_state::GameState,
    game_loop::{Currency, GameStatistics},
    grid::Grid,
    health::Health,
    tower::Tower,
};

use super::{Enemy, PathChangedEvent, goal::EnemyGoal};

pub struct EnemyAttackPlugin;

impl Plugin for EnemyAttackPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Attacking>().add_systems(
            Update,
            (advance_enemy_attack_timers, enemy_attacking, enemy_attacking_goal).run_if(in_state(GameState::Running)),
        );
        app.world_mut()
            .register_component_hooks::<Attacking>()
            .on_remove(|mut world, entity, _| {
                let weapon_id = world.get::<Attacking>(entity).unwrap().weapon_id;
                if let Some(mut entity_cmds) = world.commands().get_entity(weapon_id) {
                    entity_cmds.despawn();
                }
            });
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Attacking {
    target: Entity,
    weapon_id: Entity,
}

impl Attacking {
    pub fn new(target: Entity, weapon_id: Entity) -> Self {
        Attacking { target, weapon_id }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct AttackingGoal;

fn advance_enemy_attack_timers(mut enemies: Query<&mut Enemy>, time: Res<Time>) {
    for mut enemy in &mut enemies {
        enemy.attack_timer.tick(time.delta());
    }
}

fn enemy_attacking(
    mut enemies: Query<(&mut Enemy, &Attacking, Entity, &mut Health)>,
    mut towers: Query<(&mut Health, &Tower), Without<Enemy>>,
    mut currency: ResMut<Currency>,
    mut commands: Commands,
    mut grid: ResMut<Grid>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut event_writer: EventWriter<PathChangedEvent>,
) {
    for (mut enemy, attacking, entity, mut enemy_health) in &mut enemies {
        if !enemy.attack_timer.finished() {
            continue;
        }
        enemy.attack_timer.reset();

        if let Ok((mut health, tower)) = towers.get_mut(attacking.target) {
            **health -= enemy.damage();

            if **health <= 0 {
                commands.entity(attacking.target).despawn_recursive();
                event_writer.send(PathChangedEvent::now_free(
                    tower.clear_grid(&mut grid, attacking.target),
                ));
            }

            **enemy_health -= tower.contact_damage();

            if **enemy_health <= 0 {
                **currency += enemy.reward();
                commands.entity(entity).despawn_recursive();

                return;
            }
        }

        commands.entity(entity).remove::<Attacking>().insert((
            enemy.walk_animation_config(),
            Sprite {
                image: asset_server.load(enemy.walk_sprites()),
                texture_atlas: Some(enemy.walk_layout(&mut texture_atlas_layouts)),
                ..Default::default()
            },
        ));
    }
}

fn enemy_attacking_goal(
    mut enemies: Query<(&mut Enemy, &mut Health, Entity), With<AttackingGoal>>,
    mut commands: Commands,
    mut goal: Single<(&EnemyGoal, &mut Health), Without<Enemy>>,
    mut currency: ResMut<Currency>,
    mut stats: ResMut<GameStatistics>,
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
            **currency += enemy.reward();
            stats.money_earned += enemy.reward();
            stats.enemies_killed += 1;
            commands.entity(entity).despawn_recursive();
        }

        ***goal_health -= enemy.damage();
        if ***goal_health <= 0 {
            next_state.set(GameState::GameOver);
        }
    }
}
