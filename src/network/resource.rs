use crate::consensus::config::nakamoto_consensus_config::NakamotoConsensusConfig;
use crate::ledger_data::block::Block;

pub struct NetworkResource {
    pub blocks: Vec<Block>,
    pub config: NakamotoConsensusConfig,
    pub miners: Vec<usize>,
}
