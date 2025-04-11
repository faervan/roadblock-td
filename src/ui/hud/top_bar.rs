use bevy::prelude::*;
use bevy_lunex::{Ab, Align, Rh, Rl, UiFetchFromCamera, UiLayout, UiLayoutRoot, UiMeshPlane2d, UiTextSize};

use crate::{
    app_state::{AppState, GameState},
    game_loop::{Currency, WaveInfo, WaveStart, insert_wave_info},
};

pub struct TopBarPlugin;

impl Plugin for TopBarPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CurrencyInfoMarker>()
            .register_type::<WaveInfoMarker>()
            .add_systems(OnEnter(AppState::Game), build_ui.after(insert_wave_info))
            .add_systems(
                Update,
                (
                    update_wave.run_if(on_event::<WaveStart>),
                    update_currency.run_if(in_state(GameState::Running)),
                ),
            );
    }
}

const UI_INFO_BACKGROUND: Color = Color::srgba(0., 0., 0., 0.85);

#[derive(Component, Reflect)]
#[reflect(Component)]
struct WaveInfoMarker;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct CurrencyInfoMarker;

fn build_ui(
    camera: Single<Entity, With<Camera>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    wave: Res<WaveInfo>,
) {
    commands
        .spawn((
            Name::new("Top bar root"),
            UiLayoutRoot::new_2d(),
            UiFetchFromCamera::<0>,
        ))
        .with_children(|ui| {
            // Spawn boundary node
            ui.spawn((
                Name::new("Top bar boundary"),
                UiLayout::boundary().pos1(Ab(0.)).pos2((Rl(100.), Ab(50.))).pack(),
            ))
            .with_children(|ui| {
                ui.spawn((
                    Name::new("Wave info"),
                    UiLayout::solid().size((Ab(250.), Rl(100.))).align_x(Align::LEFT).pack(),
                    UiMeshPlane2d,
                    MeshMaterial2d(materials.add(UI_INFO_BACKGROUND)),
                ))
                .with_child((
                    WaveInfoMarker,
                    Transform::from_translation(Vec3::Z * 5.),
                    UiTextSize::from(Rh(5.)),
                    Text2d::new(format!("Wave 0/{}", wave.last)),
                ));
                ui.spawn((
                    Name::new("Currency info"),
                    UiLayout::solid()
                        .size((Ab(250.), Rl(100.)))
                        .align_x(Align::CENTER)
                        .pack(),
                    UiMeshPlane2d,
                    MeshMaterial2d(materials.add(UI_INFO_BACKGROUND)),
                ))
                .with_child((
                    CurrencyInfoMarker,
                    Transform::from_translation(Vec3::Z * 5.),
                    UiTextSize::from(Rh(5.)),
                    Text2d::new(""),
                ));
            });
        })
        .set_parent(*camera);
}

fn update_wave(
    mut wave_info: Single<&mut Text2d, With<WaveInfoMarker>>,
    mut events: EventReader<WaveStart>,
    wave: Res<WaveInfo>,
) {
    for new_wave in events.read() {
        wave_info.0 = format!("Wave {}/{}", **new_wave, wave.last);
    }
}

fn update_currency(mut currency_info: Single<&mut Text2d, With<CurrencyInfoMarker>>, currency: Res<Currency>) {
    if currency.is_changed() {
        currency_info.0 = format!("Money: {}", **currency);
    }
}
