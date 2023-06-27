//! The ECS design for simulating network.
//!
//! Network is created based on
//! [Entity-Component-System](https://en.wikipedia.org/wiki/Entity_component_system)
//! design pattern.
#![allow(unused)]

pub mod node;
pub mod packet;
mod stats;

use crate::network::node::*;
use crate::network::packet::Packet;
use crate::simulator::randomness_engine::RandomnessEngine;
use crate::simulator::Simulator;

/// Temporary constant value standing for the debugger. Will be replaced with a
/// sophisticated debugger later.
pub const LOGGER_MODE: bool = false;

//----------State----------//
/// The state of network including the ECS `World`.
pub struct NetworkState {
    /// The Entity-Component-System(ECS) design. Each node is a `usize`
    /// which is an index + generation. The ECS is of type `Network` including
    /// all components(raw data) for each node as vectors of values.
    pub ecs: Network,
    pub simulator: Simulator,
    pub randomness_engine: RandomnessEngine,
    pub packets: Vec<Packet>,
}

//----------ECS----------//
/// The Entity-Component-System(ECS) design. Each node is solely denoted by a
/// `usize` number.
pub struct Network {
    // components:
    pub node_name: Vec<Option<NodeName>>,
    pub is_connected: Vec<bool>,
    pub neighbors: Vec<Neighbors>,
    pub history: Vec<HistoryPackets>,
    pub uplink: Vec<Uplink>,
    pub downlink: Vec<Downlink>,
    // entities:
    pub num_of_nodes: usize,
    // node: Vec<usize>,
}

impl Network {
    pub fn create_with_size(num_of_nodes: usize) -> Self {
        let node_name: Vec<Option<NodeName>> = vec![Some(NodeName::default()); num_of_nodes];
        let is_connected: Vec<bool> = vec![false; num_of_nodes];
        let neighbors: Vec<Neighbors> = vec![Neighbors::default(); num_of_nodes];
        let history: Vec<HistoryPackets> = vec![HistoryPackets::default(); num_of_nodes];
        let uplink: Vec<Uplink> = vec![Uplink::default(); num_of_nodes];
        let downlink: Vec<Downlink> = vec![Downlink::default(); num_of_nodes];
        Self {
            node_name,
            is_connected,
            neighbors,
            history,
            num_of_nodes,
            uplink,
            downlink,
        }
    }
}

//----------Functions----------//

/// Add [`Connected`] component to a node.
///
/// # Arguments
///
/// * `ecs`: Mutable reference to a `Network` type;
/// * `node`: a give node `usize`.
fn connect_node(ecs: &mut Network, node: usize) -> Result<(), String> {
    if let Some(status) = ecs.is_connected.get_mut(node) {
        *status = true;
        return Ok(());
    }
    Err(format!("Index out of bounds for connect_node: {}", node))
}

/// Remove [`Connected`] component from a node.
///
/// # Arguments
///
/// * `ecs`: Mutable reference to a `Network` type;
/// * `node`: a give node `usize`.
fn disconnect_node(ecs: &mut Network, node: usize) -> Result<(), String> {
    if let Some(status) = ecs.is_connected.get_mut(node) {
        *status = false;
        return Ok(());
    }
    Err(format!("Index out of bounds for disconnect_node: {}", node))
}

/// Check if the node is online.
///
/// # Arguments
///
/// * `ecs`: Immutable reference to [`Network`];
/// * `node`: `usize` index denoting the node;
///
/// # Return
///
/// true if node exists and is connected.
///
/// # Examples
///
/// ```
/// use rust_jabs::network::{Network, node_is_connected};
/// let mut network = Network::create_with_size(1);
/// network.is_connected[0] = true;
///
/// assert_eq!(node_is_connected(&network, 0), true);
/// assert_eq!(node_is_connected(&network, 1), false);
/// assert_eq!(node_is_connected(&network, 2), false);
/// ```
pub fn node_is_connected(ecs: &Network, node: usize) -> bool {
    ecs.is_connected.get(node).map_or(false, |&status| status)
}

fn assign_random_neighbors(ecs: &mut Network, min_neighbors: usize, max_neighbors: usize) {
    use rand::seq::SliceRandom;
    use rand::Rng;

    let mut rng = rand::thread_rng(); // Create a random number generator
    let nodes: Vec<usize> = (0..ecs.num_of_nodes).collect();

    for node in nodes.iter() {
        let num_neighbors = rng.gen_range(min_neighbors..=max_neighbors); // Generate a random number between min and max

        let other_nodes: Vec<&usize> = nodes
            .iter()
            .filter(|&&neighbors| neighbors != *node) // remove itself from neighbors
            .collect();

        // Generate a random subset of other nodes to be neighbors
        let neighbors: Vec<usize> = other_nodes
            .choose_multiple(&mut rng, num_neighbors)
            .cloned()
            .cloned()
            .collect();

        assert!(min_neighbors <= neighbors.len() && max_neighbors >= neighbors.len());

        // Insert the Neighbors component to the node
        *ecs.neighbors
            .get_mut(*node)
            .expect("Failed to insert neighbors") = Neighbors { neighbors };
    }
}

pub fn set_all_nodes_connected(ecs: &mut Network) {
    ecs.is_connected.iter_mut().for_each(|mut x| *x = true);
}

pub fn create_nodes_connected_with_neighbors(
    ecs: &mut Network,
    num_of_nodes: usize,
    min_neighbors: usize,
    max_neighbors: usize,
) {
    set_all_nodes_connected(ecs);
    assign_random_neighbors(ecs, min_neighbors, max_neighbors);
}

pub fn generate_packet_default_message(size: u64, ctr: usize) -> Packet {
    Packet {
        msg: format!("packet info: size:{:?}, ctr:{:?}", size, ctr),
        size,
    }
}

pub fn simulation_packet_transfer(
    ecs: &mut Network,
    simulator: &mut Simulator,
    packets: &Vec<Packet>,
) {
    while simulator.is_there_more_events() {
        simulator.execute_next_event(ecs, packets);
    }
}

pub fn random_nodes_tx_rx(nodes: &mut Vec<usize>, count: usize) -> Vec<(usize, usize)> {
    use rand::seq::SliceRandom;

    let mut rng = rand::thread_rng();

    let mut chosen_nodes: Vec<(usize, usize)> = nodes
        .choose_multiple(&mut rng, count)
        .zip(nodes.choose_multiple(&mut rng, count))
        .filter(|(&tx, &rx)| tx != rx)
        .map(|(&tx, &rx)| (tx, rx))
        .collect();

    while chosen_nodes.len() < count {
        let send_extra = *nodes.choose(&mut rng).unwrap();
        let receive_extra = *nodes.choose(&mut rng).unwrap();
        if send_extra != receive_extra {
            chosen_nodes.push((send_extra, receive_extra));
        }
    }
    chosen_nodes
}

pub fn remaining_time_to_load(link: &mut Link, simulator: &Simulator, size: u64) -> f64 {
    let loading_time = (size as f64) / link.bandwidth;
    let start_time = link.latest_loaded_time_done.max(simulator.simulation_time);
    let end_time = start_time + loading_time;
    link.latest_loaded_time_done = end_time;
    end_time - simulator.simulation_time
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     use crate::simulator::event::packet_generation_event;
//     use crate::simulator::event::packet_generation_event::PacketGenerationEvent;
//     use std::time::Instant;
//
//     #[test]
//     fn simple_working_network() {
//         let tic = Instant::now();
//
//         const NUM_OF_PACKETS: usize = 144;
//         const NUM_OF_NODES: usize = 6000;
//         const NUM_OF_NEIGHBORS: usize = 20;
//
//         let mut state = NetworkState {
//             ecs: Network::create_with_size(NUM_OF_NODES),
//             simulator: Simulator::new(),
//             randomness_engine: RandomnessEngine::default(),
//             packets: Vec::new(),
//         };
//
//         create_nodes_connected_with_neighbors(
//             &mut state.ecs,
//             NUM_OF_NODES,
//             NUM_OF_NEIGHBORS,
//             NUM_OF_NEIGHBORS,
//         );
//         // set_bandwidth_constant(&mut network.ecs, 2, 3);
//
//         // set sender and receiver nodes:
//         let mut nodes: Vec<usize> = (0..NUM_OF_NODES).collect();
//         let event_nodes = random_nodes_tx_rx(&mut nodes, NUM_OF_PACKETS);
//
//         // for (sender, _) in event_nodes {
//         for i in 0..NUM_OF_PACKETS {
//             state.packets.push(generate_packet_default_message(1, i));
//             let initial_event = Box::new(PacketGenerationEvent::new(i, event_nodes[i].0));
//             state.simulator.put_event(initial_event, 1.0);
//         }
//
//         if LOGGER_MODE {
//             println!("{:?}", state.ecs.neighbors);
//         }
//
//         let tac = Instant::now();
//         simulation_packet_transfer(&mut state.ecs, &mut state.simulator, &state.packets);
//         let toc = Instant::now();
//         let setup_duration = tac.duration_since(tic).as_millis();
//         let propagate_duration = toc.duration_since(tac).as_millis();
//
//         println!(
//             "Total sent packets (total executed events): {}",
//             state.simulator.inserted_events
//         );
//         println!("Final simulation time: {}", state.simulator.simulation_time);
//         println!(
//             "Setup Elapsed time: {:?}.{:?}sec.",
//             setup_duration / 1000,
//             setup_duration % 1000
//         );
//         println!(
//             "Propagation Elapsed time: {:?}.{:?}sec.",
//             propagate_duration / 1000,
//             propagate_duration % 1000
//         );
//
//         // println!("{:?}", initial_packet);
//
//         // print_world(&network.ecs);
//     }
// }
