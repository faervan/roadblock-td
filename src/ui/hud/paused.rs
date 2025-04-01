use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{
    app_state::{AppState, GameState},
    ui::{TEXT_COLOR, despawn_menu},
};

pub struct PausedPlugin;

impl Plugin for PausedPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PausedMarker>()
            .add_systems(OnEnter(GameState::Paused), build_paused_info)
            .add_systems(OnExit(GameState::Paused), despawn_menu::<PausedMarker>)
            .add_systems(
                Update,
                toggle_paused_state
                    .run_if(in_state(AppState::Game))
                    .run_if(input_just_pressed(KeyCode::KeyP)),
            );
    }
}

fn toggle_paused_state(
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    next_state.set(match current_state.get() {
        GameState::Paused => GameState::Running,
        GameState::Running => GameState::Paused,
        GameState::GameOver => GameState::GameOver,
    });
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct PausedMarker;

fn build_paused_info(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            PausedMarker,
        ))
        .with_children(|p| {
            p.spawn((
                Node {
                    width: Val::Percent(100.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                BackgroundColor(Color::srgba(0., 0., 0., 0.7)),
            ))
            .with_children(|p| {
                p.spawn((
                    Text::new("Paused"),
                    TextFont {
                        font_size: 80.,
                        ..Default::default()
                    },
                    TextColor(TEXT_COLOR),
                ));
                p.spawn((
                    Text::new("Press P to unpause"),
                    TextFont {
                        font_size: 40.,
                        ..Default::default()
                    },
                    TextColor(TEXT_COLOR),
                ));
            });
        });
}
