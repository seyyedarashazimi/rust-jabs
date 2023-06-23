#![allow(unused)]

pub mod node;
pub mod packet;

use crate::network::node::*;
use crate::network::packet::Packet;
use crate::simulator::event::PropagateEvent;
use crate::simulator::rand::RandomnessEngine;
use crate::simulator::Simulator;

// use specs::{Builder, Component, DenseVecStorage, World, WorldExt};

pub const LOGGER_MODE: bool = false;

//---------Resources---------//
// struct Simulator

//----------Systems----------//
// struct SimulatingEvents;
//
// impl<'a> System<'a> for SimulatingEvents {
//     type SystemData = (
//         Entities<'a>,
//         ReadStorage<'a, Neighbors>,
//         WriteStorage<'a, HistoryPackets>,
//     );
//
//     fn run(&mut self, data: Self::SystemData) {
//         let (nodes, neighbors, history) = data;
//     }
// }

//----------World----------//
pub struct Network {
    // components:
    pub node_name: Vec<NodeName>,
    pub is_connected: Vec<bool>,
    pub neighbors: Vec<Neighbors>,
    pub history: Vec<HistoryPackets>,
    // entities:
    pub num_of_nodes: usize,
    // node: Vec<usize>,
}

impl Network {
    pub fn create_with_size(num_of_nodes: usize) -> Self {
        let node_name: Vec<NodeName> = vec![NodeName::default(); num_of_nodes];
        let is_connected: Vec<bool> = vec![false; num_of_nodes];
        let neighbors: Vec<Neighbors> = vec![Neighbors::default(); num_of_nodes];
        let history: Vec<HistoryPackets> = vec![HistoryPackets::default(); num_of_nodes];
        Self {
            node_name,
            is_connected,
            neighbors,
            history,
            num_of_nodes,
        }
    }
}

//----------State----------//
pub struct NetworkState {
    pub ecs: Network,
    pub simulator: Simulator,
    pub randomness_engine: RandomnessEngine,
}

//----------Functions----------//
fn net_write_component<T>(ecs: &mut Network, node: usize, value: T) -> Result<(), &'static str> {
    if let Some(status) = ecs.is_connected.get_mut(node) {
        *status = true;
        return Ok(());
    }
    Err("Index out of bounds")
}

fn connect_node(ecs: &mut Network, node: usize) -> Result<(), String> {
    if let Some(status) = ecs.is_connected.get_mut(node) {
        *status = true;
        return Ok(());
    }
    Err(format!("Index out of bounds for connect_node: {}", node))
}

fn disconnect_node(ecs: &mut Network, node: usize) -> Result<(), String> {
    if let Some(status) = ecs.is_connected.get_mut(node) {
        *status = false;
        return Ok(());
    }
    Err(format!("Index out of bounds for disconnect_node: {}", node))
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

// fn create_node_connected(ecs: &mut World) -> Entity {
//     ecs.create_entity()
//         .with(Neighbors::default())
//         .with(Bandwidth::default())
//         .with(NodeType::default())
//         .with(Connected::default())
//         .with(HistoryPackets::default())
//         .build()
// }

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

// pub fn set_bandwidth_constant(ecs: &mut World, download: i64, upload: i64) {
//     for node in ecs.entities().join() {
//         ecs.write_storage::<Bandwidth>()
//             .insert(node, Bandwidth { download, upload })
//             .expect("Failed to insert constant bandwidths.");
//     }
// }

// fn set_neighbor_bandwidth(ecs: &mut World) {
//     let nodes = ecs.entities();
//     let is_26 = ecs.read_storage::<Is26>();
//     let neighbors_storage = ecs.read_storage::<Neighbors>();
//
//     for (node_26, _) in (&nodes, &is_26).join() {
//         let node_neighbors = neighbors_storage
//             .get(node_26)
//             .expect("Node should have Neighbors component");
//
//         let mut bandwidth_storage = ecs.write_storage::<Bandwidth>();
//
//         for neighbor in &node_neighbors.neighbors {
//             if let Some(neighbor_bandwidth) = bandwidth_storage.get_mut(*neighbor) {
//                 neighbor_bandwidth.download = 26;
//             }
//         }
//     }
// }

pub fn generate_packet_default_message(from: usize, to: usize, size: u64) -> Packet {
    if LOGGER_MODE {
        println!(
            "A new packet generated! from:{:?}, to:{:?}, size:{:?}",
            from, to, size
        );
    }
    Packet {
        from,
        to,
        size,
        msg: format!("packet info: from:{:?}, to:{:?}, size:{:?}", from, to, size),
    }
}

pub fn simulation_packet_transfer(ecs: &mut Network, simulator: &mut Simulator) {
    while simulator.is_there_more_events() {
        simulator.execute_next_event(ecs);
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::simulator::event::PropagateEvent;
    use std::time::Instant;

    #[test]
    fn simple_working_network() {
        let tic = Instant::now();

        const NUM_OF_PACKETS: usize = 144;
        const NUM_OF_NODES: usize = 6000;
        const NUM_OF_NEIGHBORS: usize = 20;

        let mut network = NetworkState {
            ecs: Network::create_with_size(NUM_OF_NODES),
            simulator: Simulator::new(),
            randomness_engine: RandomnessEngine::default(),
        };

        create_nodes_connected_with_neighbors(
            &mut network.ecs,
            NUM_OF_NODES,
            NUM_OF_NEIGHBORS,
            NUM_OF_NEIGHBORS,
        );
        // set_bandwidth_constant(&mut network.ecs, 2, 3);

        // set sender and receiver nodes:
        let mut nodes: Vec<usize> = (0..NUM_OF_NODES).collect();
        let event_nodes = random_nodes_tx_rx(&mut nodes, NUM_OF_PACKETS);

        for (sender, receiver) in event_nodes {
            let initial_packet = generate_packet_default_message(sender, receiver, 1);
            let initial_event = Box::new(PropagateEvent {
                packet: initial_packet,
                receiving_node: sender, // give sender the initial packet
            });
            network.simulator.put_event(initial_event, 1.0);
        }

        let tac = Instant::now();
        simulation_packet_transfer(&mut network.ecs, &mut network.simulator);
        let toc = Instant::now();
        let setup_duration = tac.duration_since(tic).as_millis();
        let propagate_duration = toc.duration_since(tac).as_millis();

        println!(
            "Total sent packets (total executed events): {}",
            network.simulator.inserted_events
        );
        println!(
            "Final simulation time: {}",
            network.simulator.simulation_time
        );
        println!(
            "Setup Elapsed time: {:?}.{:?}sec.",
            setup_duration / 1000,
            setup_duration % 1000
        );
        println!(
            "Propagation Elapsed time: {:?}.{:?}sec.",
            propagate_duration / 1000,
            propagate_duration % 1000
        );

        // print_world(&network.ecs);
    }
}
