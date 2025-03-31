use bevy::prelude::*;
use hud::HUDPlugin;
use main_menu::MainMenuPlugin;

mod hud;
mod main_menu;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MainMenuPlugin, HUDPlugin));
    }
}

fn despawn_menu<T: Component>(mut commands: Commands, entities: Query<Entity, With<T>>) {
    for entity in &entities {
        commands.entity(entity).despawn_recursive();
    }
}
