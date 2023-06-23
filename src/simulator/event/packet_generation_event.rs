//! Generation event where a new packet is made and propagated to its neighbors.

use super::Event;
use crate::network::packet::Packet;
use crate::network::{node_is_connected, send_to_neighbors, LOGGER_MODE};
use crate::simulator::Simulator;
use specs::prelude::*;

#[derive(Debug, Clone)]
pub struct PacketGenerationEvent {
    pub packet: Packet,
}

impl Event for PacketGenerationEvent {
    fn execute(&mut self, ecs: &mut World, simulator: &mut Simulator) {
        let node = self.packet.from;
        if !node_is_connected(ecs, node) {
            return;
        }

        if LOGGER_MODE {
            println!("[GENERATE] A new packet generated at node {:?}", node);
        }

        send_to_neighbors(ecs, simulator, node, &self.packet);
    }
}

impl PacketGenerationEvent {
    pub fn new(packet: Packet) -> Self {
        Self { packet }
    }
}
