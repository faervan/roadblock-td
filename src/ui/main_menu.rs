use bevy::prelude::*;

use crate::app_state::{AppState, MenuState};

use super::{NORMAL_BUTTON, TEXT_COLOR, button, button_text_font, despawn_menu};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MainMenuAction>()
            .register_type::<MainMenuMarker>()
            .add_systems(OnEnter(MenuState::MainMenu), build_main_menu)
            .add_systems(OnExit(MenuState::MainMenu), despawn_menu::<MainMenuMarker>)
            .add_systems(Update, menu_action.run_if(in_state(MenuState::MainMenu)));
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
enum MainMenuAction {
    Play,
    OpenSettings,
    Quit,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct MainMenuMarker;

const BUTTON_WIDTH: f32 = 300.;

fn build_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                        button(BUTTON_WIDTH),
                        BackgroundColor(NORMAL_BUTTON),
                        MainMenuAction::Play,
                    ))
                    .with_child((
                        Text::new("Play"),
                        button_text_font(),
                        TextColor(TEXT_COLOR),
                    ));

                    // Settings button
                    p.spawn((
                        Button,
                        button(BUTTON_WIDTH),
                        BackgroundColor(NORMAL_BUTTON),
                        MainMenuAction::OpenSettings,
                    ))
                    .with_child((
                        Text::new("Settings"),
                        button_text_font(),
                        TextColor(TEXT_COLOR),
                    ));

                    // Quit button
                    p.spawn((
                        Button,
                        button(BUTTON_WIDTH),
                        BackgroundColor(NORMAL_BUTTON),
                        MainMenuAction::Quit,
                    ))
                    .with_child((
                        Text::new("Quit"),
                        button_text_font(),
                        TextColor(TEXT_COLOR),
                    ));
                });

            p.spawn((
                Sprite::from_image(asset_server.load("title_image.png")),
                Transform::from_xyz(-960., -540., -1.),
            ));
        });
}

fn menu_action(
    interaction_query: Query<(&Interaction, &MainMenuAction), (Changed<Interaction>, With<Button>)>,
    mut app_exit_events: EventWriter<AppExit>,
    mut app_state: ResMut<NextState<AppState>>,
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MainMenuAction::Play => {
                    app_state.set(AppState::Game);
                }
                MainMenuAction::OpenSettings => {
                    menu_state.set(MenuState::Settings);
                }
                MainMenuAction::Quit => {
                    app_exit_events.send(AppExit::Success);
                }
            }
        }
    }
}
