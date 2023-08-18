//! Propagation event where received packet  propagates further to the neighbors.

use super::Event;
use crate::network::message::MessageType;
use crate::network::Network;
use crate::simulator::randomness_engine::RandomnessEngine;
use crate::simulator::Simulator;

#[derive(Debug, Clone)]
pub struct SendEvent {
    pub block_index: usize,
    pub from: usize,
    pub node: usize,
    pub msg_type: MessageType,
}

impl Event for SendEvent {
    fn execute(
        &self,
        network: &mut dyn Network,
        simulator: &mut Simulator,
        rand: &mut RandomnessEngine,
    ) {
        network.send(simulator, rand, &self);
    }
}

impl SendEvent {
    pub fn new(block_index: usize, from: usize, node: usize, msg_type: MessageType) -> Self {
        Self {
            block_index,
            from,
            node,
            msg_type,
        }
    }
}
