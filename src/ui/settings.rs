use bevy::prelude::*;

use crate::{Settings, app_state::MenuState, soundtrack::SoundtrackToggled};

use super::{despawn_menu, helpers::build_menu};
pub struct SettingsMenuPlugin;

impl Plugin for SettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SettingsMarker>()
            .register_type::<SfxMarker>()
            .register_type::<SoundtrackMarker>()
            .add_systems(OnEnter(MenuState::Settings), build_ui)
            .add_systems(OnExit(MenuState::Settings), despawn_menu::<SettingsMarker>);
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct SettingsMarker;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct SfxMarker;
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct SoundtrackMarker;

const BUTTON_WIDTH: f32 = 750.;
const BUTTON_HEIGHT: f32 = 60.;
const BUTTON_GAP: f32 = 50.;

fn build_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<Settings>,
) {
    let buttons = [settings.sfx_label(), settings.soundtrack_label(), "Return"]
        .iter()
        .map(|button| {
            (
                *button,
                action(button),
                match *button {
                    v if v == Settings::SFX_VARIANTS[0]
                        || v == Settings::SFX_VARIANTS[1] =>
                    {
                        Some(insert_marker::<SfxMarker>())
                    }
                    v if v == Settings::SOUNDTRACK_VARIANTS[0]
                        || v == Settings::SOUNDTRACK_VARIANTS[1] =>
                    {
                        Some(insert_marker::<SoundtrackMarker>())
                    }
                    _ => None,
                },
            )
        })
        .collect();
    build_menu(
        &mut commands,
        &asset_server,
        "Settings menu",
        SettingsMarker,
        BUTTON_WIDTH,
        BUTTON_HEIGHT,
        BUTTON_GAP,
        buttons,
    );
}

fn action(button: &str) -> fn(&mut EntityCommands) {
    match button {
        v if v == Settings::SFX_VARIANTS[0] || v == Settings::SFX_VARIANTS[1] => {
            |cmds: &mut EntityCommands| {
                cmds.observe(
                    |_: Trigger<Pointer<Click>>,
                     mut settings: ResMut<Settings>,
                     mut text: Single<&mut Text2d, With<SfxMarker>>| {
                        settings.sfx_enabled = !settings.sfx_enabled;
                        text.0 = settings.sfx_label().to_string();
                    },
                );
            }
        }

        v if v == Settings::SOUNDTRACK_VARIANTS[0]
            || v == Settings::SOUNDTRACK_VARIANTS[1] =>
        {
            |cmds: &mut EntityCommands| {
                cmds.observe(
                    |_: Trigger<Pointer<Click>>,
                     mut settings: ResMut<Settings>,
                     mut events: EventWriter<SoundtrackToggled>,
                     mut text: Single<&mut Text2d, With<SoundtrackMarker>>| {
                        settings.soundtrack_enabled = !settings.soundtrack_enabled;
                        text.0 = settings.soundtrack_label().to_string();
                        events.write(SoundtrackToggled);
                    },
                );
            }
        }
        "Return" => |cmds: &mut EntityCommands| {
            cmds.observe(
                |_: Trigger<Pointer<Click>>,
                 mut next_state: ResMut<NextState<MenuState>>| {
                    next_state.set(MenuState::MainMenu)
                },
            );
        },
        _ => |_: &mut EntityCommands| {},
    }
}

fn insert_marker<T: Component + Default>() -> fn(&mut EntityCommands) {
    (|cmds: &mut EntityCommands| {
        cmds.insert(T::default());
    }) as fn(&mut EntityCommands)
}
