use crate::consensus::config::{ChainBasedConsensusConfig, ConsensusConfig};

#[derive(Default, Debug, Clone)]
pub struct NakamotoConsensusConfig {
    pub average_block_mining_interval: f64,
    pub confirmation_depth: i32,
    pub genesis_block_index: usize,
    pub difficulty: f64,
}

impl ConsensusConfig for NakamotoConsensusConfig {}

impl ChainBasedConsensusConfig for NakamotoConsensusConfig {
    fn get_average_block_mining_interval(&self) -> f64 {
        self.average_block_mining_interval
    }

    fn set_average_block_mining_interval(&mut self, interval: f64) {
        self.average_block_mining_interval = interval;
    }

    fn set_genesis_index(&mut self, index: usize) {
        self.genesis_block_index = index;
    }
}

impl NakamotoConsensusConfig {
    pub fn new(
        average_block_mining_interval: f64,
        confirmation_depth: i32,
        genesis_block_index: usize,
        difficulty: f64,
    ) -> Self {
        Self {
            average_block_mining_interval,
            confirmation_depth,
            genesis_block_index,
            difficulty,
        }
    }
}
