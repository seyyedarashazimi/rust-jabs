use crate::consensus::algorithm::DAGBasedConsensus;
use crate::ledger_data::single_parent::SingleParent;
use crate::network::bitcoin_network::BitcoinNetwork;
use crate::network::message::DataType::IsBlock;
use crate::network::message::MessageType;
use crate::network::message::MessageType::{DataMessage, InvMessage, RequestDataMessage};
use crate::network::node::link::remaining_time_to_load;
use crate::simulator::event::receive_event::ReceiveEvent;
use crate::simulator::event::send_event::SendEvent;
use crate::simulator::Simulator;
use std::collections::hash_map::Entry::Vacant;

// Receive methods and associated functions:
impl BitcoinNetwork {
    pub(crate) fn process_data_message(
        &mut self,
        simulator: &mut Simulator,
        receive_event: &ReceiveEvent,
    ) {
        let node = receive_event.node;
        let block_index = receive_event.block_index;

        if !self.ecs.local_block_tree[node].contains(block_index) {
            self.ecs.local_block_tree[node].add(block_index, &self.resource.blocks);
            self.ecs.already_seen_blocks[node]
                .0
                .insert(block_index, true);
            if self.ecs.local_block_tree[node]
                .local_block_dag
                .get(&block_index)
                .unwrap()
                .is_connected_to_genesis
            {
                self.process_new_block(block_index, simulator, receive_event);
                if let Some(new_blocks) =
                    self.ecs.local_block_tree[node].get_all_successors(block_index)
                {
                    for new_block in new_blocks {
                        self.process_new_block(new_block, simulator, receive_event);
                    }
                }
            } else if let Some(parent) = self.resource.blocks[block_index].get_single_parent() {
                self.simulate_download(
                    parent,
                    simulator,
                    RequestDataMessage(IsBlock),
                    receive_event,
                );
            }
        }
    }

    pub(crate) fn process_inv_message(
        &mut self,
        simulator: &mut Simulator,
        receive_event: &ReceiveEvent,
    ) {
        let seen_blocks = &mut self.ecs.already_seen_blocks[receive_event.node].0;
        if let Vacant(e) = seen_blocks.entry(receive_event.block_index) {
            e.insert(false);
            self.simulate_download(
                receive_event.block_index,
                simulator,
                RequestDataMessage(IsBlock),
                receive_event,
            );
        }
    }

    pub(crate) fn process_request_data_message(
        &mut self,
        simulator: &mut Simulator,
        receive_event: &ReceiveEvent,
    ) {
        let seen_blocks = &mut self.ecs.already_seen_blocks[receive_event.node].0;
        if *seen_blocks
            .get(&receive_event.block_index)
            .unwrap_or(&false)
        {
            self.simulate_download(
                receive_event.block_index,
                simulator,
                DataMessage(IsBlock),
                receive_event,
            );
        }
    }

    fn process_new_block(
        &mut self,
        block_index: usize,
        simulator: &mut Simulator,
        receive_event: &ReceiveEvent,
    ) {
        self.ecs.consensus_algorithm[receive_event.node].new_incoming_block(
            block_index,
            &self.resource.blocks,
            &self.resource.config,
            &self.ecs.local_block_tree[receive_event.node],
            simulator,
        );
        self.simulate_download(block_index, simulator, InvMessage(IsBlock), receive_event);
    }

    fn simulate_download(
        &mut self,
        propagate_block: usize,
        simulator: &mut Simulator,
        propagate_msg_type: MessageType,
        receive_event: &ReceiveEvent,
    ) {
        // simulate download of the received data for simulation time and create a send event.
        if let Some(downlink) = self.ecs.downlink.get_mut(receive_event.node) {
            let size = receive_event
                .msg_type
                .get_size(receive_event.block_index, &self.resource.blocks);
            let download_delay = remaining_time_to_load(&mut downlink.link, simulator, size);

            let propagate_event = Box::new(SendEvent::new(
                propagate_block,
                receive_event.from,
                receive_event.node,
                propagate_msg_type,
            ));
            simulator.put_event(propagate_event, download_delay);
        }
    }
}
