use bevy::prelude::*;
use tower_selection::TowerSelectionPlugin;

mod tower_selection;

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TowerSelectionPlugin);
    }
}
