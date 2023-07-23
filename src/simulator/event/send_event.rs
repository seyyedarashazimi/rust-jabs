//! Propagation event where received packet  propagates further to the neighbors.

use super::{Event, ReceiveEvent};
use crate::ledger_data::block::Block;
use crate::network::message::MessageType;
use crate::network::message::MessageType::{DataMessage, InvMessage, RequestDataMessage};
use crate::network::node::connection::node_is_connected;
use crate::network::node::link::remaining_time_to_load;
use crate::network::resource::NetworkResource;
use crate::network::stats::eighty_six_countries::get_latency;
use crate::network::{Network, FULL_LOGGER_MODE, LOGGER_MODE};
use crate::simulator::randomness_engine::RandomnessEngine;
use crate::simulator::Simulator;

#[derive(Debug, Clone)]
pub struct SendEvent {
    pub block_index: usize,
    pub from: usize,
    pub node: usize,
    pub msg_type: MessageType,
}

impl Event for SendEvent {
    fn execute(
        &self,
        ecs: &mut Network,
        simulator: &mut Simulator,
        rand: &mut RandomnessEngine,
        resource: &mut NetworkResource,
    ) {
        let node = self.node;

        if !node_is_connected(ecs, node) {
            return;
        }

        self.send(ecs, simulator, rand, &resource.blocks);
    }
}

impl SendEvent {
    pub fn new(block_index: usize, from: usize, node: usize, msg_type: MessageType) -> Self {
        Self {
            block_index,
            from,
            node,
            msg_type,
        }
    }

    fn send(
        &self,
        ecs: &mut Network,
        simulator: &mut Simulator,
        rand: &mut RandomnessEngine,
        blocks: &[Block],
    ) {
        match &self.msg_type {
            InvMessage(_) => self.send_inv_to_neighbors(ecs, simulator, rand, blocks),
            DataMessage(_) | RequestDataMessage(_) => {
                self.simulate_upload(ecs, simulator, rand, blocks, self.from)
            }
            _ => (),
        }
    }

    /// For each neighbor connected to a node, create a ['ReceiveEvent'] and
    /// push into the simulator. It will avoid forwarding the packet to the node
    /// `self.from` which had sent the packet to this node.
    ///
    /// # Arguments
    ///
    /// * `ecs`: Mutable reference to [`Network`];
    /// * `simulator`: Mutable reference to [`Simulator`];
    /// * `packets`: Immutable reference to `packets`.
    ///
    fn send_inv_to_neighbors(
        &self,
        ecs: &mut Network,
        simulator: &mut Simulator,
        rand: &mut RandomnessEngine,
        blocks: &[Block],
    ) {
        if let Some(neighbors) = ecs.neighbors.get(self.node) {
            // remove the sender of the packet from the set of neighbors:
            let filtered_neighbors: Vec<usize> = neighbors
                .list
                .iter()
                .filter(|&neighbor| *neighbor != self.from)
                .cloned()
                .collect();

            for neighbor in filtered_neighbors {
                self.simulate_upload(ecs, simulator, rand, blocks, neighbor);
            }
        }
    }

    fn simulate_upload(
        &self,
        ecs: &mut Network,
        simulator: &mut Simulator,
        rand: &mut RandomnessEngine,
        blocks: &[Block],
        to: usize,
    ) {
        let node = self.node;
        let index = self.block_index;

        let forward_event = Box::new(ReceiveEvent::new(index, node, to, self.msg_type.clone()));
        if let Some(uplink) = ecs.uplink.get_mut(node) {
            let size = self.msg_type.get_size(index, blocks);
            let upload_delay = remaining_time_to_load(&mut uplink.link, simulator, size);
            let delivery_delay = get_latency(ecs.country[node], ecs.country[to], rand);
            simulator.put_event(forward_event, upload_delay + delivery_delay);
            if LOGGER_MODE && FULL_LOGGER_MODE {
                println!(
                    "[Send] Forwarding message from node:{:?} to node:{:?}",
                    node, to
                );
            }
        }
    }
}
