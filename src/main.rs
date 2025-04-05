use app_state::AppStatePlugin;
use bevy::{audio::AudioPlugin, prelude::*};
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
                    mode: bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                    ..default()
                }),
                ..default()
            })
            .set(AudioPlugin {
                global_volume: GlobalVolume::new(0.05),
                ..Default::default()
            }),
    );

    if std::env::args().any(|a| a == "--egui") {
        app.add_plugins(WorldInspectorPlugin::new());
    }

    app.register_type::<Settings>();

    app.insert_resource(RngResource(Rng::new()));

    let [sfx_enabled, soundtrack_enabled] = match std::env::args().any(|a| a == "--silent") {
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
    fn sfx_label(&self) -> &str {
        match self.sfx_enabled {
            true => "Sfx enabled",
            false => "Sfx disabled",
        }
    }
    fn soundtrack_label(&self) -> &str {
        match self.soundtrack_enabled {
            true => "Soundtrack enabled",
            false => "Soundtrack disabled",
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
        app_exit.send(AppExit::Success);
    }
}
