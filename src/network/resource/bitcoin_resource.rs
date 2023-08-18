use crate::consensus::config::nakamoto_consensus_config::NakamotoConsensusConfig;
use crate::ledger_data::bitcoin_block::BitcoinBlock;

pub struct BitcoinResource {
    pub blocks: Vec<BitcoinBlock>,
    pub config: NakamotoConsensusConfig,
    pub miners: Vec<usize>,
}

impl BitcoinResource {
    pub fn new(
        average_num_of_blocks: usize,
        average_block_mining_interval: f64,
        confirmation_depth: i32,
        genesis_block_index: usize,
        difficulty: f64,
        num_of_miners: usize,
    ) -> Self {
        Self {
            blocks: Vec::with_capacity(average_num_of_blocks),
            config: NakamotoConsensusConfig::new(
                average_block_mining_interval,
                confirmation_depth,
                genesis_block_index,
                difficulty,
            ),
            miners: Vec::with_capacity(num_of_miners),
        }
    }

    pub fn num_of_miners(&self) -> usize {
        self.miners.len()
    }
}
