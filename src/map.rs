use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*, window::PrimaryWindow};

use crate::app_state::{AppState, GameState};

const BACKGROUND_COLOR: Color = Color::hsl(150., 1., 0.4);

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MapInfo>()
            .add_systems(OnEnter(AppState::Game), init)
            .add_systems(OnExit(AppState::Game), exit)
            .add_systems(
                Update,
                pan_camera
                    .run_if(in_state(AppState::Game))
                    .run_if(not(in_state(GameState::GameOver))),
            );
    }
}

#[derive(Reflect, Resource, Debug)]
#[reflect(Resource)]
struct MapInfo {
    size: Vec2,
    /// Bottom left anchor of the map in bevy's coordinate system
    anchor: Vec2,
}

fn init(mut commands: Commands) {
    let map_size = Vec2::new(3000., 2000.);
    let map_anchor = Vec2::new(-map_size.x / 2., -map_size.y / 2.);

    commands.spawn((
        Sprite::from_color(BACKGROUND_COLOR, map_size),
        Transform::from_translation(Vec3::new(
            map_anchor.x + map_size.x / 2.,
            map_anchor.y + map_size.y / 2.,
            -1.,
        )),
    ));
    commands.insert_resource(MapInfo {
        size: map_size,
        anchor: map_anchor,
    });
}

fn exit(mut commands: Commands, mut camera: Single<&mut Transform, With<Camera>>) {
    camera.translation = Vec3::ZERO;
    commands.remove_resource::<MapInfo>();
}

fn pan_camera(
    mut camera: Single<&mut Transform, With<Camera>>,
    input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    motion: Res<AccumulatedMouseMotion>,
    map_info: Res<MapInfo>,
    window: Single<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    let [w_pressed, a_pressed, s_pressed, d_pressed] = [
        input.pressed(KeyCode::KeyW),
        input.pressed(KeyCode::KeyA),
        input.pressed(KeyCode::KeyS),
        input.pressed(KeyCode::KeyD),
    ];
    if w_pressed || a_pressed || s_pressed || d_pressed {
        let mut direction = Vec3::ZERO;
        w_pressed.then(|| direction.y += 1.);
        a_pressed.then(|| direction.x -= 1.);
        s_pressed.then(|| direction.y -= 1.);
        d_pressed.then(|| direction.x += 1.);
        camera.translation += direction.normalize() * time.delta_secs() * 500.;
    } else if mouse_input.pressed(MouseButton::Middle) && motion.delta != Vec2::ZERO {
        camera.translation.x -= motion.delta.x;
        camera.translation.y += motion.delta.y;
    } else {
        return;
    }

    let window_size = window.size() / 2.;

    // Prevent panning to infinity
    if camera.translation.x < map_info.anchor.x + window_size.x {
        camera.translation.x = map_info.anchor.x + window_size.x;
    } else if camera.translation.x > map_info.anchor.x + map_info.size.x - window_size.x {
        camera.translation.x = map_info.anchor.x + map_info.size.x - window_size.x;
    }

    if camera.translation.y < map_info.anchor.y + window_size.y {
        camera.translation.y = map_info.anchor.y + window_size.y;
    } else if camera.translation.y > map_info.anchor.y + map_info.size.y - window_size.y {
        camera.translation.y = map_info.anchor.y + map_info.size.y - window_size.y;
    }
}
