use crate::consensus::blockchain::local_block_tree::LocalBlockTree;
use crate::consensus::config::ConsensusConfig;
use crate::ledger_data::block::Block;
use crate::simulator::Simulator;
use std::collections::HashSet;

pub mod nakamoto_consensus;

pub trait DAGBasedConsensus {
    type B: Block;
    type G: ConsensusConfig;

    fn new(config: &Self::G) -> Self;
    fn initial_configuration(&mut self, config: &Self::G, node_index: usize);
    fn new_incoming_block(
        &mut self,
        block_index: usize,
        blocks: &[Self::B],
        config: &Self::G,
        local_block_trees: &LocalBlockTree,
        simulator: &mut Simulator,
    );

    fn get_mut_confirmed_blocks(&mut self) -> &mut HashSet<usize>;
    fn get_node_index(&self) -> usize;
    fn set_node_index(&mut self, node_index: usize);
}

pub trait ChainBasedConsensus: DAGBasedConsensus {
    fn update_chain(
        &mut self,
        blocks: &[Self::B],
        config: &Self::G,
        local_block_trees: &LocalBlockTree,
        simulator: &mut Simulator,
    );

    fn get_longest_chain_len(&self) -> i32;
    fn get_canonical_chain_head_index(&self) -> usize;

    fn set_longest_chain_len(&mut self, len: i32);
    fn set_canonical_chain_head_index(&mut self, index: usize);
}
