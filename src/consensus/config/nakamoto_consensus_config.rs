#[derive(Default, Debug, Clone)]
pub struct NakamotoConsensusConfig {
    pub average_block_mining_interval: f64,
    pub confirmation_depth: i32,
    pub genesis_block_index: usize,
}

impl NakamotoConsensusConfig {
    pub fn new(
        average_block_mining_interval: f64,
        confirmation_depth: i32,
        genesis_block_index: usize,
    ) -> Self {
        Self {
            average_block_mining_interval,
            confirmation_depth,
            genesis_block_index,
        }
    }
}
