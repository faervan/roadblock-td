use bevy::{
    input::common_conditions::input_just_pressed, picking::pointer::PointerId, prelude::*,
    window::Monitor,
};

pub struct AppStatePlugin;

impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_sub_state::<MenuState>()
            .add_sub_state::<GameState>()
            .add_sub_state::<TowerPlacingState>()
            .add_sub_state::<WaveState>()
            .add_systems(Startup, set_app_state)
            .add_systems(
                Update,
                escape_to_menu
                    .run_if(in_state(AppState::Game))
                    .run_if(input_just_pressed(KeyCode::Escape)),
            )
            .add_systems(OnExit(AppState::Game), clear_game);
    }
}

#[derive(States, Debug, Default, Hash, PartialEq, Eq, Clone)]
pub enum AppState {
    #[default]
    Loading,
    Menu,
    Game,
}

// https://github.com/bevyengine/bevy/discussions/17625#discussioncomment-12058159
// Used this to fix a panic in soundtracks.rs
fn set_app_state(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::Menu);
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, SubStates)]
#[source(AppState = AppState::Menu)]
pub enum MenuState {
    #[default]
    MainMenu,
    Settings,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, SubStates)]
#[source(AppState = AppState::Game)]
pub enum GameState {
    #[default]
    Running,
    Paused,
    GameOver,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, SubStates)]
#[source(AppState = AppState::Game)]
pub enum TowerPlacingState {
    Placing,
    #[default]
    None,
}

#[derive(SubStates, Hash, Debug, Clone, Copy, PartialEq, Eq, Default)]
#[source(AppState = AppState::Game)]
pub enum WaveState {
    #[default]
    Starting,
    Ongoing,
    AllFinished,
}

fn escape_to_menu(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::Menu);
}

fn clear_game(
    mut commands: Commands,
    entities: Query<
        Entity,
        (
            Without<Camera>,
            Without<Window>,
            Without<Monitor>,
            Without<Observer>,
            Without<PointerId>,
        ),
    >,
) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}
