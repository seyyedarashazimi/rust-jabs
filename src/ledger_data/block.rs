use std::cmp::Ordering;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Block {
    creation_time_int: u64, // to directly have Hash and Eq traits.
    pub creator: Option<usize>,
    pub height: i32,
    pub parents: Vec<usize>,
    pub size: u64,
    pub weight_int: u64, // to directly have Hash and Eq traits.
}

impl PartialOrd for Block {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Compare the `height` of two blocks.
impl Ord for Block {
    fn cmp(&self, other: &Self) -> Ordering {
        self.height.cmp(&other.height)
    }
}

impl Block {
    pub fn new(
        creation_time: f64,
        creator: Option<usize>,
        height: i32,
        size: u64,
        weight: f64,
    ) -> Self {
        Self {
            creation_time_int: creation_time.to_bits(),
            creator,
            height,
            parents: Vec::new(),
            size,
            weight_int: weight.to_bits(),
        }
    }

    pub fn new_with_parents(
        creation_time: f64,
        creator: Option<usize>,
        height: i32,
        parents: Vec<usize>,
        size: u64,
        weight: f64,
    ) -> Self {
        Self {
            creation_time_int: creation_time.to_bits(),
            creator,
            height,
            parents,
            size,
            weight_int: weight.to_bits(),
        }
    }

    pub fn generate_genesis_block() -> Self {
        Self {
            creation_time_int: 0.0_f64.to_bits(),
            creator: None,
            height: 0,
            parents: Vec::new(),
            size: 0,
            weight_int: 0.0_f64.to_bits(),
        }
    }

    /// returns the float value of creation time when needed.
    pub fn get_creation_time(&self) -> f64 {
        f64::from_bits(self.creation_time_int)
    }

    pub fn get_weight(&self) -> f64 {
        f64::from_bits(self.weight_int)
    }

    pub fn get_single_parent(&self) -> Option<usize> {
        if self.parents.is_empty() {
            return None;
        }
        Some(self.parents[0])
    }
}
