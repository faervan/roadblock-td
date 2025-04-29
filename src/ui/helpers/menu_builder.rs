use bevy::prelude::*;
use bevy_lunex::{
    Ab, Anchor, OnHoverSetCursor, Rh, Rl, Scaling, UiBase, UiColor, UiDepth,
    UiFetchFromCamera, UiHover, UiLayout, UiLayoutRoot, UiStateTrait, UiTextSize,
    hover_set,
};

use crate::Settings;

pub fn build_menu<T: Component, F>(
    commands: &mut Commands,
    asset_server: &AssetServer,
    menu_name: &str,
    marker: T,
    button_width: f32,
    button_height: f32,
    button_gap: f32,
    buttons: Vec<(&'static str, F, Option<F>)>,
) where
    F: Fn(&mut EntityCommands),
{
    commands
        .spawn((
            Name::new(format!("{menu_name} root")),
            UiLayoutRoot::new_2d(),
            UiFetchFromCamera::<0>,
            marker,
        ))
        .with_children(|ui| {
            ui.spawn((
                Name::new(format!("{menu_name} image")),
                UiLayout::solid()
                    .size((Ab(1920.), Ab(1080.)))
                    .scaling(Scaling::Fill)
                    .pack(),
                Sprite::from_image(asset_server.load("title_image.png")),
            ));

            let x_offset = button_width / 2.;
            let y_offset = (button_height + button_gap) * buttons.len() as f32 * 0.5;
            ui.spawn((
                Name::new(format!("{menu_name} boundary")),
                UiLayout::boundary()
                    .pos1((Rl(50.) - Ab(x_offset), Rl(50.) - Ab(y_offset)))
                    .pos2((Rl(50.) + Ab(x_offset), Rl(50.) + Ab(y_offset)))
                    .pack(),
            ))
            .with_children(|ui| {
                let mut offset = 0.;
                for (button, button_modifier, text_entity_modifier) in buttons {
                    let mut button_cmds = ui.spawn((
                        Name::new(button),
                        UiLayout::window()
                            .y(Ab(offset))
                            .size((Rl(100.), Ab(button_height)))
                            .pack(),
                        OnHoverSetCursor::new(bevy::window::SystemCursorIcon::Pointer),
                    ));
                    build_button(
                        &mut button_cmds,
                        asset_server,
                        button,
                        text_entity_modifier,
                    );
                    button_modifier(&mut button_cmds);
                    offset += button_height + button_gap;
                }
            });
        });
}

fn build_button<F>(
    button_cmds: &mut EntityCommands,
    asset_server: &AssetServer,
    button: &str,
    text_entity_modifier: Option<F>,
) where
    F: Fn(&mut EntityCommands),
{
    button_cmds
        .with_children(|p| {
            p.spawn((
                UiLayout::new(vec![
                    (UiBase::id(), UiLayout::window().full()),
                    (
                        UiHover::id(),
                        UiLayout::window().size(Rl(110.)).pos(Rl(-5.)),
                    ),
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
                let mut text_cmds = p.spawn((
                    UiLayout::window()
                        .pos(Rl(50.))
                        .anchor(Anchor::Center)
                        .pack(),
                    UiTextSize::from(Rh(80.)),
                    TextFont {
                        font_size: 60.,
                        ..Default::default()
                    },
                    Text2d::new(button),
                    PickingBehavior::IGNORE,
                ));
                if let Some(modifier) = text_entity_modifier {
                    modifier(&mut text_cmds);
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
                    commands.spawn(AudioPlayer::new(
                        asset_server.load("sfx/Cloud Click.ogg"),
                    ));
                }
            },
        );
}
