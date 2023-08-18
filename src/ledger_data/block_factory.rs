use crate::ledger_data::bitcoin_block::BitcoinBlock;
use crate::simulator::randomness_engine::RandomnessEngine;
use crate::simulator::Simulator;

pub const ETHEREUM_BLOCK_HEADER_SIZE: u64 = 543; // A header could have variable size but mostly its really close this value
pub const ETHEREUM_BLOCK_HASH_SIZE: u64 = 36; // 32 byte hash + 4 byte network id
pub const ETHEREUM_MIN_DIFFICULTY: f64 = 17_146_335_232.0;

pub const BITCOIN_BLOCK_HEADER_SIZE: u64 = 80;
pub const BITCOIN_INV_SIZE: u64 = 36; // 4 byte type + 32 byte hash
pub const GET_DATA_OVERHEAD: u64 = 4;
pub const INV_MESSAGE_OVERHEAD: u64 = 1;
pub const COMPACT_REDUCTION_RATIO: f64 = 6.0 / 200.0;

pub const FORK_LOGGER: bool = true;

// pub const BITCOIN_COMPACT_BLOCK_SIZE_2020_BINS: [i64; 22] = [
//     30, 3624, 7668, 11910, 16644, 21828, 27558, 30672, 31662, 32544, 33420, 34878, 35544, 36198,
//     36840, 37476, 38130, 38838, 39630, 40674, 42732, 72714,
// ];

pub const BITCOIN_BLOCK_SIZE_2020_BINS: [f64; 23] = [
    196.0, 119880.0, 254789.0, 396047.0, 553826.0, 726752.0, 917631.0, 1021479.0, 1054560.0,
    1084003.0, 1113136.0, 1138722.0, 1161695.0, 1183942.0, 1205734.0, 1227090.0, 1248408.0,
    1270070.0, 1293647.0, 1320186.0, 1354939.0, 1423459.0, 2422858.0,
];

pub const BITCOIN_BLOCK_SIZE_2020: [f64; 23] = [
    0.0000, 0.0482, 0.0422, 0.0422, 0.0421, 0.0422, 0.0421, 0.0445, 0.0455, 0.0458, 0.0461, 0.0468,
    0.0472, 0.0481, 0.0477, 0.0479, 0.0484, 0.0482, 0.0475, 0.0464, 0.0454, 0.0434, 0.0420,
];

pub struct BlockFactory;

impl BlockFactory {
    pub fn sample_bitcoin_block_size(rand: &mut RandomnessEngine) -> u64 {
        rand.sample_from_distribution_with_bins(
            &BITCOIN_BLOCK_SIZE_2020,
            &BITCOIN_BLOCK_SIZE_2020_BINS,
        ) as u64
    }

    pub fn sample_bitcoin_block(
        blocks: &[BitcoinBlock],
        simulator: &Simulator,
        rand: &mut RandomnessEngine,
        creator: Option<usize>,
        parent: usize,
        difficulty: f64,
        weight: f64,
    ) -> BitcoinBlock {
        if FORK_LOGGER {
            for (index, block) in blocks.iter().enumerate() {
                if block.parents.contains(&parent) {
                    println!(
                        "A fork for the parent:{}, block1:{}, block2:{}",
                        parent,
                        index,
                        blocks.len()
                    );
                }
            }
        }

        BitcoinBlock::new_with_parents(
            simulator.simulation_time,
            creator,
            blocks[parent].height + 1,
            vec![parent],
            BlockFactory::compact_size(BlockFactory::sample_bitcoin_block_size(rand)),
            difficulty,
            weight,
        )
    }

    fn compact_size(size: u64) -> u64 {
        ((((size - BITCOIN_BLOCK_HEADER_SIZE) as f64) * COMPACT_REDUCTION_RATIO) as u64)
            + BITCOIN_BLOCK_HEADER_SIZE
    }
}
