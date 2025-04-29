use bevy::prelude::*;

use crate::app_state::UiHoverState;

pub fn ui_hover_state<TRIGGER: Event, const ACTIVE: bool>(
    _: Trigger<TRIGGER>,
    mut next_state: ResMut<NextState<UiHoverState>>,
) {
    next_state.set(match ACTIVE {
        true => UiHoverState::Hovering,
        false => UiHoverState::None,
    });
}
