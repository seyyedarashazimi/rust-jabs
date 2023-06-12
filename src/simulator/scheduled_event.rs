use crate::simulator::event::Event;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

/// The scheduled event struct, including the event, it's time , and the id
/// number
#[derive(Clone, Debug)]
pub struct ScheduledEvent<T: Event> {
    /// The event which must implement [`Event`] trait
    event: T,
    /// Simulation execution time of the event
    time: f64,
    /// Event ID (insertion number in event queue)
    number: i64,
}

impl<T: Event> Hash for ScheduledEvent<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.number.hash(state);
    }
}

impl<T: Event> PartialEq for ScheduledEvent<T> {
    fn eq(&self, other: &Self) -> bool {
        (self.number == other.number) && (self.time == other.time)
    }
}

impl<T: Event> Eq for ScheduledEvent<T> {}

impl<T: Event> PartialOrd for ScheduledEvent<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// This trait implementation is used by the priority queue to sort the
/// scheduled events. It first sorts the event such the event which has min time
/// will receive the most priority. If two events have equal execution time,
/// then their id number decides who executes first, by giving priority to the
/// one which is added sooner to the queue (less id number).
impl<T: Event> Ord for ScheduledEvent<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.time {
            x if x < other.time => Ordering::Greater,
            x if x > other.time => Ordering::Less,
            x if x == other.time => other.number.cmp(&self.number),
            _ => Ordering::Equal,
        }
    }
}

// impl<T: Event> Event for ScheduledEvent<T> {}

impl<T: Event> ScheduledEvent<T> {
    pub fn new(event: T, time: f64, number: i64) -> Self {
        Self {
            event,
            time,
            number,
        }
    }

    /// Returns the borrowed corresponding event.
    pub fn event(&self) -> T {
        self.event.clone()
    }

    /// Returns the execution time of the event.
    pub fn time(&self) -> f64 {
        self.time
    }

    /// Returns the event ID.
    pub fn number(&self) -> i64 {
        self.number
    }
}
