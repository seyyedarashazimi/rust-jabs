//! Propagation event where received packet  propagates further to the neighbors.

use super::{Event, ReceiveEvent};
use crate::network::packet::Packet;
use crate::network::stats::eighty_six_countries::get_latency;
use crate::network::{node_is_connected, remaining_time_to_load, Network, LOGGER_MODE};
use crate::simulator::randomness_engine::RandomnessEngine;
use crate::simulator::Simulator;

#[derive(Debug, Clone)]
pub struct SendEvent {
    pub packet_index: usize,
    pub from: usize,
    pub node: usize,
}

impl Event for SendEvent {
    fn execute(
        &mut self,
        ecs: &mut Network,
        simulator: &mut Simulator,
        rand: &mut RandomnessEngine,
        packets: &[Packet],
    ) {
        let node = self.node;

        if !node_is_connected(ecs, node) {
            return;
        }

        self.send_to_neighbors(ecs, simulator, rand, packets);
    }
}

impl SendEvent {
    pub fn new(packet_index: usize, from: usize, node: usize) -> Self {
        Self {
            packet_index,
            from,
            node,
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
    pub fn send_to_neighbors(
        &self,
        ecs: &mut Network,
        simulator: &mut Simulator,
        rand: &mut RandomnessEngine,
        packets: &[Packet],
    ) {
        let node = self.node;
        let index = self.packet_index;
        if let Some(neighbors) = ecs.neighbors.get(node) {
            // remove the sender of the packet from the set of neighbors:
            let filtered_neighbors = neighbors
                .list
                .iter()
                .filter(|&neighbor| *neighbor != self.from);

            for neighbor in filtered_neighbors {
                let forward_event = Box::new(ReceiveEvent::new(index, node, *neighbor));
                if let Some(uplink) = ecs.uplink.get_mut(node) {
                    let upload_delay =
                        remaining_time_to_load(&mut uplink.link, simulator, packets[index].size);
                    let delivery_delay =
                        get_latency(ecs.country[node], ecs.country[*neighbor], rand);
                    simulator.put_event(forward_event, upload_delay + delivery_delay);
                    if LOGGER_MODE {
                        println!(
                            "[Send] Forwarding message from node:{:?} to node:{:?}",
                            node, neighbor
                        );
                    }
                }
            }
        }
    }
}
