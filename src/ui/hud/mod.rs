use bevy::prelude::*;
use bottom_bar::BottomBarPlugin;
use game_over::GameOverPlugin;
use paused::PausedPlugin;
use top_bar::TopBarPlugin;
use wave_start::WaveStartPlugin;

mod bottom_bar;
mod game_over;
mod paused;
mod top_bar;
mod wave_start;

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            TopBarPlugin,
            BottomBarPlugin,
            WaveStartPlugin,
            GameOverPlugin,
            PausedPlugin,
        ));
    }
}
