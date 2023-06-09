//! Generation event where a new packet is made and propagated to its neighbors.

use super::{send_event::SendEvent, Event};
use crate::network::node::connection::node_is_connected;
use crate::network::packet::Packet;
use crate::network::{Network, LOGGER_MODE};
use crate::simulator::randomness_engine::RandomnessEngine;
use crate::simulator::Simulator;

#[derive(Debug, Clone)]
pub struct PacketGenerationEvent {
    pub packet_index: usize,
    pub node: usize,
}

impl Event for PacketGenerationEvent {
    fn execute(
        &mut self,
        ecs: &mut Network,
        simulator: &mut Simulator,
        _rand: &mut RandomnessEngine,
        packets: &[Packet],
    ) {
        let node = self.node;

        if !node_is_connected(ecs, node) {
            return;
        }

        self.initialize(ecs, simulator, packets);
    }
}

impl PacketGenerationEvent {
    pub fn new(packet_index: usize, node: usize) -> Self {
        Self { packet_index, node }
    }

    /// Create a `SendEvent` for the node as well as archiving it for itself.
    ///
    /// # Arguments
    ///
    /// * `ecs`: Mutable reference to [`Network`];
    /// * `simulator`: Mutable reference to [`Simulator`];
    /// * `packets`: Immutable reference to `packets`.
    ///
    fn initialize(&self, ecs: &mut Network, simulator: &mut Simulator, packets: &[Packet]) {
        let node = self.node;
        let index = self.packet_index;

        if LOGGER_MODE {
            println!("[GENERATE] A new packet generated at node:{:?}.", node);
        }

        if let Some(history_packets) = ecs.history.get_mut(node) {
            history_packets.received.insert(packets[index].clone());
        }

        simulator.put_event(Box::new(SendEvent::new(index, node, node)), 0.0);
    }
}
