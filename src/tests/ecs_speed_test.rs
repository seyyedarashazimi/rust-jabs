use crate::network::node::connection::set_all_nodes_connected;
use crate::network::node::neighbors::assign_random_neighbors;
use crate::network::packet::generate_packet_default_message;
use crate::network::stats::assign_random_countries;
use crate::network::{simulate_propagation, Network, NetworkState, LOGGER_MODE};
use crate::simulator::event::packet_generation_event::PacketGenerationEvent;
use crate::simulator::randomness_engine::RandomnessEngine;
use crate::simulator::Simulator;
use std::time::Instant;

pub fn ecs_test() {
    let tic = Instant::now();

    const NUM_OF_PACKETS: usize = 144;
    const NUM_OF_NODES: usize = 6000;
    const NUM_OF_NEIGHBORS: usize = 8;

    const SEED: u64 = 12345;

    let mut state = NetworkState {
        ecs: Network::create_with_size(NUM_OF_NODES),
        simulator: Simulator::new(),
        randomness_engine: RandomnessEngine::new(SEED),
        packets: Vec::new(),
    };

    assign_random_countries(
        &mut state.ecs.country,
        &mut state.randomness_engine,
        NUM_OF_NODES,
    );

    set_all_nodes_connected(&mut state.ecs, NUM_OF_NODES);
    assign_random_neighbors(
        &mut state.ecs,
        &mut state.randomness_engine,
        NUM_OF_NEIGHBORS,
    );

    // set sender and receiver nodes:
    let nodes: Vec<usize> = (0..NUM_OF_NODES).collect();
    let event_nodes = state.randomness_engine.sample_nodes(&nodes, NUM_OF_PACKETS);

    // for (sender, _) in event_nodes {
    for (i, sender) in event_nodes.iter().enumerate() {
        state.packets.push(generate_packet_default_message(1, i));
        let initial_event = Box::new(PacketGenerationEvent::new(i, *sender));
        state.simulator.put_event(initial_event, 0.0);
    }

    if LOGGER_MODE {
        println!("{:?}", state.ecs.neighbors);
    }

    let tac = Instant::now();
    simulate_propagation(
        &mut state.ecs,
        &mut state.simulator,
        &mut state.randomness_engine,
        &state.packets,
    );
    let toc = Instant::now();
    let setup_duration = tac.duration_since(tic).as_millis();
    let propagate_duration = toc.duration_since(tac).as_millis();

    println!("Total Executed Events: {}", state.simulator.inserted_events);
    println!("Final Simulation Time: {}", state.simulator.simulation_time);
    println!(
        "Setup Elapsed time: {:.3}sec.",
        (setup_duration as f64) / 1000.0
    );
    println!(
        "Propagation Elapsed time: {:.3}sec.",
        (propagate_duration as f64) / 1000.0
    );
}
