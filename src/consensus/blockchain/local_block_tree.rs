use crate::consensus::blockchain::local_block::LocalBlock;
use crate::ledger_data::block::Block;
use crate::ledger_data::single_parent::SingleParent;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
pub struct LocalBlockTree {
    pub local_block_dag: HashMap<usize, LocalBlock>,
}

impl Default for LocalBlockTree {
    fn default() -> Self {
        LocalBlockTree::new()
    }
}

impl LocalBlockTree {
    pub fn new() -> Self {
        let genesis_block_index: usize = 0;
        let mut genesis_local_block = LocalBlock::new(genesis_block_index);
        genesis_local_block.is_connected_to_genesis = true;
        let mut local_block_dag = HashMap::new();
        local_block_dag.insert(genesis_block_index, genesis_local_block);
        Self { local_block_dag }
    }

    pub fn add<B>(&mut self, block_index: usize, blocks: &[B])
    where
        B: Block,
    {
        if !self.contains(block_index) {
            // create a new local block for the received block index.
            let mut local_block = LocalBlock::new(block_index);

            // find all children of previously received blocks and add them to
            // the children of this local block.
            let seen_children = self
                .local_block_dag
                .keys()
                .filter(|&&block_index_in_local_dag| {
                    blocks[block_index_in_local_dag]
                        .get_parents()
                        .contains(&block_index)
                });
            local_block.children_index.extend(seen_children);

            // find all parents of this block from received blocks:
            for parent in blocks[block_index].get_parents() {
                if let Some(local_parent) = self.local_block_dag.get_mut(parent) {
                    local_parent.children_index.insert(block_index);
                    if local_parent.is_connected_to_genesis {
                        local_block.is_connected_to_genesis = true;
                        if let Some(successors) = self.get_all_successors(block_index) {
                            for successor in successors {
                                self.local_block_dag
                                    .get_mut(&successor)
                                    .unwrap()
                                    .is_connected_to_genesis = true;
                            }
                        }
                    }
                }
            }

            self.local_block_dag.insert(block_index, local_block);
        }
    }

    pub fn get_all_successors(&self, block_index: usize) -> Option<HashSet<usize>> {
        // if block is not seen, or has no children, returns None:
        if !self.local_block_dag.contains_key(&block_index)
            || self
                .local_block_dag
                .get(&block_index)
                .unwrap()
                .children_index
                .is_empty()
        {
            None
        } else {
            let mut in_current_height: HashSet<usize> = HashSet::new();
            let mut in_next_height: HashSet<usize> = HashSet::new();
            let mut all_successors: HashSet<usize> = HashSet::new();
            in_current_height.insert(block_index);
            'l: loop {
                for successor in &in_current_height {
                    in_next_height.extend(
                        self.local_block_dag
                            .get(successor)
                            .unwrap()
                            .children_index
                            .iter(),
                    );
                }
                in_current_height.clear();
                in_current_height.extend(in_next_height.iter());
                all_successors.extend(in_current_height.iter());
                in_next_height.clear();

                if in_current_height.is_empty() {
                    break 'l;
                }
            }
            Some(all_successors)
        }
    }

    /// Returns the highest common ancestor of the provided two blocks.
    ///
    /// # Arguments
    ///
    /// * `block_a`: First block index
    /// * `block_b`: Second block
    /// * `blocks`:
    ///
    /// returns: Option<usize>
    /// The `Option` ancestor with largest height value.
    ///
    pub fn get_common_ancestor<B>(&self, block_a: usize, block_b: usize, blocks: &[B]) -> usize
    where
        B: Block + SingleParent,
    {
        let mut block_x = block_a;
        let mut block_y = block_b;

        let genesis_index = 0;

        if blocks[block_a].get_height() > blocks[block_b].get_height() {
            block_x = self
                .get_single_ancestor_of_height(block_a, blocks[block_b].get_height(), blocks)
                .unwrap_or(genesis_index);
        } else if blocks[block_a].get_height() < blocks[block_b].get_height() {
            block_y = self
                .get_single_ancestor_of_height(block_b, blocks[block_a].get_height(), blocks)
                .unwrap_or(genesis_index);
        }

        loop {
            if blocks[block_x] == blocks[block_y] {
                return block_x;
            } else {
                block_x = blocks[block_x].get_single_parent().unwrap_or(genesis_index);
                block_y = blocks[block_y].get_single_parent().unwrap_or(genesis_index);
            }
        }
    }

    /// Returns the ancestor of the block with at certain height.
    /// Only use this method when all ancestors of the block is received.
    ///
    /// # Arguments
    ///
    /// * `block_index`: the block index
    /// * `height`: the targeted height that the returning ancestor is expected to have
    /// * `blocks`: immutable reference to blocks
    ///
    /// returns: the `Option` ancestor index with height equal to the input height.
    ///
    pub fn get_single_ancestor_of_height<B>(
        &self,
        block_index: usize,
        height: i32,
        blocks: &[B],
    ) -> Option<usize>
    where
        B: Block + SingleParent,
    {
        //! instead of accessing blocks directly from blocks, we accurately
        //! search for the ancestors and the block among local_block to
        //! to be sure each ancestor is received previously. A faster but
        //! not accurate way is to use blocks directly instead of local_block.

        if !self.contains(block_index) {
            return None;
        } else {
            let block = &blocks[block_index];
            match block.get_height().cmp(&height) {
                Ordering::Equal => return Some(block_index),
                Ordering::Less => return None,
                Ordering::Greater => {
                    if let Some(mut ancestor_index) = block.get_single_parent() {
                        loop {
                            if !self.contains(ancestor_index) {
                                return None;
                            } else if blocks[ancestor_index].get_height() == height {
                                return Some(ancestor_index);
                            } else {
                                match blocks[ancestor_index].get_single_parent() {
                                    Some(parent) => ancestor_index = parent,
                                    None => return None,
                                }
                                // ancestor_index = blocks[ancestor_index].get_single_parent();
                            }
                        }
                    } else {
                        return None;
                    }
                }
            }
        }
    }

    /// Returns all ancestors of the input block in a HashSet
    ///
    /// # Arguments
    ///
    /// * `block_index`: the block index which its ancestors are requested
    /// * `blocks`: immutable reference to blocks
    ///
    /// returns: all ancestors in the local block tree
    ///
    pub fn get_all_single_ancestors<B>(&self, block_index: usize, blocks: &[B]) -> HashSet<usize>
    where
        B: Block + SingleParent,
    {
        if !self.contains(block_index) {
            return HashSet::new();
        } else if let Some(mut ancestor_block_index) = blocks[block_index].get_single_parent() {
            let mut ancestors_index = HashSet::<usize>::new();
            loop {
                if self.contains(ancestor_block_index) {
                    ancestors_index.insert(ancestor_block_index);
                } else {
                    return ancestors_index;
                }
                // ancestor_block_index = blocks[ancestor_block_index].get_single_parent();
                match blocks[ancestor_block_index].get_single_parent() {
                    Some(parent) => ancestor_block_index = parent,
                    None => return ancestors_index,
                }
            }
        } else {
            return HashSet::new();
        }
    }

    /// Checks if the `block_index` is among the keys in the
    /// `self.local_block_dag` HashMap.
    pub fn contains(&self, block_index: usize) -> bool {
        self.local_block_dag.contains_key(&block_index)
    }
}

pub fn assign_initial_local_block_trees(
    local_block_tree: &mut [LocalBlockTree],
    num_of_nodes: usize,
) {
    assert_eq!(
        local_block_tree.len(),
        num_of_nodes,
        "Error: please initialize local_block_tree before calling assign_initial_local_block_trees"
    );

    for local_block_tree_initial in local_block_tree {
        *local_block_tree_initial = LocalBlockTree::new();
    }
}
