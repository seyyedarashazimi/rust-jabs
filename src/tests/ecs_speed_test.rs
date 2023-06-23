use crate::network::node::*;
use crate::network::*;
use crate::simulator::event::packet_generation_event::PacketGenerationEvent;
use crate::simulator::rand::RandomnessEngine;
use crate::simulator::Simulator;
use specs::prelude::*;
use std::time::Instant;

pub fn ecs_test() {
    let tic = Instant::now();

    const NUM_OF_PACKETS: usize = 1;
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
