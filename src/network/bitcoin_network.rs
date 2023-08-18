mod bitcoin_block_mining;
mod bitcoin_generate_block;
mod bitcoin_receive;
pub(crate) mod bitcoin_scenarios_preparation;
mod bitcoin_send;

use crate::ledger_data::block::Block;
use crate::log::blockchain_reorg_logger::BlockchainReorgLogger;
use crate::log::NetworkLogHandler;
use crate::network::ecs::bitcoin_ecs::BitcoinECS;
use crate::network::message::MessageType::{DataMessage, InvMessage, RequestDataMessage};
use crate::network::node::connection::node_is_connected;
use crate::network::resource::bitcoin_resource::BitcoinResource;
use crate::network::Network;
use crate::simulator::event::block_mining_process::BlockMiningProcess;
use crate::simulator::event::generate_block_event::GenerateBlockWithoutTxEvent;
use crate::simulator::event::receive_event::ReceiveEvent;
use crate::simulator::event::send_event::SendEvent;
use crate::simulator::randomness_engine::RandomnessEngine;
use crate::simulator::Simulator;

pub struct BitcoinNetwork {
    pub ecs: BitcoinECS,
    pub resource: BitcoinResource,
}

impl Network for BitcoinNetwork {
    fn generate_new_block_without_tx(
        &mut self,
        simulator: &mut Simulator,
        rand: &mut RandomnessEngine,
        generate_event: &GenerateBlockWithoutTxEvent,
    ) {
        let node = generate_event.node;
        if !node_is_connected(&self.ecs.is_connected, node) {
            return;
        }

        self.generate_new_block_and_receive_it(simulator, rand, node);
    }

    fn receive(&mut self, simulator: &mut Simulator, receive_event: &ReceiveEvent) {
        let node = receive_event.node;
        if !node_is_connected(&self.ecs.is_connected, node) {
            return;
        }

        match receive_event.msg_type {
            DataMessage(_) => self.process_data_message(simulator, receive_event),
            InvMessage(_) => self.process_inv_message(simulator, receive_event),
            RequestDataMessage(_) => self.process_request_data_message(simulator, receive_event),
            _ => (),
        }
    }

    fn send(
        &mut self,
        simulator: &mut Simulator,
        rand: &mut RandomnessEngine,
        send_event: &SendEvent,
    ) {
        let node = send_event.node;
        if !node_is_connected(&self.ecs.is_connected, node) {
            return;
        }

        match &send_event.msg_type {
            InvMessage(_) => self.send_inv_to_neighbors(simulator, rand, send_event),
            DataMessage(_) | RequestDataMessage(_) => {
                self.simulate_upload(simulator, rand, send_event.from, send_event);
            }
            _ => (),
        }
    }

    fn block_mining(
        &mut self,
        simulator: &mut Simulator,
        rand: &mut RandomnessEngine,
        mining_event: &BlockMiningProcess,
    ) {
        let miner = mining_event.miner;
        if !node_is_connected(&self.ecs.is_connected, miner) {
            return;
        }

        self.mine_new_block(simulator, rand, miner);
    }
}

impl NetworkLogHandler for BitcoinNetwork {
    fn get_block_creation_time(&self, block_index: usize) -> f64 {
        self.resource.blocks[block_index].get_creation_time()
    }

    fn get_block_creator(&self, block_index: usize) -> Option<usize> {
        self.resource.blocks[block_index].get_creator()
    }

    fn get_block_height(&self, block_index: usize) -> i32 {
        self.resource.blocks[block_index].get_height()
    }

    fn get_block_size(&self, block_index: usize) -> u64 {
        self.resource.blocks[block_index].get_size()
    }

    fn get_block_parents(&self, block_index: usize) -> &Vec<usize> {
        self.resource.blocks[block_index].get_parents()
    }

    fn get_num_of_nodes(&self) -> usize {
        self.ecs.num_of_nodes
    }

    // fn add_to_local_block_tree(&self, local_block_tree: &mut LocalBlockTree, block_index: usize) {
    //     local_block_tree.add(block_index, &self.resource.blocks);
    // }

    fn block_reorg_before(
        &self,
        reorg_logger: &mut BlockchainReorgLogger,
        block_index: &usize,
        node: &usize,
    ) {
        reorg_logger
            .network_view_block_tree
            .add(*block_index, &self.resource.blocks);
        reorg_logger.previous_head_chain_index =
            Some(self.ecs.consensus_algorithm[*node].current_main_chain_head_index);
        reorg_logger.current_node_index = Some(*node);
        reorg_logger.new_block_received = true;
    }

    fn block_reorg_after(&self, reorg_logger: &mut BlockchainReorgLogger) -> bool {
        if reorg_logger.new_block_received {
            reorg_logger.new_block_received = false;
            if let Some(node_index) = reorg_logger.current_node_index {
                let current_head_chain_index =
                    self.ecs.consensus_algorithm[node_index].current_main_chain_head_index;
                if let Some(previous_head_index) = reorg_logger.previous_head_chain_index {
                    let ancestor_index = reorg_logger
                        .network_view_block_tree
                        .get_single_ancestor_of_height(
                            current_head_chain_index,
                            self.resource.blocks[previous_head_index].height,
                            &self.resource.blocks,
                        );
                    return ancestor_index != reorg_logger.previous_head_chain_index;
                }
            }
        }
        false
    }

    fn block_reorg_output_length(
        &self,
        reorg_logger: &BlockchainReorgLogger,
        previous_head: usize,
        node_index: &usize,
    ) -> i32 {
        let node_chain_head =
            self.ecs.consensus_algorithm[*node_index].current_main_chain_head_index;
        let common_ancestor = reorg_logger.network_view_block_tree.get_common_ancestor(
            node_chain_head,
            previous_head,
            &self.resource.blocks,
        );
        let reorg_length: i32 = self.resource.blocks[node_chain_head].height
            - self.resource.blocks[common_ancestor].height;

        reorg_length
    }
}

impl BitcoinNetwork {
    pub fn new_with_size(
        num_of_nodes: usize,
        average_num_of_blocks: usize,
        average_block_mining_interval: f64,
        confirmation_depth: i32,
        genesis_block_index: usize,
        difficulty: f64,
        num_of_miners: usize,
    ) -> Self {
        Self {
            ecs: BitcoinECS::create_with_size(num_of_nodes),
            resource: BitcoinResource::new(
                average_num_of_blocks,
                average_block_mining_interval,
                confirmation_depth,
                genesis_block_index,
                difficulty,
                num_of_miners,
            ),
        }
    }
}
