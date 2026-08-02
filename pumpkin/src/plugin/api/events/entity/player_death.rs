use std::sync::Arc;

use crate::entity::EntityBase;
use crate::entity::player::Player;
use pumpkin_data::item_stack::ItemStack;
use pumpkin_macros::Event;
use pumpkin_util::text::TextComponent;

use super::super::player::PlayerEvent;

/// Event that is triggered when a player dies.
///
/// Fired once death is already committed (loot/stats/pose have already been applied
/// by the time this fires), so it is not cancellable — use
/// [`EntityDamageEvent`](crate::plugin::api::events::entity::entity_damage::EntityDamageEvent)
/// to prevent death outright by blocking the killing blow. `death_message` and `drops`
/// (the player's dropped main-inventory contents) can be modified by a handler; the
/// modified values are used for the broadcast chat message, the client death screen,
/// and what actually drops.
#[derive(Event, Clone)]
pub struct PlayerDeathEvent {
    /// The player who died.
    pub player: Arc<Player>,

    /// The entity that killed the player, if any.
    pub killer: Option<Arc<dyn EntityBase>>,

    /// The death message. Can be modified by a handler.
    pub death_message: TextComponent,

    /// The items that will be dropped. Can be modified by a handler.
    pub drops: Vec<ItemStack>,
}

impl PlayerEvent for PlayerDeathEvent {
    fn get_player(&self) -> &Arc<Player> {
        &self.player
    }
}
