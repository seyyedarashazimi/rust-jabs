//! The ECS design for simulating network.
//!
//! Network is created based on
//! [Entity-Component-System](https://en.wikipedia.org/wiki/Entity_component_system)
//! design pattern.
#![allow(unused)]

pub mod node;
pub mod packet;

use crate::network::node::*;
use crate::network::packet::Packet;
use crate::simulator::event::propagate_event::PropagateEvent;
use crate::simulator::rand::RandomnessEngine;
use crate::simulator::Simulator;
use specs::prelude::*;

/// Temporary constant value standing for the debugger. Will be replaced with a
/// sophisticated debugger later.
pub const LOGGER_MODE: bool = false;

//----------State----------//
/// The state of network including the ECS `World`.
pub struct NetworkState {
    /// The Entity-Component-System(ECS) design. Each node is an `Entity`
    /// which is an index + generation. The ECS is of type `World` from
    /// `specs` crate.
    pub ecs: World,
    pub simulator: Simulator,
    pub randomness_engine: RandomnessEngine,
}

//----------Functions----------//

/// Add [`Connected`] component to a node.
///
/// # Arguments
///
/// * `ecs`: Mutable reference to a `World` type;
/// * `node`: a give node `Entity`.
fn connect_node(ecs: &mut World, node: Entity) {
    ecs.write_storage::<Connected>()
        .insert(node, Connected)
        .expect("Node was already connected.");
}

/// Remove [`Connected`] component from a node.
///
/// # Arguments
///
/// * `ecs`: Mutable reference to a `World` type;
/// * `node`: a give node `Entity`.
fn disconnect_node(ecs: &mut World, node: Entity) {
    ecs.write_storage::<Connected>()
        .remove(node)
        .expect("Node was already disconnected.");
}

/// Check if the node is online.
///
/// # Arguments
///
/// * `ecs`: Immutable reference to [`World`];
/// * `node`: [`Entity`] index denoting the node;
///
/// # Return
///
/// true if node exists and is connected.
pub fn node_is_connected(ecs: &World, node: Entity) -> bool {
    ecs.read_storage::<Connected>().get(node).is_some()
}

fn assign_random_neighbors(ecs: &mut World, min_neighbors: usize, max_neighbors: usize) {
    use rand::seq::SliceRandom;
    use rand::Rng;

    let mut rng = rand::thread_rng(); // Create a random number generator
    let nodes: Vec<Entity> = ecs.entities().join().collect(); // Collect all entities in a Vec

    for node in nodes.iter() {
        let num_neighbors = rng.gen_range(min_neighbors..=max_neighbors); // Generate a random number between min and max

        let other_nodes: Vec<&Entity> = nodes
            .iter()
            .filter(|&&neighbors| neighbors != *node) // remove itself from neighbors
            .collect();

        // Generate a random subset of other nodes to be neighbors
        let neighbors: Vec<Entity> = other_nodes
            .choose_multiple(&mut rng, num_neighbors)
            .cloned()
            .cloned()
            .collect();

        assert!(min_neighbors <= neighbors.len() && max_neighbors >= neighbors.len());

        // Insert the Neighbors component to the node
        ecs.write_storage::<Neighbors>()
            .insert(*node, Neighbors { neighbors })
            .expect("Failed to insert neighbors");
    }
}

fn create_node_connected(ecs: &mut World) -> Entity {
    ecs.create_entity()
        .with(Neighbors::default())
        .with(Connected::default())
        .with(HistoryPackets::default())
        .build()
}

pub fn create_nodes_connected_with_neighbors(
    ecs: &mut World,
    num_of_nodes: usize,
    min_neighbors: usize,
    max_neighbors: usize,
) {
    for _ in 0..num_of_nodes {
        create_node_connected(ecs);
    }
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

pub fn generate_packet_default_message(from: Entity, to: Entity, size: u64) -> Packet {
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

pub fn simulation_packet_transfer(ecs: &mut World, simulator: &mut Simulator) {
    while simulator.is_there_more_events() {
        simulator.execute_next_event(ecs);
    }
}

pub fn random_nodes_tx_rx(nodes: &mut Vec<Entity>, count: usize) -> Vec<(Entity, Entity)> {
    use rand::seq::SliceRandom;

    let mut rng = rand::thread_rng();

    let mut chosen_nodes: Vec<(Entity, Entity)> = nodes
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

/// For each neighbor connected to a node, create a ['PropagateEvent'] and push
/// into the simulator
///
/// # Arguments
///
/// * `ecs`: Mutable reference to [`World`];
/// * `simulator`: Mutable reference to [`Simulator`];
/// * `node`: [`Entity`] index denoting the node;
/// * `packet`: Immutable reference to [`Packet`] that is going to be sent to
/// neighbors of `node`.
pub fn send_to_neighbors(
    ecs: &mut World,
    simulator: &mut Simulator,
    node: Entity,
    packet: &Packet,
) {
    if let Some(neighbors) = ecs.read_storage::<Neighbors>().get(node) {
        for neighbor in &neighbors.neighbors {
            // new event:
            let transfer_event = Box::new(PropagateEvent {
                packet: packet.clone(),
                receiving_node: *neighbor,
            });
            // let mut uplinks = ecs.write_storage::<Uplink>();
            // let uplink = uplinks.get_mut(node).unwrap();
            // let upload_delay = remaining_time_to_upload(uplink, simulator, packet);
            simulator.put_event(transfer_event, 2.0);
            // println!(
            //     "Forwarding message from node:{:?} to node:{:?}",
            //     node, neighbor
            // );
        }
    }
}

fn remaining_time_to_upload(uplink: &mut Uplink, simulator: &Simulator, packet: &Packet) -> f64 {
    let uploading_time = (packet.size as f64) / (uplink.upload_bandwidth as f64);
    let send_start_time = uplink
        .latest_uploaded_time_done
        .max(simulator.simulation_time);
    let send_end_time = send_start_time + uploading_time;
    uplink.latest_uploaded_time_done = send_end_time;
    send_end_time - simulator.simulation_time
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::simulator::event::packet_generation_event;
    use crate::simulator::event::packet_generation_event::PacketGenerationEvent;
    use std::time::Instant;

    #[test]
    fn simple_working_network() {
        let tic = Instant::now();

        const NUM_OF_PACKETS: usize = 144;
        const NUM_OF_NODES: usize = 6000;
        const NUM_OF_NEIGHBORS: usize = 20;

        let mut network = NetworkState {
            ecs: World::new(),
            simulator: Simulator::new(),
            randomness_engine: RandomnessEngine::default(),
        };

        // components:
        network.ecs.register::<Neighbors>();
        network.ecs.register::<Connected>();
        network.ecs.register::<HistoryPackets>();

        create_nodes_connected_with_neighbors(
            &mut network.ecs,
            NUM_OF_NODES,
            NUM_OF_NEIGHBORS,
            NUM_OF_NEIGHBORS,
        );
        // set_bandwidth_constant(&mut network.ecs, 2, 3);

        // set sender and receiver nodes:
        let mut nodes: Vec<Entity> = network.ecs.entities().join().collect();
        let event_nodes = random_nodes_tx_rx(&mut nodes, NUM_OF_PACKETS);

        for (sender, receiver) in event_nodes {
            let initial_packet = generate_packet_default_message(sender, receiver, 1);
            let initial_event = Box::new(PacketGenerationEvent {
                packet: initial_packet,
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
