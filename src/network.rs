//! The ECS design for simulating network.
//!
//! Network is created based on
//! [Entity-Component-System](https://en.wikipedia.org/wiki/Entity_component_system)
//! design pattern.
// #![allow(unused)]

pub mod node;
pub mod packet;
pub mod stats;

use self::node::*;
use self::packet::Packet;
use self::stats::eighty_six_countries::sample_random_country;
use self::stats::eighty_six_countries::Country::{self, *};
use crate::network::stats::eighty_six_countries::{
    sample_download_bandwidth, sample_upload_bandwidth,
};
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
    pub country: Vec<Country>,
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
        let country: Vec<Country> = vec![Albania; num_of_nodes];
        Self {
            node_name,
            is_connected,
            neighbors,
            history,
            num_of_nodes,
            uplink,
            downlink,
            country,
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
#[allow(unused)]
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
#[allow(unused)]
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

fn assign_random_neighbors(ecs: &mut Network, rand: &mut RandomnessEngine, min_neighbors: usize) {
    for node in 0..ecs.num_of_nodes {
        let num_neighbors = min_neighbors;
        let other_nodes: Vec<usize> = (0..ecs.num_of_nodes)
            .filter(|&neighbors| neighbors != node) // remove itself from neighbors
            .collect();

        // Generate a random subset of other nodes to be neighbors
        let node_neighbors = rand.sample_nodes(&other_nodes, num_neighbors);

        // Add the node to the list of its neighbors
        for &n in &node_neighbors {
            ecs.neighbors[n].list.push(node);
        }

        // Insert the Neighbors component to the node
        *ecs.neighbors
            .get_mut(node)
            .expect("Failed to insert neighbors") = Neighbors {
            list: node_neighbors,
        }
    }
}

pub fn set_all_nodes_connected(ecs: &mut Network, size: usize) {
    assert_eq!(ecs.is_connected.len(), size);
    ecs.is_connected.iter_mut().for_each(|x| *x = true);
}

pub fn create_nodes_connected_with_neighbors(
    ecs: &mut Network,
    rand: &mut RandomnessEngine,
    num_of_nodes: usize,
    min_neighbors: usize,
) {
    set_all_nodes_connected(ecs, num_of_nodes);
    assign_random_neighbors(ecs, rand, min_neighbors);
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
    rand: &mut RandomnessEngine,
    packets: &[Packet],
) {
    while simulator.is_there_more_events() {
        simulator.execute_next_event(ecs, rand, packets);
    }
}

#[allow(unused)]
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

pub fn assign_random_countries(country: &mut [Country], rand: &mut RandomnessEngine, size: usize) {
    assert_eq!(country.len(), size);
    for c in country.iter_mut() {
        *c = sample_random_country(rand);
    }
}

pub fn assign_all_bandwidths(
    uplink: &mut [Uplink],
    downlink: &mut [Downlink],
    country: &[Country],
    rand: &mut RandomnessEngine,
    size: usize,
) {
    assert_eq!(uplink.len(), size);
    assert_eq!(downlink.len(), size);
    for i in 0..size {
        uplink[i].link.bandwidth = sample_upload_bandwidth(country[i], rand);
        downlink[i].link.bandwidth = sample_download_bandwidth(country[i], rand);
    }
}

pub fn is_neighbors_bidirectional(neighbors: &[Neighbors]) -> bool {
    neighbors
        .iter()
        .enumerate()
        .zip(neighbors.iter().enumerate())
        .all(|((node1, neighbors1), (node2, neighbors2))| {
            neighbors1.list.contains(&node2) == neighbors2.list.contains(&node1)
        })
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
