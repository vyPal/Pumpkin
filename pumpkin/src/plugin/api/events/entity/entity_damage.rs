use std::sync::Arc;

use crate::entity::EntityBase;
use pumpkin_data::damage::DamageType;
use pumpkin_macros::{Event, cancellable};
use pumpkin_util::math::vector3::Vector3;

use super::EntityEvent;

/// Event that is triggered whenever an entity takes damage, before it is applied.
///
/// Fired from the single choke point all entity damage funnels through
/// (`LivingEntity::damage_with_context`), so this covers `PvP`, mob damage,
/// environmental damage, and projectiles alike. `source` is the immediate cause of the
/// damage (e.g. an arrow), while `cause` is the ultimate responsible entity (e.g. the
/// player who shot it) — both may be absent for damage with no attributable entity
/// (e.g. fall damage, fire, starvation). Cancelling this event prevents the damage
/// from being applied at all; modifying `amount` changes the damage before mitigation
/// (armor, enchantments, resistance) is applied.
#[cancellable]
#[derive(Event, Clone)]
pub struct EntityDamageEvent {
    /// The entity taking damage.
    pub victim: Arc<dyn EntityBase>,

    /// The amount of damage, before mitigation. Can be modified by a handler.
    pub amount: f32,

    /// The type of damage.
    pub damage_type: DamageType,

    /// The immediate source of the damage (e.g. an arrow), if any.
    pub source: Option<Arc<dyn EntityBase>>,

    /// The ultimate entity responsible for the damage (e.g. the player who shot the
    /// arrow), if any.
    pub cause: Option<Arc<dyn EntityBase>>,

    /// The position the damage originated from, if applicable.
    pub position: Option<Vector3<f64>>,
}

impl EntityEvent for EntityDamageEvent {
    fn get_victim(&self) -> &Arc<dyn EntityBase> {
        &self.victim
    }
}
