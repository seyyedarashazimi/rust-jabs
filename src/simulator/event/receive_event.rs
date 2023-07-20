use crate::consensus::config::nakamoto_consensus_config::NakamotoConsensusConfig;
use crate::ledger_data::block::Block;
use crate::network::message::DataType::IsBlock;
use crate::network::message::MessageType;
use crate::network::message::MessageType::{DataMessage, InvMessage, RequestDataMessage};
use crate::network::node::connection::node_is_connected;
use crate::network::node::link::remaining_time_to_load;
use crate::network::resource::NetworkResource;
use crate::network::{Network, FULL_LOGGER_MODE, LOGGER_MODE};
use crate::simulator::event::send_event::SendEvent;
use crate::simulator::event::Event;
use crate::simulator::randomness_engine::RandomnessEngine;
use crate::simulator::Simulator;

#[derive(Debug)]
pub struct ReceiveEvent {
    pub block_index: usize,
    pub from: usize,
    pub node: usize,
    pub msg_type: MessageType,
}

impl Event for ReceiveEvent {
    fn execute(
        &mut self,
        ecs: &mut Network,
        simulator: &mut Simulator,
        _: &mut RandomnessEngine,
        resource: &mut NetworkResource,
    ) {
        let node = self.node;
        if !node_is_connected(ecs, node) {
            return;
        }

        if LOGGER_MODE && FULL_LOGGER_MODE {
            println!(
                "[RECEIVED] packet received at node:{:?} from node:{:?}.",
                node, self.from
            );
        }

        // self.receive_packet(ecs, simulator, packets);
        self.receive(ecs, simulator, &resource.blocks, &resource.config);
    }
}

impl ReceiveEvent {
    pub fn new(block_index: usize, from: usize, node: usize, msg_type: MessageType) -> Self {
        Self {
            block_index,
            from,
            node,
            msg_type,
        }
    }

    pub fn receive(
        &self,
        ecs: &mut Network,
        simulator: &mut Simulator,
        blocks: &[Block],
        consensus_config: &NakamotoConsensusConfig,
    ) {
        match &self.msg_type {
            DataMessage(_) => self.process_data_message(ecs, simulator, blocks, consensus_config),
            InvMessage(_) => self.process_inv_message(ecs, simulator, blocks),
            RequestDataMessage(_) => self.process_request_data_message(ecs, simulator, blocks),
            _ => (),
        }
    }

    fn process_data_message(
        &self,
        ecs: &mut Network,
        simulator: &mut Simulator,
        blocks: &[Block],
        consensus_config: &NakamotoConsensusConfig,
    ) {
        let node = self.node;
        let block_index = self.block_index;

        if !ecs.local_block_tree[node].contains(block_index) {
            ecs.local_block_tree[node].add(block_index, blocks);
            ecs.already_seen_blocks[node].list.insert(block_index, true);
            if ecs.local_block_tree[node]
                .local_block_dag
                .get(&block_index)
                .unwrap()
                .is_connected_to_genesis
            {
                ecs.consensus_algorithm[node].new_incoming_block(
                    block_index,
                    blocks,
                    consensus_config,
                    &ecs.local_block_tree[node],
                );
                self.simulate_download(ecs, simulator, blocks, InvMessage(IsBlock));
            }
        }
    }

    fn process_inv_message(&self, ecs: &mut Network, simulator: &mut Simulator, blocks: &[Block]) {
        let seen_blocks = &mut ecs.already_seen_blocks[self.node].list;
        if !seen_blocks.contains_key(&self.block_index) {
            seen_blocks.insert(self.block_index, false);
            self.simulate_download(ecs, simulator, blocks, RequestDataMessage(IsBlock));
        }
    }

    fn process_request_data_message(
        &self,
        ecs: &mut Network,
        simulator: &mut Simulator,
        blocks: &[Block],
    ) {
        let seen_blocks = &mut ecs.already_seen_blocks[self.node].list;
        if *seen_blocks.get(&self.block_index).unwrap_or(&false) {
            self.simulate_download(ecs, simulator, blocks, DataMessage(IsBlock));
        }
    }

    fn simulate_download(
        &self,
        ecs: &mut Network,
        simulator: &mut Simulator,
        blocks: &[Block],
        msg_type: MessageType,
    ) {
        // simulate download of the received data for simulation time and create a send event.
        if let Some(downlink) = ecs.downlink.get_mut(self.node) {
            let size = msg_type.get_size(self.block_index, blocks);
            let download_delay = remaining_time_to_load(&mut downlink.link, simulator, size);

            let propagate_event = Box::new(SendEvent::new(
                self.block_index,
                self.from,
                self.node,
                msg_type,
            ));
            simulator.put_event(propagate_event, download_delay);
        }
    }
}
