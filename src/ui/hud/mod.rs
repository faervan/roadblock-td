use bevy::prelude::*;
use bottom_bar::BottomBarPlugin;
use game_over::GameOverPlugin;
use paused::PausedPlugin;
use top_bar::TopBarPlugin;

mod bottom_bar;
mod game_over;
mod paused;
mod top_bar;

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((TopBarPlugin, BottomBarPlugin, GameOverPlugin, PausedPlugin));
    }
}
