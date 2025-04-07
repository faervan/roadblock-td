use bevy::prelude::*;
use bevy_lunex::{
    Ab, Anchor, OnHoverSetCursor, Rh, Rl, Scaling, UiBase, UiColor, UiDepth, UiFetchFromCamera, UiHover, UiLayout,
    UiLayoutRoot, UiStateTrait, UiTextSize, hover_set,
};

use crate::{Settings, app_state::MenuState, soundtrack::SoundtrackToggled};

use super::despawn_menu;
pub struct SettingsMenuPlugin;

impl Plugin for SettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SettingsMarker>()
            .register_type::<SfxMarker>()
            .register_type::<SoundtrackMarker>()
            .add_systems(OnEnter(MenuState::Settings), build_ui)
            .add_systems(OnExit(MenuState::Settings), despawn_menu::<SettingsMarker>);
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct SettingsMarker;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct SfxMarker;
#[derive(Component, Reflect)]
#[reflect(Component)]
struct SoundtrackMarker;

const BUTTON_WIDTH: f32 = 750.;
const BUTTON_HEIGHT: f32 = 60.;
const BUTTON_GAP: f32 = 50.;

fn build_ui(mut commands: Commands, asset_server: Res<AssetServer>, settings: Res<Settings>) {
    let menu_buttons = vec![settings.sfx_label(), settings.soundtrack_label(), "Return"];

    commands
        .spawn((
            Name::new("Settings root"),
            UiLayoutRoot::new_2d(),
            UiFetchFromCamera::<0>,
            SettingsMarker,
        ))
        .with_children(|ui| {
            ui.spawn((
                Name::new("Menu background image"),
                UiLayout::solid()
                    .size((Ab(1920.), Ab(1080.)))
                    .scaling(Scaling::Fill)
                    .pack(),
                Sprite::from_image(asset_server.load("title_image.png")),
            ));

            let x_offset = BUTTON_WIDTH / 2.;
            let y_offset = (BUTTON_HEIGHT + BUTTON_GAP) * menu_buttons.len() as f32 * 0.5;
            ui.spawn((
                Name::new("Settings boundary"),
                UiLayout::boundary()
                    .pos1((Rl(50.) - Ab(x_offset), Rl(50.) - Ab(y_offset)))
                    .pos2((Rl(50.) + Ab(x_offset), Rl(50.) + Ab(y_offset)))
                    .pack(),
            ))
            .with_children(|ui| {
                let mut offset = 0.;
                for button in menu_buttons {
                    let mut button_cmds = ui.spawn((
                        Name::new(button),
                        UiLayout::window()
                            .y(Ab(offset))
                            .size((Rl(100.), Ab(BUTTON_HEIGHT)))
                            .pack(),
                        OnHoverSetCursor::new(bevy::window::SystemCursorIcon::Pointer),
                    ));
                    button_cmds
                        .with_children(|p| {
                            p.spawn((
                                UiLayout::new(vec![
                                    (UiBase::id(), UiLayout::window().full()),
                                    (UiHover::id(), UiLayout::window().size(Rl(110.)).pos(Rl(-5.))),
                                ]),
                                UiHover::new().forward_speed(40.).backward_speed(8.),
                                PickingBehavior::IGNORE,
                            ))
                            .with_children(|p| {
                                p.spawn((
                                    Name::new("Button background"),
                                    UiLayout::window()
                                        .pos(Rl((49., 65.)))
                                        .size(Rl((100., 300.)))
                                        .anchor(Anchor::Center)
                                        .pack(),
                                    UiDepth::Add(-1.),
                                    Sprite::from_image(asset_server.load("button_border.png")),
                                    PickingBehavior::IGNORE,
                                ))
                                .with_children(|p| {
                                    p.spawn((
                                        UiLayout::window().full().pack(),
                                        UiDepth::Add(-0.5),
                                        UiColor::new(vec![
                                            (UiBase::id(), Color::srgb(0.2, 0., 0.)),
                                            (UiHover::id(), Color::srgb(1., 0., 0.)),
                                        ]),
                                        UiHover::new().forward_speed(40.).backward_speed(8.),
                                        Sprite::from_image(asset_server.load("button_highlight.png")),
                                        PickingBehavior::IGNORE,
                                    ));
                                });
                                let mut text = p.spawn((
                                    UiLayout::window().pos(Rl(50.)).anchor(Anchor::Center).pack(),
                                    UiTextSize::from(Rh(80.)),
                                    TextFont {
                                        font_size: 60.,
                                        ..Default::default()
                                    },
                                    Text2d::new(button),
                                    PickingBehavior::IGNORE,
                                ));
                                match button {
                                    v if v == Settings::SFX_VARIANTS[0] || v == Settings::SFX_VARIANTS[1] => {
                                        text.insert(SfxMarker);
                                    }
                                    v if v == Settings::SOUNDTRACK_VARIANTS[0]
                                        || v == Settings::SOUNDTRACK_VARIANTS[1] =>
                                    {
                                        text.insert(SoundtrackMarker);
                                    }
                                    _ => {}
                                }
                            });
                        })
                        .observe(hover_set::<Pointer<Over>, true>)
                        .observe(hover_set::<Pointer<Out>, false>)
                        .observe(
                            |_: Trigger<Pointer<Click>>,
                             settings: Res<Settings>,
                             asset_server: Res<AssetServer>,
                             mut commands: Commands| {
                                if settings.sfx_enabled {
                                    commands.spawn(AudioPlayer::new(asset_server.load("sfx/Cloud Click.ogg")));
                                }
                            },
                        );

                    match button {
                        v if v == Settings::SFX_VARIANTS[0] || v == Settings::SFX_VARIANTS[1] => {
                            button_cmds.observe(
                                |_: Trigger<Pointer<Click>>,
                                 mut settings: ResMut<Settings>,
                                 mut text: Single<&mut Text2d, With<SfxMarker>>| {
                                    settings.sfx_enabled = !settings.sfx_enabled;
                                    text.0 = settings.sfx_label().to_string();
                                },
                            );
                        }

                        v if v == Settings::SOUNDTRACK_VARIANTS[0] || v == Settings::SOUNDTRACK_VARIANTS[1] => {
                            button_cmds.observe(
                                |_: Trigger<Pointer<Click>>,
                                 mut settings: ResMut<Settings>,
                                 mut events: EventWriter<SoundtrackToggled>,
                                 mut text: Single<&mut Text2d, With<SoundtrackMarker>>| {
                                    settings.soundtrack_enabled = !settings.soundtrack_enabled;
                                    text.0 = settings.soundtrack_label().to_string();
                                    events.send(SoundtrackToggled);
                                },
                            );
                        }
                        "Return" => {
                            button_cmds.observe(
                                |_: Trigger<Pointer<Click>>, mut next_state: ResMut<NextState<MenuState>>| {
                                    next_state.set(MenuState::MainMenu)
                                },
                            );
                        }
                        _ => {}
                    }
                    offset += BUTTON_HEIGHT + BUTTON_GAP;
                }
            });
        });
}
