use bevy::{prelude::*, time::Stopwatch};

use crate::app_state::{AppState, GameState};

pub struct GameLoopPlugin;

impl Plugin for GameLoopPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Currency(50));
        app.register_type::<GameStatistics>()
            .add_systems(OnEnter(AppState::Game), insert_statistics)
            .add_systems(
                Update,
                advance_stat_time.run_if(in_state(GameState::Running)),
            );
    }
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct GameStatistics {
    pub enemies_killed: usize,
    pub time: Stopwatch,
}

#[derive(Reflect, Resource)]
#[reflect(Resource)]
pub struct Currency(pub i32);

fn insert_statistics(mut commands: Commands) {
    commands.insert_resource(GameStatistics::default());
}

fn advance_stat_time(time: Res<Time>, mut stats: ResMut<GameStatistics>) {
    stats.time.tick(time.delta());
}
