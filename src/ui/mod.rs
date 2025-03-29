use bevy::prelude::*;

use crate::tower::{SelectedTower, TowerPlaceState, TowerType};

// I would have automated this but I don't think it is possible :/
const TYPES: [TowerType; 3] = [TowerType::Wall, TowerType::SpikedWall, TowerType::Canon];
const TILE_SIZE_PX: f32 = 30.0;

const BACKGROUND_COLOR: Color = Color::srgba(0.0, 0.0, 0.0, 0.5);
const BUTTON_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);
const BUTTON_HOVER_COLOR: Color = Color::srgb(0.2, 0.2, 0.2);
const BUTTON_PRESS_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_ui);
        app.add_systems(Update, handle_buttons);
        app.register_type::<TowerButton>();
    }
}

type ButtonQueryData<'a, T> = (&'a Interaction, &'a T, &'a mut BackgroundColor);
type ButtonQueryFilter = (Changed<Interaction>, With<Button>);

#[derive(Reflect, Component)]
struct TowerButton(TowerType);

fn init_ui(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(200.0),
                align_items: AlignItems::Center,
                align_self: AlignSelf::FlexEnd,
                ..default()
            },
            BackgroundColor(BACKGROUND_COLOR),
        ))
        .with_children(|parent| {
            for tower in TYPES {
                parent
                    .spawn((
                        TowerButton(tower),
                        Button,
                        Node {
                            width: Val::Px(190.0),
                            height: Val::Px(190.0),
                            margin: UiRect {
                                left: Val::Px(10.0),
                                right: Val::Px(10.0),
                                top: Val::Px(10.0),
                                bottom: Val::Px(10.0),
                            },
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(BUTTON_COLOR),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            // Replace this with ImageNode when we get textures for Towers
                            Node {
                                width: Val::Px(tower.size().0 as f32 * TILE_SIZE_PX),
                                height: Val::Px(tower.size().1 as f32 * TILE_SIZE_PX),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.0, 0.5, 1.0)),
                        ));
                    });
            }
        });
}

fn handle_buttons(
    mut button: Query<ButtonQueryData<TowerButton>, ButtonQueryFilter>,
    mut selection: ResMut<SelectedTower>,
    mut next_state: ResMut<NextState<TowerPlaceState>>,
) {
    for (interaction, tower, mut color) in button.iter_mut() {
        match interaction {
            Interaction::Hovered => *color = BackgroundColor(BUTTON_HOVER_COLOR),
            Interaction::Pressed => {
                *color = BackgroundColor(BUTTON_PRESS_COLOR);
                selection.variant = tower.0;
                next_state.set(TowerPlaceState::Active);
            }
            _ => *color = BackgroundColor(BUTTON_COLOR),
        }
    }
}
