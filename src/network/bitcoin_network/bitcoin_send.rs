use crate::network::bitcoin_network::BitcoinNetwork;
use crate::network::node::link::remaining_time_to_load;
use crate::network::stats::eighty_six_countries::get_latency;
use crate::simulator::event::receive_event::ReceiveEvent;
use crate::simulator::event::send_event::SendEvent;
use crate::simulator::randomness_engine::RandomnessEngine;
use crate::simulator::Simulator;

// Send methods and associated functions:
impl BitcoinNetwork {
    /// For each neighbor connected to a node, create a ['ReceiveEvent'] and
    /// push into the simulator. It will avoid forwarding the packet to the node
    /// `send_event.from` which had sent the packet to this node.
    ///
    /// # Arguments
    ///
    /// * `ecs`: Mutable reference to [`BitcoinECS`];
    /// * `simulator`: Mutable reference to [`Simulator`];
    /// * `packets`: Immutable reference to `packets`.
    ///
    pub(crate) fn send_inv_to_neighbors(
        &mut self,
        simulator: &mut Simulator,
        rand: &mut RandomnessEngine,
        send_event: &SendEvent,
    ) {
        if let Some(neighbors) = self.ecs.neighbors.get(send_event.node) {
            // remove the sender of the packet from the set of neighbors:
            let filtered_neighbors: Vec<usize> = neighbors
                .0
                .iter()
                .filter(|&neighbor| *neighbor != send_event.from)
                .cloned()
                .collect();

            for neighbor in filtered_neighbors {
                self.simulate_upload(simulator, rand, neighbor, send_event);
            }
        }
    }

    pub(crate) fn simulate_upload(
        &mut self,
        simulator: &mut Simulator,
        rand: &mut RandomnessEngine,
        to: usize,
        send_event: &SendEvent,
    ) {
        let node = send_event.node;
        let index = send_event.block_index;

        let forward_event = Box::new(ReceiveEvent::new(index, node, to, send_event.msg_type));
        if let Some(uplink) = self.ecs.uplink.get_mut(node) {
            let size = send_event.msg_type.get_size(index, &self.resource.blocks);
            let upload_delay = remaining_time_to_load(&mut uplink.link, simulator, size);
            let delivery_delay = get_latency(self.ecs.country[node], self.ecs.country[to], rand);
            simulator.put_event(forward_event, upload_delay + delivery_delay);
        }
    }
}
