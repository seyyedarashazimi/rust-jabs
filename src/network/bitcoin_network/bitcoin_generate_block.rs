use crate::consensus::algorithm::nakamoto_consensus::NakamotoConsensus;
use crate::ledger_data::bitcoin_block::BitcoinBlock;
use crate::ledger_data::block_factory::BlockFactory;
use crate::network::bitcoin_network::BitcoinNetwork;
use crate::network::message::DataType::IsBlock;
use crate::network::message::MessageType::DataMessage;
use crate::network::Network;
use crate::simulator::event::receive_event::ReceiveEvent;
use crate::simulator::randomness_engine::RandomnessEngine;
use crate::simulator::Simulator;

// Block generation methods and associated functions:
impl BitcoinNetwork {
    pub fn generate_new_block_and_receive_it(
        &mut self,
        simulator: &mut Simulator,
        rand: &mut RandomnessEngine,
        node: usize,
    ) {
        // here a new block will be added to the current slice.
        let new_block_index = self.resource.blocks.len();

        let bitcoin_block_without_tx = BitcoinNetwork::new_block_from_factory(
            &self.resource.blocks,
            simulator,
            rand,
            &self.ecs.consensus_algorithm[node],
            node,
            self.resource.config.difficulty,
        );
        self.resource.blocks.push(bitcoin_block_without_tx);

        let receive_at_this_node =
            ReceiveEvent::new(new_block_index, node, node, DataMessage(IsBlock));
        self.receive(simulator, &receive_at_this_node);
    }

    fn new_block_from_factory(
        blocks: &[BitcoinBlock],
        simulator: &Simulator,
        rand: &mut RandomnessEngine,
        consensus_algorithm: &NakamotoConsensus,
        node: usize,
        difficulty: f64,
    ) -> BitcoinBlock {
        let canonical_chain_head = consensus_algorithm.current_main_chain_head_index;
        let weight: f64 = rand.sample_exponential_distribution_mean_1();
        BlockFactory::sample_bitcoin_block(
            blocks,
            simulator,
            rand,
            Some(node),
            canonical_chain_head,
            difficulty,
            weight,
        )
    }
}
