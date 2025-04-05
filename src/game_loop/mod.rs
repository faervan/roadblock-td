use bevy::{prelude::*, time::Stopwatch};

use crate::app_state::{AppState, GameState};

pub struct GameLoopPlugin;

impl Plugin for GameLoopPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GameStatistics>()
            .add_systems(OnEnter(AppState::Game), insert_resources)
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
    pub money_earned: i32,
    pub money_spend: i32,
}

#[derive(Reflect, Resource, Deref)]
#[reflect(Resource)]
pub struct Currency(pub i32);

pub fn insert_resources(mut commands: Commands) {
    commands.insert_resource(GameStatistics::default());
    commands.insert_resource(Currency(50));
}

fn advance_stat_time(time: Res<Time>, mut stats: ResMut<GameStatistics>) {
    stats.time.tick(time.delta());
}
