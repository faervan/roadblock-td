use bevy::prelude::*;
use game_over::GameOverPlugin;
use paused::PausedPlugin;
use tower_selection::TowerSelectionPlugin;

mod game_over;
mod paused;
mod tower_selection;

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((GameOverPlugin, PausedPlugin, TowerSelectionPlugin));
    }
}
