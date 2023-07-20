// use crate::consensus::blockchain::local_block::LocalBlock;
// use crate::ledger_data::block::Block;
// use std::collections::{HashMap, HashSet};
//
// #[derive(Default, Clone)]
// pub struct LocalBlockDAG {
//     local_block_dag: HashMap<usize, LocalBlock>,
//     genesis_block_index: usize,
// }
//
// impl LocalBlockDAG {
//     pub fn new(genesis_block_index: usize) -> Self {
//         let mut genesis_local_block = LocalBlock::new(genesis_block_index);
//         genesis_local_block.is_connected_to_genesis = true;
//         let mut local_block_dag = HashMap::new();
//         local_block_dag.insert(genesis_block_index, genesis_local_block);
//         Self {
//             local_block_dag,
//             genesis_block_index,
//         }
//     }
//
//     pub fn add(&mut self, blocks: &[Block], block_index: usize) {
//         if !self.local_block_dag.contains_key(&block_index) {
//             // create a new local block for the received block index.
//             let mut local_block = LocalBlock::new(block_index);
//
//             // find all children of previously received blocks and add them to
//             // the children of this local block.
//             let seen_children = self
//                 .local_block_dag
//                 .keys()
//                 .filter(|&&block_index_in_local_dag| {
//                     blocks[block_index_in_local_dag]
//                         .parents
//                         .contains(&block_index)
//                 });
//             local_block.children_index.extend(seen_children);
//
//             // find all parents of this block from received blocks:
//             for parent in &blocks[block_index].parents {
//                 // todo: benchmark here (untested):
//                 // version 1.
//                 // if self.local_block_dag.contains_key(parent) {
//                 //     let local_parent = self.local_block_dag.get_mut(parent).unwrap();
//                 //
//                 // version 2.
//                 if let Some(local_parent) = self.local_block_dag.get_mut(parent) {
//                     // end of to do
//                     local_parent.children_index.insert(block_index);
//                     if local_parent.is_connected_to_genesis {
//                         local_block.is_connected_to_genesis = true;
//                         if let Some(successors) = self.get_all_successors(block_index) {
//                             for successor in successors {
//                                 self.local_block_dag
//                                     .get_mut(&successor)
//                                     .unwrap()
//                                     .is_connected_to_genesis = true;
//                             }
//                         }
//                     }
//                 }
//             }
//
//             self.local_block_dag.insert(block_index, local_block);
//         }
//     }
//
//     pub fn get_all_successors(&self, block_index: usize) -> Option<HashSet<usize>> {
//         // if block is not seen, or has no children, returns None:
//         if self.local_block_dag.contains_key(&block_index)
//             || self
//                 .local_block_dag
//                 .get(&block_index)
//                 .unwrap()
//                 .children_index
//                 .is_empty()
//         {
//             None
//         } else {
//             let mut in_current_height: HashSet<usize> = HashSet::new();
//             let mut in_next_height: HashSet<usize> = HashSet::new();
//             let mut all_successors: HashSet<usize> = HashSet::new();
//             in_current_height.insert(block_index);
//             'l: loop {
//                 for successor in &in_current_height {
//                     in_next_height.extend(
//                         self.local_block_dag
//                             .get(successor)
//                             .unwrap()
//                             .children_index
//                             .iter(),
//                     );
//                 }
//                 in_current_height.clear();
//                 in_current_height.extend(in_next_height.iter());
//                 all_successors.extend(in_current_height.iter());
//                 in_next_height.clear();
//
//                 if in_current_height.is_empty() {
//                     break 'l;
//                 }
//             }
//             Some(all_successors)
//         }
//     }
// }
