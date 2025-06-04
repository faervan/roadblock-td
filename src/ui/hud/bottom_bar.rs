use bevy::prelude::*;
use bevy_lunex::{Ab, Align, Rl, UiFetchFromCamera, UiLayout, UiLayoutRoot};
use player_health::update_player_health;

use crate::app_state::AppState;

pub struct BottomBarPlugin;

impl Plugin for BottomBarPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<player_health::PlayerHpCircle>()
            .register_type::<player_health::PlayerHpTextMarker>()
            .add_systems(OnEnter(AppState::Game), build_ui)
            .add_systems(
                Update,
                update_player_health.run_if(in_state(AppState::Game)),
            );
    }
}

fn build_ui(
    camera: Single<Entity, With<Camera>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands
        .spawn((
            Name::new("Bottom bar root"),
            UiLayoutRoot::new_2d(),
            UiFetchFromCamera::<0>,
            ChildOf(*camera),
        ))
        .with_children(|ui| {
            // Spawn boundary node
            ui.spawn((
                Name::new("Bottom bar boundary"),
                UiLayout::boundary()
                    .pos1((Ab(0.), Rl(100.) - Ab(150.)))
                    .pos2(Rl(100.))
                    .pack(),
            ))
            .with_children(|ui| {
                ui.spawn((
                    Name::new("Tower selection"),
                    // Aspect ratio, not size
                    UiLayout::solid()
                        .size((Rl(50.) - Ab(75.), Rl(100.)))
                        .align_x(Align::LEFT)
                        .pack(),
                ))
                .with_children(|p| tower_selection::build(p, &mut materials));
                ui.spawn((Name::new("Player health bar"), UiLayout::solid().pack()))
                    .with_children(|p| {
                        player_health::build(p, &mut materials, &mut meshes)
                    });
            });
        });
}

mod tower_selection {
    use bevy::{prelude::*, window::SystemCursorIcon};
    use bevy_lunex::{
        Ab, OnHoverSetCursor, Rl, UiBase, UiColor, UiHover, UiLayout, UiMeshPlane2d,
        UiStateTrait, hover_set,
    };

    use crate::{
        Settings,
        app_state::TowerPlacingState,
        tower::{SelectedTower, Tower, TowerType},
        ui::helpers::ui_hover_state,
    };

    // I would have automated this but I don't think it is possible :/
    const TYPES: [TowerType; 3] =
        [TowerType::Wall, TowerType::SpikedWall, TowerType::Canon];
    const TILE_SIZE_PX: f32 = 30.0;

    const BUTTON_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);
    const BUTTON_COLOR_HOVER: Color = Color::srgb(1., 0., 0.);

    pub fn build(
        builder: &mut ChildSpawnerCommands,
        materials: &mut Assets<ColorMaterial>,
    ) {
        for (index, tower) in TYPES.iter().enumerate() {
            builder
                .spawn((
                    Name::new(format!("TowerButton: {tower:?}")),
                    UiLayout::new(vec![
                        (
                            UiBase::id(),
                            UiLayout::window()
                                .pos((Ab(50.) + Ab(200.) * index as f32, Ab(0.)))
                                .size(Ab(150.)),
                        ),
                        (
                            UiHover::id(),
                            UiLayout::window()
                                .pos((Ab(40.) + Ab(200.) * index as f32, Ab(-10.)))
                                .size(Ab(170.)),
                        ),
                    ]),
                    UiColor::new(vec![
                        (UiBase::id(), BUTTON_COLOR),
                        (UiHover::id(), BUTTON_COLOR_HOVER),
                    ]),
                    UiHover::new().forward_speed(20.).backward_speed(5.),
                    Sprite::default(),
                    OnHoverSetCursor::new(SystemCursorIcon::Pointer),
                ))
                .with_children(|p| {
                    let icon_width = tower.size().0 as f32 * TILE_SIZE_PX;
                    let icon_height = tower.size().1 as f32 * TILE_SIZE_PX;
                    let icon_margin_x = 75. - 0.5 * icon_width;
                    let icon_margin_y = 75. - 0.5 * icon_height;
                    p.spawn((
                        UiLayout::boundary()
                            .pos1((Ab(icon_margin_x), Ab(icon_margin_y)))
                            // This panics when replacing Ab(150.) with Rl(100.), even though the
                            // parent indeed has a size of 150x150
                            // (error msg: Rect size has to be positive)
                            .pos2((
                                Rl(100.) - Ab(icon_margin_x),
                                Rl(100.) - Ab(icon_margin_y),
                            ))
                            .pack(),
                        Pickable::IGNORE,
                    ))
                    .with_child((
                        UiLayout::solid()
                            .size((Ab(icon_width), Ab(icon_height)))
                            .pack(),
                        UiMeshPlane2d,
                        MeshMaterial2d(materials.add(Color::srgb(0., 0.5, 1.))),
                        Pickable::IGNORE,
                    ));
                })
                .observe(hover_set::<Pointer<Over>, true>)
                .observe(hover_set::<Pointer<Out>, false>)
                .observe(ui_hover_state::<Pointer<Over>, true>)
                .observe(ui_hover_state::<Pointer<Out>, false>)
                .observe(
                    |_: Trigger<Pointer<Click>>,
                     mut selection: ResMut<SelectedTower>,
                     mut next_state: ResMut<NextState<TowerPlacingState>>,
                     asset_server: Res<AssetServer>,
                     mut commands: Commands,
                     settings: Res<Settings>| {
                        if settings.sfx_enabled {
                            commands.spawn(AudioPlayer::new(
                                asset_server.load("sfx/Cloud Click.ogg"),
                            ));
                        }
                        selection.0 = Tower::new(*tower, selection.orientation);
                        next_state.set(TowerPlacingState::Placing);
                    },
                );
        }
    }
}

mod player_health {
    use std::f32::consts::PI;

    use bevy::prelude::*;
    use bevy_lunex::{Rh, UiLayout, UiTextSize};

    use crate::{enemy::EnemyGoal, health::Health, ui::helpers::ui_hover_state};

    #[derive(Component, Reflect)]
    #[reflect(Component)]
    pub struct PlayerHpCircle(f32);

    #[derive(Component, Reflect)]
    #[reflect(Component)]
    pub struct PlayerHpTextMarker;

    pub fn build(
        builder: &mut ChildSpawnerCommands,
        materials: &mut Assets<ColorMaterial>,
        meshes: &mut Assets<Mesh>,
    ) {
        builder
            .spawn((
                Name::new("Player health"),
                UiLayout::solid().pack(),
                Mesh2d(meshes.add(Circle::new(75.))),
                MeshMaterial2d(materials.add(Color::BLACK)),
            ))
            .with_children(|p| {
                p.spawn((
                    PlayerHpCircle(75.),
                    UiLayout::solid().pack(),
                    Mesh2d(meshes.add(CircularSector::new(75., PI))),
                    MeshMaterial2d(materials.add(Color::srgb(0., 255., 0.))),
                    Pickable::IGNORE,
                ))
                .with_child((
                    Transform::from_translation(Vec3::Z),
                    Mesh2d(meshes.add(Circle::new(65.))),
                    MeshMaterial2d(materials.add(Color::BLACK)),
                    Pickable::IGNORE,
                ));
                p.spawn((
                    PlayerHpTextMarker,
                    Transform::from_translation(Vec3::Z * 5.),
                    UiTextSize::from(Rh(5.)),
                    Text2d::new("Full health"),
                    TextFont {
                        font_size: 25.,
                        ..default()
                    },
                    Pickable::IGNORE,
                ));
            })
            .observe(ui_hover_state::<Pointer<Over>, true>)
            .observe(ui_hover_state::<Pointer<Out>, false>);
    }

    pub fn update_player_health(
        query: Query<&Health, (With<EnemyGoal>, Changed<Health>)>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut hp_circle: Single<(Entity, &PlayerHpCircle, &mut Transform)>,
        mut hp_text: Single<&mut Text2d, With<PlayerHpTextMarker>>,
        mut commands: Commands,
    ) {
        if let Ok(health) = query.single() {
            commands.entity(hp_circle.0).insert(Mesh2d(
                meshes.add(CircularSector::new(hp_circle.1.0, health.percentage() * PI)),
            ));
            hp_circle.2.rotation = Quat::from_rotation_z(health.percentage() * PI);
            hp_text.0 = format!("{}/{}", health.current, health.max);
        }
    }
}
