use bevy::prelude::*;
use bottom_bar::BottomBarPlugin;
use game_over::GameOverPlugin;
use paused::PausedPlugin;

mod bottom_bar;
mod game_over;
mod paused;

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((BottomBarPlugin, GameOverPlugin, PausedPlugin));
    }
}
