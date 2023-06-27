//! Packet including message, size, sender, and receiver.

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Packet {
    pub msg: String,
    pub size: u64,
    // pub from: usize,
    // pub to: usize,
}
