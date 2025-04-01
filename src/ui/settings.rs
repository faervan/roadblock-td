use bevy::prelude::*;

use crate::{Settings, app_state::MenuState, soundtrack::SoundtrackToggled};

use super::{NORMAL_BUTTON, TEXT_COLOR, button, button_text_font, despawn_menu};

pub struct SettingsMenuPlugin;

impl Plugin for SettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MenuState::Settings), build_settings_menu)
            .add_systems(OnExit(MenuState::Settings), despawn_menu::<SettingsMarker>)
            .add_systems(Update, menu_action.run_if(in_state(MenuState::Settings)));
    }
}

#[derive(Component)]
enum SettingsAction {
    ToggleSfx,
    ToggleSoundtrack,
    Return,
}

#[derive(Component)]
struct SettingsMarker;

const BUTTON_WIDTH: f32 = 400.;

fn build_settings_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<Settings>,
) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            SettingsMarker,
        ))
        .with_children(|p| {
            p.spawn((Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::vertical(Val::Px(10.)),
                ..default()
            },))
                .with_children(|p| {
                    // ToggleSfx button
                    p.spawn((
                        Button,
                        button(BUTTON_WIDTH),
                        BackgroundColor(NORMAL_BUTTON),
                        SettingsAction::ToggleSfx,
                    ))
                    .with_child((
                        Text::new(settings.sfx_label()),
                        button_text_font(),
                        TextColor(TEXT_COLOR),
                    ));

                    // ToggleSoundtrack button
                    p.spawn((
                        Button,
                        button(BUTTON_WIDTH),
                        BackgroundColor(NORMAL_BUTTON),
                        SettingsAction::ToggleSoundtrack,
                    ))
                    .with_child((
                        Text::new(settings.soundtrack_label()),
                        button_text_font(),
                        TextColor(TEXT_COLOR),
                    ));

                    // Return button
                    p.spawn((
                        Button,
                        button(BUTTON_WIDTH),
                        BackgroundColor(NORMAL_BUTTON),
                        SettingsAction::Return,
                    ))
                    .with_child((
                        Text::new("Return"),
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
    interaction_query: Query<
        (&Interaction, &SettingsAction, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut settings: ResMut<Settings>,
    mut soundtrack_event: EventWriter<SoundtrackToggled>,
) {
    for (interaction, menu_button_action, children) in &interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                SettingsAction::ToggleSfx => {
                    settings.sfx_enabled = !settings.sfx_enabled;
                    **text = settings.sfx_label().to_string();
                }
                SettingsAction::ToggleSoundtrack => {
                    settings.soundtrack_enabled = !settings.soundtrack_enabled;
                    **text = settings.soundtrack_label().to_string();
                    soundtrack_event.send(SoundtrackToggled);
                }
                SettingsAction::Return => menu_state.set(MenuState::MainMenu),
            }
        }
    }
}
