use std::time::Duration;

use bevy::prelude::*;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, run_animations);
    }
}

#[derive(Component)]
pub struct AnimationConfig {
    first_sprite: usize,
    last_sprite: usize,
    frame_timer: Timer,
}

impl AnimationConfig {
    pub fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite: first,
            last_sprite: last,
            frame_timer: Timer::new(
                Duration::from_secs_f32(1. / fps as f32),
                TimerMode::Repeating,
            ),
        }
    }
}

fn run_animations(time: Res<Time>, mut query: Query<(&mut AnimationConfig, &mut Sprite)>) {
    for (mut config, mut sprite) in &mut query {
        config.frame_timer.tick(time.delta());

        if config.frame_timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index == config.last_sprite {
                    atlas.index = config.first_sprite;
                } else {
                    atlas.index += 1;
                }
            }
        }
    }
}
