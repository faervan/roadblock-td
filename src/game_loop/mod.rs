use bevy::{prelude::*, time::Stopwatch};
use wave::WavePlugin;

use crate::app_state::{AppState, GameState};

pub use wave::{SpawnerInfo, WaveInfo, WaveStart, insert_wave_info};

mod wave;

pub struct GameLoopPlugin;

impl Plugin for GameLoopPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GameStatistics>()
            .add_plugins(WavePlugin)
            .add_systems(OnEnter(AppState::Game), insert_game_resources)
            .add_systems(Update, advance_stat_time.run_if(in_state(GameState::Running)));
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

#[derive(Reflect, Resource, Deref, DerefMut)]
#[reflect(Resource)]
pub struct Currency(i32);

fn insert_game_resources(mut commands: Commands) {
    commands.insert_resource(GameStatistics::default());
    commands.insert_resource(Currency(80));
}

fn advance_stat_time(time: Res<Time>, mut stats: ResMut<GameStatistics>) {
    stats.time.tick(time.delta());
}
