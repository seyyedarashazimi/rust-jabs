use crate::network::*;
use crate::simulator::event::packet_generation_event::PacketGenerationEvent;
use crate::simulator::randomness_engine::RandomnessEngine;
use crate::simulator::Simulator;
use std::time::Instant;

pub fn ecs_test() {
    let tic = Instant::now();

    const NUM_OF_PACKETS: usize = 144;
    const NUM_OF_NODES: usize = 6000;
    const NUM_OF_NEIGHBORS: usize = 20;

    let mut state = NetworkState {
        ecs: Network::create_with_size(NUM_OF_NODES),
        simulator: Simulator::new(),
        randomness_engine: RandomnessEngine::default(),
        packets: Vec::new(),
    };

    create_nodes_connected_with_neighbors(
        &mut state.ecs,
        NUM_OF_NODES,
        NUM_OF_NEIGHBORS,
        NUM_OF_NEIGHBORS,
    );
    // set_bandwidth_constant(&mut network.ecs, 2, 3);

    // set sender and receiver nodes:
    let mut nodes: Vec<usize> = (0..NUM_OF_NODES).collect();
    let event_nodes = random_nodes_tx_rx(&mut nodes, NUM_OF_PACKETS);

    // for (sender, _) in event_nodes {
    for i in 0..NUM_OF_PACKETS {
        state.packets.push(generate_packet_default_message(1, i));
        let initial_event = Box::new(PacketGenerationEvent::new(i, event_nodes[i].0));
        state.simulator.put_event(initial_event, 1.0);
    }

    if LOGGER_MODE {
        println!("{:?}", state.ecs.neighbors);
    }

    let tac = Instant::now();
    simulation_packet_transfer(&mut state.ecs, &mut state.simulator, &state.packets);
    let toc = Instant::now();
    let setup_duration = tac.duration_since(tic).as_millis();
    let propagate_duration = toc.duration_since(tac).as_millis();

    println!(
        "Total sent packets (total executed events): {}",
        state.simulator.inserted_events
    );
    println!("Final simulation time: {}", state.simulator.simulation_time);
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

    // println!("{:?}", initial_packet);

    // print_world(&network.ecs);
}
