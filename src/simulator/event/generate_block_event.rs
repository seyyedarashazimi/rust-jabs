//! Generation event where a new packet is made and propagated to its neighbors.

use super::Event;
use crate::network::Network;
use crate::simulator::randomness_engine::RandomnessEngine;
use crate::simulator::Simulator;

#[derive(Debug)]
pub struct GenerateBlockWithoutTxEvent {
    pub node: usize,
}

impl Event for GenerateBlockWithoutTxEvent {
    fn execute(
        &self,
        network: &mut dyn Network,
        simulator: &mut Simulator,
        rand: &mut RandomnessEngine,
    ) {
        network.generate_new_block_without_tx(simulator, rand, &self);
    }
}

impl GenerateBlockWithoutTxEvent {
    pub(crate) fn new(node: usize) -> Self {
        Self { node }
    }
}
