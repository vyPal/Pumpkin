use crate::wit::pumpkin::plugin::event::{Event, EntityDamageEventData, EventType};

use super::super::FromIntoEvent;

/// An event that occurs when any entity takes damage, before it is applied.
///
/// The associated [`EntityDamageEventData`] contains the victim, the damage amount (which can
/// be modified by a handler), the damage type, and the optional `source`/`cause` entities
/// (the immediate source of damage and the ultimate attacker, respectively). This event is
/// cancellable; cancelling it prevents the damage from being applied at all.
pub struct EntityDamageEvent;

impl FromIntoEvent for EntityDamageEvent {
    const EVENT_TYPE: EventType = EventType::EntityDamageEvent;
    type Data = EntityDamageEventData;

    fn data_from_event(event: Event) -> Self::Data {
        match event {
            Event::EntityDamageEvent(data) => data,
            _ => panic!("unexpected event"),
        }
    }

    fn data_into_event(data: Self::Data) -> Event {
        Event::EntityDamageEvent(data)
    }
}
