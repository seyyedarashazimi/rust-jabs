// //! Packet including block_index, size, and message_type.
//
// use crate::network::message::MessageType;
//
// #[derive(Debug, Hash, Eq, PartialEq, Clone)]
// pub struct Packet {
//     pub block_index: usize,
//     pub size: u64,
//     message_type: MessageType,
// }
//
// impl Packet {
//     pub fn new(index: usize, size: u64, message_type: MessageType) -> Self {
//         Self {
//             block_index: index,
//             size,
//             message_type,
//         }
//     }
// }
