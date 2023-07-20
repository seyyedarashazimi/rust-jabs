use std::time::{Duration, Instant};
use crate::consensus::blockchain::local_block_tree::assign_initial_local_block_trees;
use crate::consensus::config::nakamoto_consensus_config::NakamotoConsensusConfig;
use crate::ledger_data::block::Block;
use crate::network::resource::NetworkResource;
use crate::network::{FULL_LOGGER_MODE, LOGGER_MODE, Network, NetworkState};
use crate::network::node::connection::set_all_nodes_connected;
use crate::network::node::link::assign_all_bandwidths;
use crate::network::node::neighbors::{assign_random_neighbors, is_neighbors_bidirectional};
use crate::network::stats::eighty_six_countries::bitcoin_stats::bitcoin_node_global_network_stats_86_countries::BITCOIN_NUM_NODES_2022;
use crate::network::stats::eighty_six_countries::bitcoin_stats::{sample_bitcoin_miner_nodes, sample_bitcoin_node_countries};
use crate::network::stats::eighty_six_countries::bitcoin_stats::bitcoin_pow_global_network_stats_86_countries::BITCOIN_NUM_MINERS_2022;
use crate::simulator::event::generate_block_event::GenerateBlockEvent;
use crate::simulator::randomness_engine::RandomnessEngine;
use crate::simulator::Simulator;

//----------Functions----------//
pub fn simulate_propagation(
    ecs: &mut Network,
    simulator: &mut Simulator,
    rand: &mut RandomnessEngine,
    resource: &mut NetworkResource,
    stop_time: f64,
) {
    while simulator.is_there_more_events() && !simulation_stop_condition(simulator, stop_time) {
        simulator.execute_next_event(ecs, rand, resource);
    }
}

fn simulation_stop_condition(simulator: &Simulator, stop_time: f64) -> bool {
    simulator.simulation_time > stop_time
}

pub fn run(average_block_interval: f64, confirmation_depth: i32, stop_time: f64, seed: u64) {
    let preparation_starting_time = Instant::now();

    const GENESIS_BLOCK_INDEX: usize = 0;
    const NUM_OF_NODES: usize = BITCOIN_NUM_NODES_2022; // 7983;
    const NUM_OF_MINERS: usize = BITCOIN_NUM_MINERS_2022; // 30
    const NUM_OF_NEIGHBORS: usize = 8;
    const PROGRESS_LOGGER_SECONDS: u64 = 2;

    let progress_message_intervals = Duration::from_secs(PROGRESS_LOGGER_SECONDS).as_nanos();

    let average_num_of_blocks: usize = (stop_time / average_block_interval) as usize;

    let mut state = NetworkState {
        ecs: Network::create_with_size(NUM_OF_NODES),
        simulator: Simulator::new(),
        randomness_engine: RandomnessEngine::new(seed),
        resource: NetworkResource {
            blocks: Vec::with_capacity(average_num_of_blocks),
            config: NakamotoConsensusConfig::new(
                average_block_interval,
                confirmation_depth,
                GENESIS_BLOCK_INDEX,
            ),
            miners: Vec::with_capacity(NUM_OF_MINERS),
        },
    };

    for consensus in &mut state.ecs.consensus_algorithm {
        consensus.initial_configuration(&state.resource.config);
    }

    sample_bitcoin_miner_nodes(
        &mut state.resource.miners,
        &mut state.randomness_engine,
        NUM_OF_NODES,
        NUM_OF_MINERS,
    );
    sample_bitcoin_node_countries(
        &mut state.ecs.country,
        &state.resource.miners,
        &mut state.randomness_engine,
        NUM_OF_NODES,
        NUM_OF_MINERS,
    );

    set_all_nodes_connected(&mut state.ecs, NUM_OF_NODES);
    assign_random_neighbors(
        &mut state.ecs,
        &mut state.randomness_engine,
        NUM_OF_NEIGHBORS,
    );

    assign_initial_local_block_trees(&mut state.ecs.local_block_tree, NUM_OF_NODES);

    assign_all_bandwidths(
        &mut state.ecs.uplink,
        &mut state.ecs.downlink,
        &state.ecs.country,
        &mut state.randomness_engine,
        NUM_OF_NODES,
    );

    let event_nodes = state
        .randomness_engine
        .sample_nodes(&state.resource.miners, average_num_of_blocks);

    // Genesis must be always the first block in the blocks. (genesis_index=0)
    state.resource.blocks.push(Block::generate_genesis_block());

    let mut initial_event_timer: f64 = 0.0;
    for miner in event_nodes {
        let initial_event = Box::new(GenerateBlockEvent::new(miner));
        state
            .simulator
            .put_event(initial_event, initial_event_timer);
        initial_event_timer += average_block_interval;
    }

    if LOGGER_MODE && FULL_LOGGER_MODE {
        println!("{:?}", state.ecs.neighbors);
        println!("{:?}", state.resource.blocks);
        println!("{:?}", state.ecs.local_block_tree);
        if is_neighbors_bidirectional(&state.ecs.neighbors) {
            println!("Neighbors are bidirectional.")
        } else {
            println!("neighbors are not assigned correctly.")
        }
    }

    // running the simulation
    eprintln!("Staring One day in the life of Bitcoin...");
    let simulation_starting_time = Instant::now();
    let mut last_progress_message_time = simulation_starting_time;
    while state.simulator.is_there_more_events()
        && !simulation_stop_condition(&state.simulator, stop_time)
    {
        state.simulator.execute_next_event(
            &mut state.ecs,
            &mut state.randomness_engine,
            &mut state.resource,
        );

        if Instant::now()
            .duration_since(last_progress_message_time)
            .as_nanos()
            > progress_message_intervals
        {
            let real_time = Instant::now()
                .duration_since(simulation_starting_time)
                .as_secs();
            let real_time_hour = (real_time / 3600) % 24;
            let real_time_minute = (real_time / 60) % 60;
            let real_time_second = real_time % 60;

            let simulation_time =
                Duration::from_secs_f64(state.simulator.simulation_time).as_secs();
            let simulation_time_hour = (simulation_time / 3600) % 24;
            let simulation_time_minute = (simulation_time / 60) % 60;
            let simulation_time_second = simulation_time % 60;

            eprintln!("Simulation in progress... Elapsed Real Time: {:02}:{:02}:{:02}, Elapsed Simulation Time: {:02}:{:02}:{:02}", real_time_hour, real_time_minute, real_time_second, simulation_time_hour, simulation_time_minute, simulation_time_second);
            last_progress_message_time = Instant::now();
        }
    }
    // simulate_propagation(
    //     &mut state.ecs,
    //     &mut state.simulator,
    //     &mut state.randomness_engine,
    //     &mut state.resource,
    //     stop_time,
    // );
    let simulation_ending_time = Instant::now();
    eprintln!("Finished One day in the life of Bitcoin.");

    println!("total created blocks:{}", state.resource.blocks.len());
    let setup_duration = simulation_starting_time
        .duration_since(preparation_starting_time)
        .as_millis();
    let propagate_duration = simulation_ending_time
        .duration_since(simulation_starting_time)
        .as_millis();

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
