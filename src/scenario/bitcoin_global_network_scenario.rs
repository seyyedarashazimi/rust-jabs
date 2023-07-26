use std::time::{Duration, Instant};
use crate::consensus::blockchain::local_block_tree::assign_initial_local_block_trees;
use crate::consensus::config::nakamoto_consensus_config::NakamotoConsensusConfig;
use crate::ledger_data::block::Block;
use crate::log::Logger;
use crate::network::resource::NetworkResource;
use crate::network::{Network, NetworkState};
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

pub struct BitcoinGlobalNetworkScenario {
    average_block_interval: f64,
    confirmation_depth: i32,
    loggers: Vec<Box<dyn Logger>>,
    name: String,
    seed: u64,
    stop_time: f64,
    average_num_of_blocks: usize,
    difficulty: f64,
    num_of_miners: usize,
    num_of_neighbors: usize,
    num_of_nodes: usize,
    progress_logger_seconds: u64,
}

impl BitcoinGlobalNetworkScenario {
    pub fn new(
        average_block_interval: f64,
        confirmation_depth: i32,
        name: &str,
        seed: u64,
        stop_time: f64,
    ) -> Self {
        Self {
            average_block_interval,
            confirmation_depth,
            loggers: Vec::new(),
            name: name.to_string(),
            seed,
            stop_time,
            average_num_of_blocks: (stop_time / average_block_interval) as usize,
            difficulty: BITCOIN_DIFFICULTY_2022,    // 225.0
            num_of_miners: BITCOIN_NUM_MINERS_2022, // 30
            num_of_neighbors: 8,
            num_of_nodes: BITCOIN_NUM_NODES_2022 + BITCOIN_NUM_MINERS_2022, // 8013
            progress_logger_seconds: 2,
        }
    }

    pub fn add_new_logger(&mut self, logger: Box<dyn Logger>) {
        self.loggers.push(logger);
    }

    fn create_network(&self) -> NetworkState {
        const GENESIS_BLOCK_INDEX: usize = 0;

        let mut state = NetworkState {
            ecs: Network::create_with_size(self.num_of_nodes),
            simulator: Simulator::new(),
            randomness_engine: RandomnessEngine::new(self.seed),
            resource: NetworkResource {
                blocks: Vec::with_capacity(self.average_num_of_blocks),
                config: NakamotoConsensusConfig::new(
                    self.average_block_interval,
                    self.confirmation_depth,
                    GENESIS_BLOCK_INDEX,
                    self.difficulty,
                ),
                miners: Vec::with_capacity(self.num_of_miners),
            },
        };

        for (node_index, consensus) in &mut state.ecs.consensus_algorithm.iter_mut().enumerate() {
            consensus.initial_configuration(&state.resource.config, node_index);
        }

        sample_bitcoin_miner_nodes(
            &mut state.resource.miners,
            &mut state.randomness_engine,
            self.num_of_nodes,
            self.num_of_miners,
        );
        sample_bitcoin_node_countries(
            &mut state.ecs.country,
            &state.resource.miners,
            &mut state.randomness_engine,
            self.num_of_nodes,
            self.num_of_miners,
        );

        set_all_nodes_connected(&mut state.ecs, self.num_of_nodes);
        assign_random_neighbors(
            &mut state.ecs,
            &mut state.randomness_engine,
            self.num_of_neighbors,
        );

        assert!(is_neighbors_bidirectional(&state.ecs.neighbors));

        assign_initial_local_block_trees(&mut state.ecs.local_block_tree, self.num_of_nodes);

        assign_all_bandwidths(
            &mut state.ecs.uplink,
            &mut state.ecs.downlink,
            &state.ecs.country,
            &mut state.randomness_engine,
            self.num_of_nodes,
        );

        // Genesis must be always the first block in the blocks. (genesis_index=0)
        state.resource.blocks.push(Block::generate_genesis_block());

        let miners = state.resource.miners.clone();
        reset_and_sample_all_bitcoin_miners_hash_power(
            &miners,
            &mut state.ecs.hash_power,
            &mut state.randomness_engine,
            self.average_block_interval,
            state.resource.config.difficulty,
        );

        state
    }

    fn insert_initial_event(&self, state: &mut NetworkState) {
        let miners = state.resource.miners.clone();
        for miner in miners {
            BlockMiningProcess::initialize_mining_event(
                miner,
                &mut state.ecs,
                &mut state.simulator,
                &mut state.randomness_engine,
                &mut state.resource,
            );
        }
    }

    pub fn run(&mut self) -> Result<(), std::io::Error> {
        let preparation_starting_time = Instant::now();
        let progress_message_intervals =
            Duration::from_secs(self.progress_logger_seconds).as_nanos();

        let scenario_data = ScenarioData::new(
            self.name.to_string(),
            self.num_of_nodes,
            "1-day of bitcoin".to_string(),
        );

        let mut state = self.create_network();

        self.insert_initial_event(&mut state);

        for logger in self.loggers.iter_mut() {
            logger.initial_log(&scenario_data)?;
        }

        // running the simulation
        eprintln!("Staring {}...", scenario_data.name);
        let simulation_starting_time = Instant::now();
        let mut last_progress_message_time = simulation_starting_time;
        while state.simulator.is_there_more_events()
            && !simulation_stop_condition(&state.simulator, self.stop_time)
        {
            let logger_info = state
                .simulator
                .peek_event()
                .unwrap()
                .logger_data(state.simulator.simulation_time);

            for logger in self.loggers.iter_mut() {
                logger.log_before_each_event(&logger_info, &state.ecs, &state.resource)?;
            }

            state.simulator.execute_next_event(
                &mut state.ecs,
                &mut state.randomness_engine,
                &mut state.resource,
            );

            for logger in self.loggers.iter_mut() {
                logger.log_after_each_event(&logger_info, &state.ecs, &state.resource)?;
            }

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
        for logger in self.loggers.iter_mut() {
            logger.final_log(&scenario_data)?;
        }
        eprintln!("Finished {}.", self.name);

        let simulation_ending_time = Instant::now();

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
}
