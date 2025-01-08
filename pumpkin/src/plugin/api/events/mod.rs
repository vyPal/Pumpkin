use std::any::Any;

pub mod block;
pub mod player;

pub trait Event: Any + Send + Sync {
    fn get_name_static() -> &'static str
    where
        Self: Sized;
    fn get_name(&self) -> &'static str;
    fn as_any(&mut self) -> &mut dyn Any;
}

pub trait CancellableEvent: Event {
    fn is_cancelled(&self) -> bool;
    fn set_cancelled(&mut self, cancelled: bool);
}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
// Lowest priority events are executed first, so that higher priority events can override their changes
pub enum EventPriority {
    Highest,
    High,
    Normal,
    Low,
    Lowest,
}
