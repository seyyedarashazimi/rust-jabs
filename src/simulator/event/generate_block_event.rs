//! Generation event where a new packet is made and propagated to its neighbors.

use super::Event;
use crate::consensus::algorithm::nakamoto_consensus::NakamotoConsensus;
use crate::consensus::config::nakamoto_consensus_config::NakamotoConsensusConfig;
use crate::ledger_data::block::Block;
use crate::ledger_data::block_factory::BlockFactory;
use crate::network::message::DataType::IsBlock;
use crate::network::message::MessageType::DataMessage;
use crate::network::node::connection::node_is_connected;
use crate::network::resource::NetworkResource;
use crate::network::{Network, LOGGER_MODE};
use crate::simulator::event::receive_event::ReceiveEvent;
use crate::simulator::randomness_engine::RandomnessEngine;
use crate::simulator::Simulator;

#[derive(Debug, Clone)]
pub struct GenerateBlockEvent {
    pub node: usize,
}

impl Event for GenerateBlockEvent {
    fn execute(
        &mut self,
        ecs: &mut Network,
        simulator: &mut Simulator,
        rand: &mut RandomnessEngine,
        resource: &mut NetworkResource,
    ) {
        let node = self.node;

        if !node_is_connected(ecs, node) {
            return;
        }

        self.initialize(ecs, simulator, rand, &mut resource.blocks, &resource.config)
    }
}

impl GenerateBlockEvent {
    pub fn new(node: usize) -> Self {
        Self { node }
    }

    /// Create a `SendEvent` for the node as well as archiving it for itself.
    ///
    /// # Arguments
    ///
    /// * `ecs`: Mutable reference to [`Network`];
    /// * `simulator`: Mutable reference to [`Simulator`];
    /// * `packets`: Immutable reference to `packets`.
    ///
    fn initialize(
        &self,
        ecs: &mut Network,
        simulator: &mut Simulator,
        rand: &mut RandomnessEngine,
        blocks: &mut Vec<Block>,
        consensus_config: &NakamotoConsensusConfig,
    ) {
        let node = self.node;

        if LOGGER_MODE {
            println!("[GENERATE] A new packet generated at node:{:?}.", node);
        }

        let new_block_index = GenerateBlockEvent::generate_new_block_event(
            blocks,
            simulator,
            rand,
            &ecs.consensus_algorithm[node],
            node,
        );

        let receive_at_this_node =
            ReceiveEvent::new(new_block_index, node, node, DataMessage(IsBlock));
        receive_at_this_node.receive(ecs, simulator, blocks, consensus_config);
    }

    pub fn generate_new_block_event(
        blocks: &mut Vec<Block>,
        simulator: &Simulator,
        rand: &mut RandomnessEngine,
        consensus_algorithm: &NakamotoConsensus,
        node: usize,
    ) -> usize {
        // here a new block will be added to the current slice.
        let block_index = blocks.len();

        let bitcoin_block_without_tx = GenerateBlockEvent::generate_new_block(
            blocks,
            simulator,
            rand,
            consensus_algorithm,
            node,
        );
        blocks.push(bitcoin_block_without_tx);

        block_index
    }

    fn generate_new_block(
        blocks: &mut [Block],
        simulator: &Simulator,
        rand: &mut RandomnessEngine,
        consensus_algorithm: &NakamotoConsensus,
        node: usize,
    ) -> Block {
        let canonical_chain_head = consensus_algorithm.current_main_chain_head_index;
        let weight: f64 = rand.sample_exponential_distribution(1.0);
        BlockFactory::sample_bitcoin_block(
            blocks,
            simulator,
            rand,
            Some(node),
            canonical_chain_head,
            weight,
        )
    }
}
