use std::time::{Duration, Instant};
use crate::log::Logger;
use crate::network::bitcoin_network::BitcoinNetwork;
use crate::network::Network;
use crate::network::stats::eighty_six_countries::bitcoin_stats::bitcoin_node_global_network_stats_86_countries::BITCOIN_NUM_NODES_2022;
use crate::network::stats::eighty_six_countries::bitcoin_stats::bitcoin_pow_global_network_stats_86_countries::{BITCOIN_DIFFICULTY_2022, BITCOIN_NUM_MINERS_2022};
use crate::scenario::ScenarioData;
use crate::simulator::randomness_engine::RandomnessEngine;
use crate::simulator::Simulator;

//----------Functions----------//
pub fn _simulate_propagation(
    network: &mut dyn Network,
    simulator: &mut Simulator,
    rand: &mut RandomnessEngine,
    stop_time: f64,
) {
    while simulator.is_there_more_events() && !simulation_stop_condition(simulator, stop_time) {
        simulator.execute_next_event(network, rand);
    }
}

fn simulation_stop_condition(simulator: &Simulator, stop_time: f64) -> bool {
    simulator.simulation_time > stop_time
}

pub struct BitcoinGlobalNetworkScenario {
    average_block_mining_interval: f64,
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
            average_block_mining_interval: average_block_interval,
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

    pub fn run(&mut self) -> Result<(), std::io::Error> {
        const GENESIS_BLOCK_INDEX: usize = 0;

        let preparation_starting_time = Instant::now();
        let progress_message_intervals =
            Duration::from_secs(self.progress_logger_seconds).as_nanos();

        let scenario_data = ScenarioData::new(
            self.name.to_string(),
            self.num_of_nodes,
            "1-day of bitcoin".to_string(),
        );

        // create network, simulator, randomness_engine and prepare the network.
        let mut network = BitcoinNetwork::new_with_size(
            self.num_of_nodes,
            self.average_num_of_blocks,
            self.average_block_mining_interval,
            self.confirmation_depth,
            GENESIS_BLOCK_INDEX,
            self.difficulty,
            self.num_of_miners,
        );
        let mut simulator = Simulator::new();
        let mut rand = RandomnessEngine::new(self.seed);

        network.prepare(
            &mut rand,
            self.average_block_mining_interval,
            self.num_of_neighbors,
            self.num_of_miners,
        );
        network.insert_initial_event(&mut simulator, &mut rand);

        for logger in self.loggers.iter_mut() {
            logger.initial_log(&scenario_data)?;
        }

        // running the simulation
        eprintln!("Staring {}...", scenario_data.name);
        let simulation_starting_time = Instant::now();
        let mut last_progress_message_time = simulation_starting_time;
        while simulator.is_there_more_events()
            && !simulation_stop_condition(&simulator, self.stop_time)
        {
            let logger_info = simulator
                .peek_event()
                .unwrap()
                .logger_data(simulator.simulation_time);

            for logger in self.loggers.iter_mut() {
                logger.log_before_each_event(&logger_info, &network)?;
            }

            simulator.execute_next_event(&mut network, &mut rand);

            for logger in self.loggers.iter_mut() {
                logger.log_after_each_event(&logger_info, &network)?;
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

                let simulation_time = Duration::from_secs_f64(simulator.simulation_time).as_secs();
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

        println!(
            "Total Created Blocks: {}",
            network.resource.blocks.len() - 1
        );

        let setup_duration = simulation_starting_time
            .duration_since(preparation_starting_time)
            .as_millis();
        let propagate_duration = simulation_ending_time
            .duration_since(simulation_starting_time)
            .as_millis();
        println!("Total Executed Events: {}", simulator.inserted_events);
        println!("Final Simulation Time: {}", simulator.simulation_time);
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
