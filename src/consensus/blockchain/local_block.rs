use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct LocalBlock {
    pub block_index: usize,
    pub children_index: HashSet<usize>,
    pub is_connected_to_genesis: bool,
}

impl LocalBlock {
    pub fn new(block_index: usize) -> LocalBlock {
        Self {
            block_index,
            children_index: HashSet::new(),
            is_connected_to_genesis: false,
        }
    }
}
