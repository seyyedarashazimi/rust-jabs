pub mod event;
pub mod rand;
pub mod scheduled_event;

pub use self::scheduled_event::*;
use crate::simulator::event::Event;
use specs::World;
use std::collections::BinaryHeap;

/// @author arash:
/// A discrete time event-based simulator with event queue
#[derive(Default)]
pub struct Simulator {
    /// The queue that contains all events which are going to be executed. This
    /// queue is a priority queue sorted by the time in which the event should
    /// be executed.
    ///
    /// The `event_queue` is of type [`BinaryHeap`], including events of type
    /// [`ScheduledEvent`].  
    event_queue: BinaryHeap<ScheduledEvent>,
    /// The simulation execution time of the most recent event
    pub simulation_time: f64,
    /// Number of events inserted in the event queue till now (whether simulated
    /// or not)
    pub inserted_events: i64,
}

impl Simulator {
    pub fn new() -> Self {
        Self {
            event_queue: BinaryHeap::new(),
            simulation_time: 0.0_f64,
            inserted_events: 0_i64,
        }
    }

    /// Executes the next event in the event queue.
    pub fn execute_next_event(&mut self, ecs: &mut World) {
        if let Some(mut current_scheduled_event) = self.event_queue.pop() {
            self.simulation_time = current_scheduled_event.time();
            // println!("simulation time: {}", self.simulation_time);
            current_scheduled_event.event.execute(ecs, self);
        }
    }

    /// Returns an immutable reference to the next event to be executed without
    /// executing the event.
    ///
    /// # Returns
    ///
    /// Immutable reference of the next event to be executed in the simulator
    pub fn peek_event(&self) -> Option<&Box<dyn Event>> {
        self.event_queue.peek().map(|se| &(se.event))
    }

    /// Check if more events exist in the event queue to be simulated.
    ///
    /// # Returns
    ///
    /// true if there is any event in the queue
    pub fn is_there_more_events(&self) -> bool {
        !self.event_queue.is_empty()
    }

    /// Inserts a new event in event queue. The event execution time will be the
    /// summation of current time and remaining time to execution.
    ///
    /// # Arguments
    ///
    /// * `event`: The event to be executed
    /// * `remaining_time_to_execution`: The time remaining to execution time of
    /// the event.
    pub fn put_event(&mut self, event: Box<dyn Event>, remaining_time_to_execution: f64) {
        let s_event = ScheduledEvent::new(
            event,
            self.simulation_time + remaining_time_to_execution,
            self.inserted_events,
        );
        // let s_event_key: ScheduledEventKey = (&s_event).into();
        self.event_queue.push(s_event);
        self.inserted_events += 1;
    }

    /// Removes an event already available in the event queue. It is specially
    /// useful for processes that are ongoing such as packet receiving process
    /// or block mining process.
    ///
    /// # Arguments
    ///
    /// * `scheduled_event`: The  scheduled event to be removed
    // pub fn remove_event(&mut self, scheduled_event: ScheduledEvent) {
    //     self.event_queue.remove(&scheduled_event);
    // }

    /// Returns the simulation time that the latest event has executed.
    ///
    /// # Returns
    ///
    /// Simulation time of the latest simulated event
    // pub fn get_simulation_time(&self) -> f64 {
    //     self.simulation_time
    // }

    /// Returns the inserted events.
    ///
    /// # Returns
    ///
    ///  inserted events
    // pub fn get_inserted_events(&self) -> i64 {
    //     self.inserted_events
    // }

    /// Clears the event queue from any more events. Restarts the current time
    /// of simulation to zero.
    pub fn reset(&mut self) {
        self.event_queue.clear();
        self.simulation_time = 0.0;
    }
}