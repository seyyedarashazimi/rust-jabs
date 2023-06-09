use crate::network::node::connection::node_is_connected;
use crate::network::node::link::remaining_time_to_load;
use crate::network::packet::Packet;
use crate::network::{Network, LOGGER_MODE};
use crate::simulator::event::send_event::SendEvent;
use crate::simulator::event::Event;
use crate::simulator::randomness_engine::RandomnessEngine;
use crate::simulator::Simulator;

#[derive(Debug)]
pub struct ReceiveEvent {
    pub packet_index: usize,
    pub from: usize,
    pub node: usize,
}

impl Event for ReceiveEvent {
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

        if LOGGER_MODE {
            println!(
                "[RECEIVED] packet received at node:{:?} from node:{:?}.",
                node, self.from
            );
        }

        self.receive_packet(ecs, simulator, packets);
    }
}

impl ReceiveEvent {
    pub fn new(packet_index: usize, from: usize, node: usize) -> Self {
        Self {
            packet_index,
            from,
            node,
        }
    }

    /// Archive the received packet in history events.
    /// If the packet is new, create a `SendEvent` for the node.
    ///
    /// # Arguments
    ///
    /// * `ecs`: Mutable reference to [`Network`];
    /// * `simulator`: Mutable reference to [`Simulator`];
    /// * `packets`: Immutable reference to `packets`.
    ///
    pub fn receive_packet(&self, ecs: &mut Network, simulator: &mut Simulator, packets: &[Packet]) {
        let node = self.node;
        let index = self.packet_index;

        let mut packet_is_new = false;
        if let Some(history_packets) = ecs.history.get_mut(node) {
            if !history_packets.received.contains(&packets[index]) {
                packet_is_new = history_packets.received.insert(packets[index].clone());
            } else if LOGGER_MODE {
                println!(
                    "[EXIST] packet already exist in history packets at node:{:?}.",
                    node
                );
            }
        }

        if packet_is_new {
            let propagate_event = Box::new(SendEvent::new(index, self.from, self.node));
            if let Some(downlink) = ecs.downlink.get_mut(node) {
                let download_delay =
                    remaining_time_to_load(&mut downlink.link, simulator, packets[index].size);
                simulator.put_event(propagate_event, download_delay);
            }
        }
    }
}
