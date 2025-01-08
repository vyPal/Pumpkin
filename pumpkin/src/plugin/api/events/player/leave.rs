use std::sync::Arc;

use pumpkin_core::text::TextComponent;

use crate::{
    entity::player::Player,
    plugin::{CancellableEvent, Event},
};

use super::{PlayerEvent, PlayerLeaveEvent};

pub struct PlayerLeaveEventImpl {
    player: Arc<Player>,
    leave_message: TextComponent,
    is_cancelled: bool,
}

impl PlayerLeaveEventImpl {
    pub fn new(player: Arc<Player>, leave_message: TextComponent) -> Self {
        Self {
            player,
            leave_message,
            is_cancelled: false,
        }
    }
}

impl PlayerLeaveEvent for PlayerLeaveEventImpl {
    fn get_leave_message(&self) -> TextComponent {
        self.leave_message.clone()
    }

    fn set_leave_message(&mut self, message: TextComponent) {
        self.leave_message = message;
    }
}

impl PlayerEvent for PlayerLeaveEventImpl {
    fn get_player(&self) -> &Player {
        &self.player
    }
}

impl CancellableEvent for PlayerLeaveEventImpl {
    fn is_cancelled(&self) -> bool {
        self.is_cancelled
    }

    fn set_cancelled(&mut self, cancelled: bool) {
        self.is_cancelled = cancelled;
    }
}

impl Event for PlayerLeaveEventImpl {
    fn get_name_static() -> &'static str {
        "PlayerLeaveEvent"
    }

    fn get_name(&self) -> &'static str {
        "PlayerLeaveEvent"
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
