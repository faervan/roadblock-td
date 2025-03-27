use bevy::prelude::*;

use crate::tower::Tower;

// I would have automated this but I don't think it is possible :/
static TYPES: [Tower; 3] = [Tower::Wall, Tower::SpikedWall, Tower::Canon];
static TILE_SIZE_PX: f32 = 30.0;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_ui);
    }
}

fn init_ui(mut commands: Commands) {
    commands.spawn(
        (Node {
            width: Val::Percent(100.0),
            height: Val::Px(200.0),
            align_items: AlignItems::Center,
            align_self: AlignSelf::FlexEnd,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5))
    )
    ).with_children(|parent| {
        for tower in TYPES.iter() {
            parent.spawn((
                Button,
                Node {
                    width: Val:: Px(190.0),
                    height: Val::Px(190.0),
                    margin: UiRect { left: Val::Px(10.0), right: Val::Px(10.0), top: Val::Px(10.0), bottom: Val::Px(10.0) },
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.2, 0.2))
            )).with_children(|parent| {
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
