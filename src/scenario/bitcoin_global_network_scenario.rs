use std::path::Path;
use std::time::{Duration, Instant};
use crate::consensus::blockchain::local_block_tree::assign_initial_local_block_trees;
use crate::consensus::config::nakamoto_consensus_config::NakamotoConsensusConfig;
use crate::ledger_data::block::Block;
use crate::log::block_confirmation_logger::BlockConfirmationLogger;
use crate::log::Logger;
use crate::network::resource::NetworkResource;
use crate::network::{FULL_LOGGER_MODE, LOGGER_MODE, Network, NetworkState};
use crate::network::node::connection::set_all_nodes_connected;
use crate::network::node::link::assign_all_bandwidths;
use crate::network::node::neighbors::{assign_random_neighbors, is_neighbors_bidirectional};
use crate::network::stats::eighty_six_countries::bitcoin_stats::bitcoin_node_global_network_stats_86_countries::BITCOIN_NUM_NODES_2022;
use crate::network::stats::eighty_six_countries::bitcoin_stats::{reset_and_sample_all_bitcoin_miners_hash_power, sample_bitcoin_miner_nodes, sample_bitcoin_node_countries};
use crate::network::stats::eighty_six_countries::bitcoin_stats::bitcoin_pow_global_network_stats_86_countries::{BITCOIN_DIFFICULTY_2022, BITCOIN_NUM_MINERS_2022};
use crate::scenario::ScenarioData;
use crate::simulator::event::block_mining_process::BlockMiningProcess;
use crate::simulator::randomness_engine::RandomnessEngine;
use crate::simulator::Simulator;

//----------Functions----------//
pub fn _simulate_propagation(
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

pub fn run(
    average_block_interval: f64,
    confirmation_depth: i32,
    stop_time: f64,
    seed: u64,
    name: &str,
) -> Result<(), std::io::Error> {
    let preparation_starting_time = Instant::now();

    const GENESIS_BLOCK_INDEX: usize = 0; // must be always zero.
    const NUM_OF_NODES: usize = BITCOIN_NUM_NODES_2022 + BITCOIN_NUM_MINERS_2022; // 8013
    const NUM_OF_MINERS: usize = BITCOIN_NUM_MINERS_2022; // 30
    const NUM_OF_NEIGHBORS: usize = 8;
    const PROGRESS_LOGGER_SECONDS: u64 = 2;

    let progress_message_intervals = Duration::from_secs(PROGRESS_LOGGER_SECONDS).as_nanos();

    let average_num_of_blocks: usize = (stop_time / average_block_interval) as usize;
    let difficulty: f64 = BITCOIN_DIFFICULTY_2022; // 225.0

    let scenario_data = ScenarioData::new(
        name.to_string(),
        NUM_OF_NODES,
        "1-day of bitcoin".to_string(),
    );

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
                difficulty,
            ),
            miners: Vec::with_capacity(NUM_OF_MINERS),
        },
    };

    for (node_index, consensus) in &mut state.ecs.consensus_algorithm.iter_mut().enumerate() {
        consensus.initial_configuration(&state.resource.config, node_index);
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

    // let event_nodes = state
    //     .randomness_engine
    //     .sample_nodes(&state.resource.miners, average_num_of_blocks);

    // Genesis must be always the first block in the blocks. (genesis_index=0)
    state.resource.blocks.push(Block::generate_genesis_block());

    let miners = state.resource.miners.clone();
    reset_and_sample_all_bitcoin_miners_hash_power(
        &miners,
        &mut state.ecs.hash_power,
        &mut state.randomness_engine,
        average_block_interval,
        state.resource.config.difficulty,
    );
    for miner in miners {
        BlockMiningProcess::initialize_mining_event(
            miner,
            &mut state.ecs,
            &mut state.simulator,
            &mut state.randomness_engine,
            &mut state.resource,
        );

        // let initial_event = Box::new(GenerateBlockEvent::new(miner));
        // state
        //     .simulator
        //     .put_event(initial_event, initial_event_timer);
        // initial_event_timer += average_block_interval;
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

    let logger_dir = Path::new("output");
    let mut block_confirmation_logger =
        BlockConfirmationLogger::from_path(&logger_dir.join("bitcoin-confirmations-log.csv"))?;

    block_confirmation_logger.initial_log(&scenario_data)?;

    // running the simulation
    eprintln!("Staring One day in the life of Bitcoin...");
    let simulation_starting_time = Instant::now();
    let mut last_progress_message_time = simulation_starting_time;
    while state.simulator.is_there_more_events()
        && !simulation_stop_condition(&state.simulator, stop_time)
    {
        let block_confirmation_logger_info = state.simulator.peek_event().unwrap().logger_data();

        block_confirmation_logger.log_before_each_event(
            &block_confirmation_logger_info,
            &state.ecs,
            &state.resource,
        )?;

        state.simulator.execute_next_event(
            &mut state.ecs,
            &mut state.randomness_engine,
            &mut state.resource,
        );

        block_confirmation_logger.log_after_each_event(
            &block_confirmation_logger_info,
            &state.ecs,
            &state.resource,
        )?;

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
    let simulation_ending_time = Instant::now();
    eprintln!("Finished One day in the life of Bitcoin.");

    block_confirmation_logger.final_log(&scenario_data)?;

    println!("Total Created Blocks: {}", state.resource.blocks.len() - 1);

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
    Ok(())
}
