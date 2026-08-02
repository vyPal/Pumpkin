use crate::wit::pumpkin::plugin::event::{Event, EntityRegainHealthEventData, EventType};

use super::super::FromIntoEvent;

/// An event that occurs when any entity's health increases (a heal).
///
/// The associated [`EntityRegainHealthEventData`] contains the victim entity, the heal
/// amount (which can be modified), and a `reason` distinguishing natural regeneration,
/// eating, potions/effects, plugin-initiated heals, and other sources. This event is
/// cancellable; cancelling it prevents the heal from being applied at all.
pub struct EntityRegainHealthEvent;

impl FromIntoEvent for EntityRegainHealthEvent {
    const EVENT_TYPE: EventType = EventType::EntityRegainHealthEvent;
    type Data = EntityRegainHealthEventData;

    fn data_from_event(event: Event) -> Self::Data {
        match event {
            Event::EntityRegainHealthEvent(data) => data,
            _ => panic!("unexpected event"),
        }
    }

    fn data_into_event(data: Self::Data) -> Event {
        Event::EntityRegainHealthEvent(data)
    }
}
