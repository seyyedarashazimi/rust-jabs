use crate::network::Network;
use crate::simulator::event::Event;
use crate::simulator::randomness_engine::RandomnessEngine;
use crate::simulator::Simulator;

#[derive(Debug, Clone)]
pub struct BlockMiningProcess {
    pub(crate) miner: usize,
}

impl Event for BlockMiningProcess {
    fn execute(
        &self,
        network: &mut dyn Network,
        simulator: &mut Simulator,
        rand: &mut RandomnessEngine,
    ) {
        network.block_mining(simulator, rand, &self);
    }
}

impl BlockMiningProcess {
    pub fn new(miner: usize) -> Self {
        Self { miner }
    }
}
