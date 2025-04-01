use bevy::{
    input::common_conditions::input_just_pressed, picking::pointer::PointerId, prelude::*,
    window::Monitor,
};

pub struct AppStatePlugin;

impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_computed_state::<InGame>()
            .add_computed_state::<GameOver>()
            .add_computed_state::<TowerPlacing>()
            .add_sub_state::<MenuState>()
            .add_systems(Startup, set_app_state)
            .add_systems(
                Update,
                escape_to_menu
                    .run_if(in_state(InGame))
                    .run_if(input_just_pressed(KeyCode::Escape)),
            )
            .add_systems(OnExit(InGame), clear_game);
    }
}

#[derive(States, Debug, Default, Hash, PartialEq, Eq, Clone)]
pub enum AppState {
    #[default]
    Loading,
    Menu,
    Game {
        game_over: bool,
        paused: bool,
        placing_tower: bool,
    },
}

// https://github.com/bevyengine/bevy/discussions/17625#discussioncomment-12058159
// Used this to fix a panic in soundtracks.rs
fn set_app_state(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::Menu);
}

impl AppState {
    pub fn in_game() -> Self {
        AppState::Game {
            game_over: false,
            paused: false,
            placing_tower: false,
        }
    }
    pub fn game_over() -> Self {
        AppState::Game {
            game_over: true,
            paused: true,
            placing_tower: false,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct InGame;

impl ComputedStates for InGame {
    type SourceStates = AppState;
    fn compute(sources: Self::SourceStates) -> Option<Self> {
        match sources {
            AppState::Game { .. } => Some(InGame),
            _ => None,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct GameOver;

impl ComputedStates for GameOver {
    type SourceStates = AppState;
    fn compute(sources: Self::SourceStates) -> Option<Self> {
        match sources {
            AppState::Game {
                game_over: true, ..
            } => Some(GameOver),
            _ => None,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct TowerPlacing;

impl ComputedStates for TowerPlacing {
    type SourceStates = AppState;
    fn compute(sources: Self::SourceStates) -> Option<Self> {
        match sources {
            AppState::Game {
                placing_tower: true,
                ..
            } => Some(TowerPlacing),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, SubStates)]
#[source(AppState = AppState::Menu)]
pub enum MenuState {
    #[default]
    MainMenu,
    Settings,
}

pub fn set_tower_placing_state(
    current_state: &State<AppState>,
    next_state: &mut NextState<AppState>,
) {
    if let AppState::Game {
        game_over,
        paused,
        placing_tower,
    } = current_state.get()
    {
        next_state.set(AppState::Game {
            game_over: *game_over,
            paused: *paused,
            placing_tower: !placing_tower,
        });
    }
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
