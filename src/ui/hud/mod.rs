use bevy::prelude::*;
use paused::PausedPlugin;
use tower_selection::TowerSelectionPlugin;

mod paused;
mod tower_selection;

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PausedPlugin, TowerSelectionPlugin));
    }
}
