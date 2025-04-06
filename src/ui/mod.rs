use bevy::prelude::*;
use bevy_lunex::{Dimension, UiLunexPlugins, UiTextSize};
use hud::HUDPlugin;
use main_menu::MainMenuPlugin;
use settings::SettingsMenuPlugin;

use crate::Settings;

mod hud;
mod main_menu;
mod settings;

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Dimension>()
            .register_type::<UiTextSize>()
            .add_plugins(UiLunexPlugins)
            .add_plugins((MainMenuPlugin, HUDPlugin, SettingsMenuPlugin))
            .add_systems(Update, button_interaction);
    }
}

fn button_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    settings: Res<Settings>,
) {
    for (interaction, mut background_color) in &mut interaction_query {
        *background_color = match *interaction {
            Interaction::Pressed => {
                if settings.sfx_enabled {
                    commands.spawn(AudioPlayer::new(asset_server.load("sfx/Cloud Click.ogg")));
                }
                PRESSED_BUTTON.into()
            }
            Interaction::Hovered => {
                if settings.sfx_enabled {
                    commands.spawn(AudioPlayer::new(asset_server.load("sfx/Toom Click.ogg")));
                }
                HOVERED_BUTTON.into()
            }
            Interaction::None => NORMAL_BUTTON.into(),
        }
    }
}

fn despawn_menu<T: Component>(mut commands: Commands, entities: Query<Entity, With<T>>) {
    for entity in &entities {
        commands.entity(entity).despawn_recursive();
    }
}

fn button(width: f32) -> Node {
    Node {
        width: Val::Px(width),
        height: Val::Px(65.0),
        margin: UiRect::axes(Val::Px(20.), Val::Px(10.)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}

fn button_text_font() -> TextFont {
    TextFont {
        font_size: 33.0,
        ..default()
    }
}
