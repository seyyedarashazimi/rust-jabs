use crate::network::stats::eighty_six_countries::{get_country_from_usize, Country};
// use crate::network::stats::eighty_six_countries::bitcoin_stats::bitcoin_node_global_network_stats_86_countries::BITCOIN_NUM_NODES_2022;
use crate::simulator::randomness_engine::RandomnessEngine;

const BITCOIN_MINER_REGION_DISTRIBUTION_2020: [f64; 86] = [
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.158, 0.0, 0.4906, 0.0, 0.0, 0.0, 0.0, 0.0,
    0.0, 0.0, 0.0, 0.0, 0.0, 0.269, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.467, 0.207, 0.0, 0.0,
    0.0, 0.765, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.366, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.647, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.1612, 0.0, 0.0, 0.0, 0.602,
];

/// Hash power probability distribution (CDF) in Bitcoin Network
const BITCOIN_HASH_POWER_DISTRIBUTION_2022: [f64; 16] = [
    0.0625, 0.0625, 0.0625, 0.0625, 0.0625, 0.0625, 0.0625, 0.0625, 0.0625, 0.0625, 0.0625, 0.0625,
    0.0625, 0.0625, 0.0625, 0.0625,
];

/// Hash power probability distribution (Hash Power Values) in Bitcoin Network
/// presented in ExaHash per second
const BITCOIN_HASH_POWER_DISTRIBUTION_BIN_2022: [f64; 16] = [
    50.0, 37.0, 33.0, 23.0, 22.0, 17.0, 13.0, 10.0, 8.0, 5.0, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0,
];

pub const BITCOIN_NUM_MINERS_2022: usize = 30;

pub const BITCOIN_DIFFICULTY_2022: f64 = 225.0;

pub struct BitcoinProofOfWorkGlobalNetworkStats86Countries {}

impl BitcoinProofOfWorkGlobalNetworkStats86Countries {
    pub fn sample_miner_region(rand: &mut RandomnessEngine) -> Country {
        get_country_from_usize(
            rand.sample_from_distribution(&BITCOIN_MINER_REGION_DISTRIBUTION_2020),
        )
    }

    pub fn sample_miner_hash_power(rand: &mut RandomnessEngine) -> f64 {
        rand.sample_from_distribution_with_bins(
            &BITCOIN_HASH_POWER_DISTRIBUTION_2022,
            &BITCOIN_HASH_POWER_DISTRIBUTION_BIN_2022,
        )
    }

    // /// returns: The share of miner nodes to all nodes in the Bitcoin network
    // ///
    // pub fn share_of_miners_to_all_nodes() -> f64 {
    //     (BITCOIN_NUM_MINERS_2022 as f64) / (BITCOIN_NUM_NODES_2022 as f64)
    // }
}
