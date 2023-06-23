//! Propagation event where received packet  propagates further to the neighbors.
use super::Event;
use crate::network::node::HistoryPackets;
use crate::network::packet::Packet;
use crate::network::{node_is_connected, send_to_neighbors, LOGGER_MODE};
use crate::simulator::Simulator;
use specs::prelude::*;

#[derive(Debug, Clone)]
pub struct PropagateEvent {
    pub packet: Packet,
    pub receiving_node: Entity,
}

impl Event for PropagateEvent {
    fn execute(&mut self, ecs: &mut World, simulator: &mut Simulator) {
        self.propagate_packet(ecs, simulator);
    }
}

impl PropagateEvent {
    fn propagate_packet(&mut self, ecs: &mut World, simulator: &mut Simulator) {
        let node = self.receiving_node;
        if !node_is_connected(ecs, node) {
            return;
        }

        let packet = &self.packet;
        // determine whether the received packet is new and put it inside the
        // history packets.
        let mut packet_is_new = false;
        if let Some(history_packets) = ecs.write_storage::<HistoryPackets>().get_mut(node) {
            if !history_packets.received.contains(packet) {
                packet_is_new = history_packets.received.insert(packet.clone());
            }
        }
        if packet_is_new {
            // println!("[NEW] new packet received at node {:?}", node);
            if packet.to == node {
                if LOGGER_MODE {
                    println!("[RECEIVED] packet safely received at {:?}.", node);
                    println!("The received packet message is: {}", packet.msg);
                }
            }
            // create next events if packet is new and receiver is not this node.
            // new events are collected into next_events field.
            else {
                send_to_neighbors(ecs, simulator, node, packet);
            }
        }
    }
}
