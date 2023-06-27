//! Event trait including `execute()` method.

pub mod packet_generation_event;
mod receive_event;
mod send_event;

use self::receive_event::ReceiveEvent;
use crate::network::packet::Packet;
use crate::network::Network;
use crate::simulator::Simulator;
use std::fmt::Debug;

/// To enforce `Box<dyn Event>` to have `execute()` method to be called
///  when popped from [`Simulator`].
pub trait Event: Debug {
    fn execute(&mut self, ecs: &mut Network, sim: &mut Simulator, packets: &Vec<Packet>);
}
