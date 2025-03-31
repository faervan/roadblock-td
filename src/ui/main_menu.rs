use bevy::prelude::*;

use crate::app_state::AppState;

use super::despawn_menu;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Menu), build_main_menu)
            .add_systems(OnExit(AppState::Menu), despawn_menu::<MainMenuMarker>)
            .add_systems(
                Update,
                (menu_action, button_interaction).run_if(in_state(AppState::Menu)),
            );
    }
}

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

#[derive(Component)]
enum MainMenuAction {
    Play,
    Quit,
}

#[derive(Component)]
struct MainMenuMarker;

fn build_main_menu(mut commands: Commands) {
    let button_node = Node {
        width: Val::Px(300.0),
        height: Val::Px(65.0),
        margin: UiRect::axes(Val::Px(20.), Val::Px(10.)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_font = TextFont {
        font_size: 33.0,
        ..default()
    };

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            MainMenuMarker,
        ))
        .with_children(|p| {
            p.spawn((Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::vertical(Val::Px(10.)),
                ..default()
            },))
                .with_children(|p| {
                    // Play button
                    p.spawn((
                        Button,
                        button_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        MainMenuAction::Play,
                    ))
                    .with_child((
                        Text::new("Play"),
                        button_text_font.clone(),
                        TextColor(TEXT_COLOR),
                    ));

                    // Quit button
                    p.spawn((
                        Button,
                        button_node,
                        BackgroundColor(NORMAL_BUTTON),
                        MainMenuAction::Quit,
                    ))
                    .with_child((
                        Text::new("Quit"),
                        button_text_font,
                        TextColor(TEXT_COLOR),
                    ));
                });
        });
}

fn menu_action(
    interaction_query: Query<(&Interaction, &MainMenuAction), (Changed<Interaction>, With<Button>)>,
    mut app_exit_events: EventWriter<AppExit>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MainMenuAction::Play => {
                    app_state.set(AppState::in_game());
                }
                MainMenuAction::Quit => {
                    app_exit_events.send(AppExit::Success);
                }
            }
        }
    }
}

fn button_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background_color) in &mut interaction_query {
        *background_color = match *interaction {
            Interaction::Pressed => PRESSED_BUTTON.into(),
            Interaction::Hovered => HOVERED_BUTTON.into(),
            Interaction::None => NORMAL_BUTTON.into(),
        }
    }
}
