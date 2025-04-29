use bevy::prelude::*;

use crate::{
    app_state::GameState,
    game_loop::GameStatistics,
    ui::{TEXT_COLOR, despawn_menu, helpers::ui_hover_state},
};

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GameOverMarker>()
            .add_systems(OnEnter(GameState::GameOver), build_game_over_info)
            .add_systems(OnExit(GameState::GameOver), despawn_menu::<GameOverMarker>);
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct GameOverMarker;

fn stat_row(p: &mut ChildBuilder, key: impl ToString, value: impl ToString) {
    p.spawn(Node {
        width: Val::Percent(100.),
        justify_content: JustifyContent::SpaceBetween,
        flex_direction: FlexDirection::Row,
        ..Default::default()
    })
    .with_children(|p| {
        p.spawn((
            Text(key.to_string()),
            TextFont {
                font_size: 40.,
                ..Default::default()
            },
            TextColor(TEXT_COLOR),
        ));
        p.spawn((
            Text(value.to_string()),
            TextFont {
                font_size: 40.,
                ..Default::default()
            },
            TextColor(TEXT_COLOR),
        ));
    });
}

fn build_game_over_info(mut commands: Commands, stats: Res<GameStatistics>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            GameOverMarker,
        ))
        .with_children(|p| {
            p.spawn((
                Node {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(70.),
                    padding: UiRect::axes(Val::Px(40.), Val::Px(20.)),
                    ..Default::default()
                },
                BackgroundColor(Color::srgba(0., 0., 0., 0.85)),
            ))
            .with_children(|p| {
                p.spawn((
                    Text::new("Game over!"),
                    TextFont {
                        font_size: 80.,
                        ..Default::default()
                    },
                    TextColor(TEXT_COLOR),
                ));

                stat_row(
                    p,
                    "Time elapsed:",
                    format!(
                        "{}min {}sec",
                        (stats.time.elapsed_secs() / 60.).round(),
                        (stats.time.elapsed_secs() % 60.).round()
                    ),
                );
                stat_row(p, "Enemies killed:", stats.enemies_killed);
                stat_row(p, "Money earned:", stats.money_earned);
                stat_row(p, "Money spend:", stats.money_spend);
            })
            .observe(ui_hover_state::<Pointer<Over>, true>)
            .observe(ui_hover_state::<Pointer<Out>, false>);
        });
}
