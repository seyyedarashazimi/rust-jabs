use crate::ledger_data::block::Block;
use crate::ledger_data::pow::PoW;
use crate::ledger_data::single_parent::SingleParent;
use std::cmp::Ordering;

#[derive(Debug, Default, Hash, Eq, PartialEq, Clone)]
pub struct BitcoinBlock {
    creation_time_int: u64, // to directly have Hash and Eq traits.
    pub creator: Option<usize>,
    pub height: i32,
    pub parents: Vec<usize>,
    pub size: u64,
    difficulty_int: u64,
    weight_int: u64,
}

impl PartialOrd for BitcoinBlock {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Compare the `height` of two blocks.
impl Ord for BitcoinBlock {
    fn cmp(&self, other: &Self) -> Ordering {
        self.height.cmp(&other.height)
    }
}

impl Block for BitcoinBlock {
    /// returns the float value of creation time when needed.
    fn get_creation_time(&self) -> f64 {
        f64::from_bits(self.creation_time_int)
    }
    fn get_creator(&self) -> Option<usize> {
        self.creator
    }
    fn get_height(&self) -> i32 {
        self.height
    }
    fn get_size(&self) -> u64 {
        self.size
    }
    fn get_parents(&self) -> &Vec<usize> {
        &self.parents
    }

    fn set_creation_time(&mut self, creation_time: f64) {
        self.creation_time_int = creation_time.to_bits();
    }
    fn set_creator(&mut self, creator: Option<usize>) {
        self.creator = creator;
    }
    fn set_height(&mut self, height: i32) {
        self.height = height;
    }
    fn set_size(&mut self, size: u64) {
        self.size = size;
    }
    fn set_parents(&mut self, parents: Vec<usize>) {
        self.parents = parents
    }
}

impl SingleParent for BitcoinBlock {
    fn get_single_parent(&self) -> Option<usize> {
        if self.parents.is_empty() {
            return None;
        }
        Some(self.parents[0])
    }
}

impl PoW for BitcoinBlock {
    fn get_difficulty(&self) -> f64 {
        f64::from_bits(self.difficulty_int)
    }
    fn get_weight(&self) -> f64 {
        f64::from_bits(self.weight_int)
    }

    fn set_difficulty(&mut self, difficulty: f64) {
        self.difficulty_int = difficulty.to_bits();
    }
    fn set_weight(&mut self, weight: f64) {
        self.weight_int = weight.to_bits();
    }
}

impl BitcoinBlock {
    pub(crate) fn generate_genesis_block() -> Self {
        Self {
            creation_time_int: 0.0_f64.to_bits(),
            difficulty_int: 0.0_f64.to_bits(),
            weight_int: 0.0_f64.to_bits(),
            creator: None,
            height: 0,
            parents: Vec::new(),
            size: 0,
        }
    }
}

impl BitcoinBlock {
    pub fn new(
        creation_time: f64,
        creator: Option<usize>,
        height: i32,
        size: u64,
        difficulty: f64,
        weight: f64,
    ) -> Self {
        Self {
            creation_time_int: creation_time.to_bits(),
            creator,
            height,
            parents: Vec::new(),
            size,
            difficulty_int: difficulty.to_bits(),
            weight_int: weight.to_bits(),
        }
    }

    pub fn new_with_parents(
        creation_time: f64,
        creator: Option<usize>,
        height: i32,
        parents: Vec<usize>,
        size: u64,
        difficulty: f64,
        weight: f64,
    ) -> Self {
        let mut block = BitcoinBlock::new(creation_time, creator, height, size, difficulty, weight);
        block.parents = parents;

        block
    }
}
