use crate::network::bitcoin_network::BitcoinNetwork;
use crate::simulator::event::block_mining_process::BlockMiningProcess;
use crate::simulator::event::generate_block_event::GenerateBlockWithoutTxEvent;
use crate::simulator::randomness_engine::RandomnessEngine;
use crate::simulator::Simulator;

impl BitcoinNetwork {
    pub fn mine_new_block(
        &mut self,
        simulator: &mut Simulator,
        rand: &mut RandomnessEngine,
        miner: usize,
    ) {
        // generate new block
        let generate_block_event = Box::new(GenerateBlockWithoutTxEvent::new(miner));
        simulator.put_event(generate_block_event, 0.0);

        // update average_time for next generation if the difficulty gets updated.
        let average_time_between_generation: f64 =
            self.resource.config.difficulty / self.ecs.hash_power[miner].unwrap();
        let time_to_next_generation =
            rand.sample_exponential_distribution(average_time_between_generation);
        let block_mining_process = BlockMiningProcess::new(miner);
        simulator.put_event(Box::new(block_mining_process), time_to_next_generation);
    }
}
