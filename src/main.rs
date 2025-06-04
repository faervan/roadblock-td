use app_state::AppStatePlugin;
use bevy::{
    audio::{AudioPlugin, Volume},
    prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_lunex::UiSourceCamera;
use enemy::EnemyPlugin;
use fastrand::Rng;
use game_loop::GameLoopPlugin;
use grid::GridPlugin;
use health::HealthPlugin;
use map::MapPlugin;
use soundtrack::SoundtrackPlugin;
use tower::TowerPlugin;
use ui::UIPlugin;

mod animation;
mod app_state;
mod enemy;
mod game_loop;
mod grid;
mod health;
mod map;
mod soundtrack;
mod tower;
mod ui;

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    #[cfg(debug_assertions)]
                    present_mode: bevy::window::PresentMode::AutoNoVsync,
                    mode: bevy::window::WindowMode::BorderlessFullscreen(
                        MonitorSelection::Primary,
                    ),
                    ..default()
                }),
                ..default()
            })
            .set(AudioPlugin {
                global_volume: GlobalVolume::new(Volume::Linear(0.1)),
                ..Default::default()
            }),
    );

    if std::env::args().any(|a| a == "--egui") {
        app.add_plugins(WorldInspectorPlugin::new());
    }

    app.register_type::<Settings>();
    app.register_type::<AssetLock>();

    app.insert_resource(RngResource(Rng::new()));

    let [sfx_enabled, soundtrack_enabled] =
        match std::env::args().any(|a| a == "--silent") {
            true => [false, false],
            false => [true, true],
        };

    app.insert_resource(Settings {
        sfx_enabled,
        soundtrack_enabled,
    });

    app.add_plugins((
        animation::AnimationPlugin,
        AppStatePlugin,
        EnemyPlugin,
        GameLoopPlugin,
        GridPlugin,
        HealthPlugin,
        MapPlugin,
        SoundtrackPlugin,
        TowerPlugin,
        UIPlugin,
    ));

    app.add_systems(PreStartup, preload_assets);
    app.add_systems(Startup, setup);
    app.add_systems(Update, exit_on_ctrl_q);

    app.run();
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
struct Settings {
    sfx_enabled: bool,
    soundtrack_enabled: bool,
}

impl Settings {
    const SFX_VARIANTS: &[&str] = &["Sfx enabled", "Sfx disabled"];
    const SOUNDTRACK_VARIANTS: &[&str] = &["Soundtrack enabled", "Soundtrack disabled"];

    fn sfx_label(&self) -> &'static str {
        match self.sfx_enabled {
            true => Self::SFX_VARIANTS[0],
            false => Self::SFX_VARIANTS[1],
        }
    }

    fn soundtrack_label(&self) -> &'static str {
        match self.soundtrack_enabled {
            true => Self::SOUNDTRACK_VARIANTS[0],
            false => Self::SOUNDTRACK_VARIANTS[1],
        }
    }
}

#[derive(Resource, Deref, DerefMut)]
struct RngResource(Rng);

#[derive(Reflect, Default, PartialEq, Debug, Clone, Copy)]
enum Orientation {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

impl Orientation {
    fn is_horizontal(&self) -> bool {
        match self {
            Orientation::Up | Orientation::Down => false,
            Orientation::Left | Orientation::Right => true,
        }
    }
}

const CAMERA_POS: Vec3 = Vec3::new(0., 0., 900.);
fn setup(mut commands: Commands) {
    commands.spawn((
        Name::new("MainCamera"),
        Camera2d,
        Transform::from_translation(CAMERA_POS),
        UiSourceCamera::<0>,
    ));
}

fn exit_on_ctrl_q(mut app_exit: EventWriter<AppExit>, input: Res<ButtonInput<KeyCode>>) {
    if input.pressed(KeyCode::ControlLeft) && input.just_pressed(KeyCode::KeyQ) {
        app_exit.write(AppExit::Success);
    }
}

#[allow(dead_code)]
#[derive(Component, Reflect)]
#[reflect(Component)]
struct AssetLock(#[reflect(ignore)] Vec<UntypedHandle>);

/// This system makes sure certain Assets never get dropped by bevy, keeping them in memory for the
/// entire lifetime of the game
fn preload_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Name::new("AssetLock"),
        AssetLock(vec![
            asset_server.load::<Image>("title_image.png").untyped(),
            asset_server
                .load::<AudioSource>("sfx/Cloud Click.ogg")
                .untyped(),
        ]),
    ));
}
