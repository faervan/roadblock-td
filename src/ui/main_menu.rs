use bevy::prelude::*;
use bevy_lunex::{
    Ab, Anchor, OnHoverSetCursor, Rh, Rl, Scaling, UiBase, UiColor, UiDepth, UiFetchFromCamera, UiHover, UiLayout,
    UiLayoutRoot, UiStateTrait, UiTextSize, hover_set,
};

use crate::{
    Settings,
    app_state::{AppState, MenuState},
};

use super::despawn_menu;
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MainMenuMarker>()
            .add_systems(OnEnter(MenuState::MainMenu), build_ui)
            .add_systems(OnExit(MenuState::MainMenu), despawn_menu::<MainMenuMarker>);
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct MainMenuMarker;

const MENU_BUTTONS: &[&str] = &["Play", "Settings", "Quit"];
const BUTTON_WIDTH: f32 = 450.;
const BUTTON_HEIGHT: f32 = 60.;
const BUTTON_GAP: f32 = 50.;

fn build_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Name::new("Main menu root"),
            UiLayoutRoot::new_2d(),
            UiFetchFromCamera::<0>,
            MainMenuMarker,
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
            let y_offset = (BUTTON_HEIGHT + BUTTON_GAP) * MENU_BUTTONS.len() as f32 * 0.5;
            ui.spawn((
                Name::new("Main menu boundary"),
                UiLayout::boundary()
                    .pos1((Rl(50.) - Ab(x_offset), Rl(50.) - Ab(y_offset)))
                    .pos2((Rl(50.) + Ab(x_offset), Rl(50.) + Ab(y_offset)))
                    .pack(),
            ))
            .with_children(|ui| {
                let mut offset = 0.;
                for button in MENU_BUTTONS {
                    let mut button_cmds = ui.spawn((
                        Name::new(*button),
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
                                p.spawn((
                                    UiLayout::window().pos(Rl(50.)).anchor(Anchor::Center).pack(),
                                    UiTextSize::from(Rh(80.)),
                                    TextFont {
                                        font_size: 60.,
                                        ..Default::default()
                                    },
                                    Text2d::new(*button),
                                    PickingBehavior::IGNORE,
                                ));
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
                    match *button {
                        "Play" => {
                            button_cmds.observe(
                                |_: Trigger<Pointer<Click>>, mut next_state: ResMut<NextState<AppState>>| {
                                    next_state.set(AppState::Game)
                                },
                            );
                        }
                        "Settings" => {
                            button_cmds.observe(
                                |_: Trigger<Pointer<Click>>, mut next_state: ResMut<NextState<MenuState>>| {
                                    next_state.set(MenuState::Settings)
                                },
                            );
                        }
                        "Quit" => {
                            button_cmds.observe(|_: Trigger<Pointer<Click>>, mut app_exit: EventWriter<AppExit>| {
                                app_exit.send(AppExit::Success);
                            });
                        }
                        _ => {}
                    }
                    offset += BUTTON_HEIGHT + BUTTON_GAP;
                }
            });
        });
}
