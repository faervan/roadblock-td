use bevy::prelude::*;
use bevy_lunex::{Dimension, UiLunexPlugins, UiTextSize};
use hud::HUDPlugin;
use main_menu::MainMenuPlugin;
use settings::SettingsMenuPlugin;

mod hud;
mod main_menu;
mod settings;

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Dimension>()
            .register_type::<UiTextSize>()
            .add_plugins(UiLunexPlugins)
            .add_plugins((MainMenuPlugin, HUDPlugin, SettingsMenuPlugin));
    }
}

fn despawn_menu<T: Component>(mut commands: Commands, entities: Query<Entity, With<T>>) {
    for entity in &entities {
        commands.entity(entity).despawn_recursive();
    }
}
