use bevy::prelude::*;

use crate::app_state::GameState;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Health>()
            .register_type::<HealthBar>()
            .register_type::<HasHealthBar>()
            .register_type::<NoHealthBar>()
            .add_systems(
                Update,
                (add_health_bar, adjust_health_bar).run_if(in_state(GameState::Running)),
            );
    }
}

#[derive(Component, Reflect, Deref, DerefMut, Debug)]
#[reflect(Component)]
pub struct Health {
    #[deref]
    pub current: isize,
    pub max: isize,
    offset: Vec2,
}

impl Health {
    pub fn new(max: isize, offset: Vec2) -> Self {
        Health {
            current: max,
            max,
            offset,
        }
    }

    pub fn percentage(&self) -> f32 {
        1. / self.max as f32 * self.current as f32
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct HasHealthBar {
    child: Entity,
    childs_child: Entity,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct NoHealthBar;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct HealthBar;

fn add_health_bar(
    mut commands: Commands,
    query: Query<
        (Entity, &Health),
        (Changed<Health>, Without<HasHealthBar>, Without<NoHealthBar>),
    >,
) {
    for (entity, health) in &query {
        if health.current != health.max {
            let child = commands
                .spawn((
                    Sprite::from_color(Color::BLACK, Vec2::new(40., 8.)),
                    Transform::from_translation(health.offset.extend(1.)),
                    ChildOf(entity),
                ))
                .id();
            let childs_child = commands
                .spawn((
                    HealthBar,
                    Sprite {
                        color: Color::srgb(0., 255., 0.),
                        custom_size: Some(Vec2::new(40. * health.percentage(), 8.)),
                        anchor: bevy::sprite::Anchor::CenterLeft,
                        ..Default::default()
                    },
                    Transform::from_translation(Vec3::new(-20., 0., 1.)),
                    ChildOf(child),
                ))
                .id();
            commands.entity(entity).insert(HasHealthBar {
                child,
                childs_child,
            });
        }
    }
}

fn adjust_health_bar(
    query: Query<(&HasHealthBar, &Health), Changed<Health>>,
    mut children: Query<&mut Sprite, With<HealthBar>>,
) {
    for (entity, health) in &query {
        if let Ok(mut sprite) = children.get_mut(entity.childs_child) {
            sprite.custom_size = Some(Vec2::new(40. * health.percentage(), 8.));
        }
    }
}
