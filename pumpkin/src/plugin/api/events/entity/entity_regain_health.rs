use std::sync::Arc;

use crate::entity::EntityBase;
use crate::entity::living::HealReason;
use pumpkin_macros::{Event, cancellable};

use super::EntityEvent;

/// Event that is triggered whenever an entity's health increases (a heal), before it
/// is applied.
///
/// The `reason` field distinguishes natural regeneration (saturation-based) from
/// eating, potions/effects, plugin-initiated heals, and other sources — letting a
/// handler block only natural regen (e.g. for a UHC-style "no regen" ruleset) while
/// leaving other heals untouched. Cancelling this event prevents the heal from being
/// applied at all; modifying `amount` changes how much health is restored.
#[cancellable]
#[derive(Event, Clone)]
pub struct EntityRegainHealthEvent {
    /// The entity being healed.
    pub victim: Arc<dyn EntityBase>,

    /// The amount of health to be restored. Can be modified by a handler.
    pub amount: f32,

    /// The reason for the heal.
    pub reason: HealReason,
}

impl EntityEvent for EntityRegainHealthEvent {
    fn get_victim(&self) -> &Arc<dyn EntityBase> {
        &self.victim
    }
}
