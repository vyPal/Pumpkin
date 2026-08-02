use crate::wit::pumpkin::plugin::event::{Event, EventType, PlayerDeathEventData};

use super::super::FromIntoEvent;

/// An event that occurs when a player dies.
///
/// The associated [`PlayerDeathEventData`] contains the victim player, the optional killer
/// entity, the death message (which can be modified), and the drops (which can also be
/// modified). This event is not cancellable — by the time it fires, death is already
/// committed.
pub struct PlayerDeathEvent;

impl FromIntoEvent for PlayerDeathEvent {
    const EVENT_TYPE: EventType = EventType::PlayerDeathEvent;
    type Data = PlayerDeathEventData;

    fn data_from_event(event: Event) -> Self::Data {
        match event {
            Event::PlayerDeathEvent(data) => data,
            _ => panic!("unexpected event"),
        }
    }

    fn data_into_event(data: Self::Data) -> Event {
        Event::PlayerDeathEvent(data)
    }
}
