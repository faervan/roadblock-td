use std::time::Duration;

use bevy::prelude::*;

use crate::{Health, grid::Grid, tower::Tower};

use super::{Enemy, PathChangedEvent};

pub struct EnemyAttackPlugin;

impl Plugin for EnemyAttackPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Attacking>()
            .add_systems(Update, enemy_attacking);
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Attacking {
    target: Entity,
    timer: Timer,
}

impl Attacking {
    pub fn new(target: Entity) -> Self {
        Self {
            target,
            timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
        }
    }
}

fn enemy_attacking(
    mut enemies: Query<(&Enemy, &mut Attacking, Entity)>,
    mut towers: Query<(&mut Health, &Tower)>,
    mut commands: Commands,
    mut grid: ResMut<Grid>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut event_writer: EventWriter<PathChangedEvent>,
) {
    for (enemy, mut attack, entity) in &mut enemies {
        attack.timer.tick(time.delta());
        if !attack.timer.just_finished() {
            continue;
        }

        if let Ok((mut health, tower)) = towers.get_mut(attack.target) {
            health.0 -= enemy.damage();

            if health.0 <= 0 {
                commands.entity(attack.target).despawn_recursive();
                event_writer.send(PathChangedEvent::now_free(
                    tower.clear_grid(&mut grid, attack.target),
                ));
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
