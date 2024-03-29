//! Scheduled event to be executed in simulator.

use crate::simulator::event::Event;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

/// The scheduled event struct, including the event, it's time , and the id
/// number
#[derive(Debug)]
pub struct ScheduledEvent {
    /// The event which must implement [`Event`] trait
    pub event: Box<dyn Event>,
    /// Simulation execution time of the event
    time: f64,
    /// Event ID (insertion number in event queue)
    number: i64,
}

impl Hash for ScheduledEvent {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.number.hash(state);
    }
}

impl PartialEq for ScheduledEvent {
    fn eq(&self, other: &Self) -> bool {
        self.number.eq(&other.number) && self.time.eq(&other.time)
    }
}

impl Eq for ScheduledEvent {}

impl PartialOrd for ScheduledEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// This trait implementation is used by the priority queue to sort the
/// scheduled events. It first sorts the event such the event which has min time
/// will receive the most priority. If two events have equal execution time,
/// then their id number decides who executes first, by giving priority to the
/// one which is added sooner to the queue (less id number).
impl Ord for ScheduledEvent {
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

impl ScheduledEvent {
    pub fn new(event: Box<dyn Event>, time: f64, number: i64) -> Self {
        Self {
            event,
            time,
            number,
        }
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
