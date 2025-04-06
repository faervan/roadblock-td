use bevy::{prelude::*, sprite::Anchor};
use bevy_lunex::{
    Ab, Align, Rh, Rl, UiFetchFromCamera, UiLayout, UiLayoutRoot, UiMeshPlane2d, UiTextSize,
};

use crate::{
    app_state::{GameState, WaveState},
    game_loop::WaveInfo,
    ui::despawn_menu,
};

pub struct WaveStartPlugin;

impl Plugin for WaveStartPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<WaveStartUIMarker>()
            .register_type::<WaveLoadingMarker>()
            .add_systems(OnEnter(WaveState::Starting), build_ui)
            .add_systems(
                OnExit(WaveState::Starting),
                despawn_menu::<WaveStartUIMarker>,
            )
            .add_systems(
                Update,
                update_loading_bar
                    .run_if(in_state(GameState::Running).and(in_state(WaveState::Starting))),
            );
    }
}

const UI_INFO_BACKGROUND: Color = Color::srgba(0., 0., 0., 0.85);
const WAVE_LOADING_BAR: Color = Color::srgb(1., 0., 0.);

const HEIGHT: f32 = 80.;
const WIDTH: f32 = 600.;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct WaveStartUIMarker;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct WaveLoadingMarker;

fn build_ui(
    camera: Single<Entity, With<Camera>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn((
            WaveStartUIMarker,
            Name::new("Wave start root"),
            UiLayoutRoot::new_2d(),
            UiFetchFromCamera::<0>,
        ))
        .with_children(|ui| {
            ui.spawn((
                Name::new("Wave start boundary"),
                UiLayout::boundary()
                    .pos1((Rl(50.) - Ab(WIDTH / 2.), Rl(30.)))
                    .pos2((Rl(50.) + Ab(WIDTH / 2.), Rl(30.) + Ab(HEIGHT)))
                    .pack(),
            ))
            .with_children(|ui| {
                ui.spawn((
                    Transform::from_translation(Vec3::Z * 5.),
                    UiTextSize::from(Rh(80.)),
                    TextFont::from_font_size(60.),
                    Text2d::new("NEW WAVE STARTING!"),
                ));
                ui.spawn((
                    Name::new("Wave loading bar"),
                    UiLayout::solid()
                        .size((Rl(100.), Rl(16.)))
                        .align_y(Align::END)
                        .pack(),
                    UiMeshPlane2d,
                    MeshMaterial2d(materials.add(UI_INFO_BACKGROUND)),
                ))
                .with_child((
                    WaveLoadingMarker,
                    Transform::from_xyz(-WIDTH / 2., 0., 1.),
                    Sprite {
                        color: WAVE_LOADING_BAR,
                        custom_size: Some(Vec2::new(0., HEIGHT / 5.)),
                        anchor: Anchor::CenterLeft,
                        ..Default::default()
                    },
                ));
            });
        })
        .set_parent(*camera);
}

fn update_loading_bar(
    wave: Res<WaveInfo>,
    mut loading_bar: Single<&mut Sprite, With<WaveLoadingMarker>>,
) {
    loading_bar.custom_size = Some(Vec2::new(
        wave.margin.elapsed_secs() / wave.margin.duration().as_secs_f32() * WIDTH,
        HEIGHT / 5.,
    ));
}
