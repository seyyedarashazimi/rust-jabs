pub mod nakamoto_consensus_config;

pub trait ConsensusConfig {}

pub trait ChainBasedConsensusConfig: ConsensusConfig {
    fn get_average_block_mining_interval(&self) -> f64;
    fn get_genesis_index(&self) -> usize {
        0_usize
    }

    fn set_average_block_mining_interval(&mut self, interval: f64);
    fn set_genesis_index(&mut self, index: usize);
}
