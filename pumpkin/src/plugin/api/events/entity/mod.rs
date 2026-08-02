pub mod entity_damage;
pub mod entity_regain_health;
pub mod player_death;

use std::sync::Arc;

use crate::entity::EntityBase;

/// A trait representing events related to entities in general (not just players).
///
/// This trait provides a method to retrieve the entity associated with the event.
pub trait EntityEvent: Send + Sync {
    /// Retrieves a reference to the entity associated with the event.
    ///
    /// # Returns
    /// A reference to the `Arc<dyn EntityBase>` involved in the event.
    fn get_victim(&self) -> &Arc<dyn EntityBase>;
}
