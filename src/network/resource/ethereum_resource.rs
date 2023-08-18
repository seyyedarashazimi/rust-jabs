use crate::consensus::config::nakamoto_consensus_config::NakamotoConsensusConfig;
use crate::ledger_data::bitcoin_block::BitcoinBlock;

pub struct EthereumResource {
    pub blocks: Vec<BitcoinBlock>,
    pub config: NakamotoConsensusConfig,
    pub miners: Vec<usize>,
}
