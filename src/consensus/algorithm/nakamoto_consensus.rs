use crate::consensus::blockchain::local_block_tree::LocalBlockTree;
use crate::consensus::config::nakamoto_consensus_config::NakamotoConsensusConfig;
use crate::ledger_data::block::Block;
use crate::simulator::event::block_confirmation_event::BlockConfirmationEvent;
use crate::simulator::Simulator;
use std::collections::HashSet;

#[derive(Default, Debug, Clone)]
pub struct NakamotoConsensus {
    pub longest_chain_len: i32,
    pub current_main_chain_head_index: usize,
    pub confirmed_blocks: HashSet<usize>,
    pub node_index: usize,
}

impl NakamotoConsensus {
    pub fn new(config: &NakamotoConsensusConfig) -> Self {
        Self {
            longest_chain_len: 0,
            current_main_chain_head_index: config.genesis_block_index,
            confirmed_blocks: HashSet::new(),
            node_index: 0,
        }
    }

    pub fn initial_configuration(&mut self, config: &NakamotoConsensusConfig, node_index: usize) {
        self.longest_chain_len = 0;
        self.current_main_chain_head_index = config.genesis_block_index;
        self.confirmed_blocks = HashSet::new();
        self.node_index = node_index;
    }

    /// When a new block is received, this function should be called. The
    /// consensus algorithm should take actions required accordingly to update
    /// the state.
    ///
    /// # Arguments
    ///
    /// * `block_index`: index of the recently received block
    /// * `blocks`: immutable reference to `Block`
    /// * `config`: consensus algorithm config
    /// * `local_block_trees`: immutable reference to `LocalBlockTree`
    ///
    pub fn new_incoming_block(
        &mut self,
        block_index: usize,
        blocks: &[Block],
        config: &NakamotoConsensusConfig,
        local_block_trees: &LocalBlockTree,
        simulator: &mut Simulator,
    ) {
        let block_height = blocks[block_index].height;
        if block_height > self.longest_chain_len {
            self.longest_chain_len = block_height;
            self.current_main_chain_head_index = block_index;
            self.chain_update(blocks, config, local_block_trees, simulator);
        }
    }

    fn chain_update(
        &mut self,
        blocks: &[Block],
        config: &NakamotoConsensusConfig,
        local_block_trees: &LocalBlockTree,
        simulator: &mut Simulator,
    ) {
        let current_main_head_height = blocks[self.current_main_chain_head_index].height;
        let confirmation_depth = config.confirmation_depth;
        if current_main_head_height > confirmation_depth {
            let height_of_confirmed_blocks = current_main_head_height - confirmation_depth;
            if let Some(highest_confirmed_block_index) = local_block_trees
                .get_single_ancestor_of_height(
                    self.current_main_chain_head_index,
                    height_of_confirmed_blocks,
                    blocks,
                )
            {
                self.confirmed_blocks = local_block_trees
                    .get_all_single_ancestors(highest_confirmed_block_index, blocks);
                let current_time = simulator.simulation_time;
                let block_confirmation_event = Box::new(BlockConfirmationEvent::new(
                    highest_confirmed_block_index,
                    self.node_index,
                    current_time,
                ));
                simulator.put_event(block_confirmation_event, 0.0);
            }
        }
    }
}
