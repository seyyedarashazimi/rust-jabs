use crate::consensus::algorithm::DAGBasedConsensus;
use crate::consensus::blockchain::local_block_tree::assign_initial_local_block_trees;
use crate::ledger_data::bitcoin_block::BitcoinBlock;
use crate::network::bitcoin_network::BitcoinNetwork;
use crate::network::node::connection::set_all_nodes_connected;
use crate::network::node::link::assign_all_bandwidths;
use crate::network::node::neighbors::{assign_random_neighbors, is_neighbors_bidirectional};
use crate::network::stats::eighty_six_countries::bitcoin_stats::{
    reset_and_sample_all_bitcoin_miners_hash_power, sample_bitcoin_miner_nodes,
    sample_bitcoin_node_countries,
};
use crate::simulator::event::block_mining_process::BlockMiningProcess;
use crate::simulator::randomness_engine::RandomnessEngine;
use crate::simulator::Simulator;

impl BitcoinNetwork {
    pub fn prepare(
        &mut self,
        rand: &mut RandomnessEngine,
        average_block_mining_interval: f64,
        min_neighbors: usize,
        num_of_miners: usize,
    ) {
        for (node_index, consensus) in &mut self.ecs.consensus_algorithm.iter_mut().enumerate() {
            consensus.initial_configuration(&self.resource.config, node_index);
        }

        sample_bitcoin_miner_nodes(
            &mut self.resource.miners,
            rand,
            self.ecs.num_of_nodes,
            num_of_miners,
        );
        sample_bitcoin_node_countries(
            &mut self.ecs.country,
            &self.resource.miners,
            rand,
            self.ecs.num_of_nodes,
            self.resource.num_of_miners(),
        );
        set_all_nodes_connected(&mut self.ecs.is_connected, self.ecs.num_of_nodes);
        assign_random_neighbors(
            &mut self.ecs.neighbors,
            rand,
            min_neighbors,
            self.ecs.num_of_nodes,
        );

        assert!(is_neighbors_bidirectional(&self.ecs.neighbors));

        assign_initial_local_block_trees(&mut self.ecs.local_block_tree, self.ecs.num_of_nodes);

        assign_all_bandwidths(
            &mut self.ecs.uplink,
            &mut self.ecs.downlink,
            &self.ecs.country,
            rand,
            self.ecs.num_of_nodes,
        );

        // Genesis must be always the first block in the blocks. (genesis_index=0)
        self.resource
            .blocks
            .push(BitcoinBlock::generate_genesis_block());

        let miners = self.resource.miners.clone();
        reset_and_sample_all_bitcoin_miners_hash_power(
            &miners,
            &mut self.ecs.hash_power,
            rand,
            average_block_mining_interval,
            self.resource.config.difficulty,
        );
    }

    pub(crate) fn insert_initial_event(
        &mut self,
        simulator: &mut Simulator,
        rand: &mut RandomnessEngine,
    ) {
        let miners = self.resource.miners.clone();
        for miner in miners {
            self.initialize_mining_event(miner, simulator, rand);
        }
    }

    fn initialize_mining_event(
        &mut self,
        miner: usize,
        simulator: &mut Simulator,
        rand: &mut RandomnessEngine,
    ) {
        // update average_time for next generation if the difficulty gets updated.
        let average_time_between_generation: f64 =
            self.resource.config.difficulty / self.ecs.hash_power[miner].unwrap();
        let time_to_next_generation =
            rand.sample_exponential_distribution(average_time_between_generation);
        let block_mining_process = BlockMiningProcess::new(miner);
        simulator.put_event(Box::new(block_mining_process), time_to_next_generation);
    }
}
