use crate::network::node::connection::node_is_connected;
use crate::network::resource::NetworkResource;
use crate::network::Network;
use crate::simulator::event::generate_block_event::GenerateBlockEvent;
use crate::simulator::event::Event;
use crate::simulator::randomness_engine::RandomnessEngine;
use crate::simulator::Simulator;

#[derive(Debug, Clone)]
pub struct BlockMiningProcess {
    miner: usize,
}

impl Event for BlockMiningProcess {
    fn execute(
        &self,
        ecs: &mut Network,
        simulator: &mut Simulator,
        rand: &mut RandomnessEngine,
        resource: &mut NetworkResource,
    ) {
        let miner = self.miner;

        if !node_is_connected(ecs, miner) {
            return;
        }
        // generate new block
        let generate_block_event = Box::new(GenerateBlockEvent::new(miner));
        simulator.put_event(generate_block_event, 0.0);

        // continue the generation process
        BlockMiningProcess::initialize_mining_event(miner, ecs, simulator, rand, resource);
    }
}

impl BlockMiningProcess {
    pub fn new(miner: usize) -> Self {
        Self { miner }
    }

    pub fn initialize_mining_event(
        miner: usize,
        ecs: &mut Network,
        simulator: &mut Simulator,
        rand: &mut RandomnessEngine,
        resource: &mut NetworkResource,
    ) {
        // update average_time for next generation if the difficulty gets updated.
        let average_time_between_generation: f64 =
            resource.config.difficulty / ecs.hash_power[miner].unwrap();
        let time_to_next_generation =
            rand.sample_exponential_distribution(average_time_between_generation);
        let block_mining_process = BlockMiningProcess::new(miner);
        simulator.put_event(Box::new(block_mining_process), time_to_next_generation);
    }
}
