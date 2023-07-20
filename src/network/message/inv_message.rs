// use crate::ledger_data::block_factory::{BITCOIN_INV_SIZE, INV_MESSAGE_OVERHEAD};
//
// pub struct InvMessage {
//     block_index: usize,
//     size: u64,
// }
//
// impl InvMessage {
//     pub fn new(block_index: usize, hash_size: u64) -> Self {
//         Self {
//             block_index,
//             size: hash_size + INV_MESSAGE_OVERHEAD,
//         }
//     }
//
//     pub fn new_bitcoin(block_index: usize) -> Self {
//         InvMessage::new(block_index, BITCOIN_INV_SIZE)
//     }
// }
