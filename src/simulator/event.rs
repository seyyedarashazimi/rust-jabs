//! Event trait including `execute()` method.

mod packet_delivery_event;
pub mod packet_generation_event;
pub mod propagate_event;

use crate::simulator::Simulator;
use specs::World;
use std::fmt::Debug;

/// To enforce `Box<dyn Event>` to have `execute()` method to be called
///  when popped from [`Simulator`].
pub trait Event: Debug {
    fn execute(&mut self, ecs: &mut World, sim: &mut Simulator);
}
