use std::time::Duration;

use bevy::{prelude::*, utils::HashMap};

use crate::{
    app_state::{AppState, GameState, WaveState},
    enemy::{Enemy, EnemyType},
};

pub struct WavePlugin;

impl Plugin for WavePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WaveStart>()
            .add_systems(OnEnter(AppState::Game), insert_wave_info)
            .add_systems(
                Update,
                (
                    count_wave_margin.run_if(in_state(WaveState::Starting)),
                    check_wave_finished.run_if(in_state(WaveState::Ongoing)),
                )
                    .run_if(in_state(GameState::Running)),
            );
    }
}

type Wave = usize;

#[derive(Event, Deref)]
pub struct WaveStart {
    #[deref]
    current: Wave,
    pub new_spawners: Vec<SpawnerInfo>,
}

#[derive(Resource, Default)]
pub struct WaveInfo {
    spawners: HashMap<Wave, Vec<SpawnerInfo>>,
    current: Wave,
    pub last: Wave,
    pub margin: Timer,
    /// Count of spawners already spawned up to this wave
    current_spawners: usize,
    /// Count of the spawners which completed all of their spawns for this wave
    pub done_this_wave: usize,
}

#[derive(Clone, Copy)]
pub struct SpawnerInfo {
    pub interval: fn(Wave) -> f32,
    pub enemies: fn(Wave) -> Vec<EnemyType>,
}

impl Default for SpawnerInfo {
    fn default() -> Self {
        Self {
            interval: |_| 1.,
            enemies: |wave| vec![EnemyType::Skeleton; wave],
        }
    }
}

pub fn insert_wave_info(mut commands: Commands) {
    let spawner = SpawnerInfo {
        interval: |_| 0.5,
        enemies: |wave| vec![EnemyType::Skeleton; wave + 1],
    };
    commands.insert_resource(WaveInfo {
        spawners: HashMap::from([
            (1, vec![spawner; 2]),
            (2, vec![spawner]),
            (5, vec![spawner; 2]),
            (6, vec![spawner]),
            (7, vec![spawner]),
            (8, vec![spawner; 2]),
        ]),
        last: 10,
        margin: Timer::new(Duration::from_secs(5), TimerMode::Once),
        ..Default::default()
    });
}

fn count_wave_margin(
    mut wave: ResMut<WaveInfo>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<WaveState>>,
    mut events: EventWriter<WaveStart>,
) {
    wave.margin.tick(time.delta());
    if wave.margin.just_finished() {
        next_state.set(WaveState::Ongoing);
        wave.current += 1;
        wave.margin.reset();
        wave.done_this_wave = 0;

        let spawners = wave.spawners.get(&wave.current).cloned().unwrap_or_default();
        wave.current_spawners += spawners.len();
        events.send(WaveStart {
            current: wave.current,
            new_spawners: spawners,
        });
    }
}

fn check_wave_finished(
    wave: Res<WaveInfo>,
    enemies: Query<(), With<Enemy>>,
    mut next_state: ResMut<NextState<WaveState>>,
) {
    if wave.current_spawners == wave.done_this_wave && enemies.is_empty() {
        if wave.current < wave.last {
            next_state.set(WaveState::Starting);
        } else {
            next_state.set(WaveState::AllFinished);
        }
    }
}
