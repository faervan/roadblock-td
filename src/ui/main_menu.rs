use bevy::prelude::*;

use crate::app_state::{AppState, MenuState};

use super::{despawn_menu, helpers::build_menu};
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MainMenuMarker>()
            .add_systems(OnEnter(MenuState::MainMenu), build_ui)
            .add_systems(OnExit(MenuState::MainMenu), despawn_menu::<MainMenuMarker>);
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct MainMenuMarker;

const MENU_BUTTONS: &[&str] = &["Play", "Settings", "Quit"];
const BUTTON_WIDTH: f32 = 450.;
const BUTTON_HEIGHT: f32 = 60.;
const BUTTON_GAP: f32 = 50.;

fn build_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let buttons = MENU_BUTTONS
        .iter()
        .map(|button| (*button, action(button), None))
        .collect();
    build_menu(
        &mut commands,
        &asset_server,
        "Main menu",
        MainMenuMarker,
        BUTTON_WIDTH,
        BUTTON_HEIGHT,
        BUTTON_GAP,
        buttons,
    );
}

fn action(button: &str) -> fn(&mut EntityCommands) {
    match button {
        "Play" => |cmds: &mut EntityCommands| {
            cmds.observe(
                |_: Trigger<Pointer<Click>>,
                 mut next_state: ResMut<NextState<AppState>>| {
                    next_state.set(AppState::Game)
                },
            );
        },
        "Settings" => |cmds: &mut EntityCommands| {
            cmds.observe(
                |_: Trigger<Pointer<Click>>,
                 mut next_state: ResMut<NextState<MenuState>>| {
                    next_state.set(MenuState::Settings)
                },
            );
        },
        "Quit" => |cmds: &mut EntityCommands| {
            cmds.observe(
                |_: Trigger<Pointer<Click>>, mut app_exit: EventWriter<AppExit>| {
                    app_exit.send(AppExit::Success);
                },
            );
        },
        _ => |_: &mut EntityCommands| {},
    }
}
