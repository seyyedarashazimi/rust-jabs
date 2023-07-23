pub mod bitcoin_node_global_network_stats_86_countries;
pub mod bitcoin_pow_global_network_stats_86_countries;

use crate::network::stats::eighty_six_countries::Country;
use crate::simulator::randomness_engine::RandomnessEngine;
use bitcoin_node_global_network_stats_86_countries::BitcoinNodeGlobalNetworkStats86Countries;
use bitcoin_pow_global_network_stats_86_countries::BitcoinProofOfWorkGlobalNetworkStats86Countries;

pub fn sample_bitcoin_miner_nodes(
    miners: &mut Vec<usize>,
    rand: &mut RandomnessEngine,
    num_of_total_nodes: usize,
    num_of_miners: usize,
) {
    *miners = rand.sample_nodes(&(0..num_of_total_nodes).collect(), num_of_miners);
}

pub fn sample_bitcoin_node_countries(
    country: &mut [Country],
    miners: &[usize],
    rand: &mut RandomnessEngine,
    num_of_total_nodes: usize,
    num_of_miner: usize,
) {
    assert_eq!(country.len(), num_of_total_nodes);
    assert_eq!(miners.len(), num_of_miner);

    for (node, country) in country.iter_mut().enumerate() {
        if miners.contains(&node) {
            *country = BitcoinProofOfWorkGlobalNetworkStats86Countries::sample_miner_region(rand);
        } else {
            *country = BitcoinNodeGlobalNetworkStats86Countries::sample_region(rand);
        }
    }
}

pub fn reset_and_sample_all_bitcoin_miners_hash_power(
    miners: &[usize],
    hash_power: &mut [Option<f64>],
    rand: &mut RandomnessEngine,
    average_mining_block_interval: f64,
    difficulty: f64,
) {
    for i in 0..hash_power.len() {
        hash_power[i] = None;
    }

    // initial sampling hash-powers
    let sampled_hash_power: Vec<f64> = (0..miners.len())
        .map(|_| BitcoinProofOfWorkGlobalNetworkStats86Countries::sample_miner_hash_power(rand))
        .collect();

    let total_hash_power: f64 = sampled_hash_power.iter().sum();
    let hash_power_scale: f64 = difficulty / (total_hash_power * average_mining_block_interval);

    // scaling the hash-powers and assign to miners
    for (miner, hp) in miners.iter().zip(sampled_hash_power.iter()) {
        hash_power[*miner] = Some(hp * hash_power_scale);
    }
}
