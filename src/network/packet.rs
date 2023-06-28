//! Packet including message, size, sender, and receiver.

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Packet {
    pub msg: String,
    pub size: u64,
}

pub fn generate_packet_default_message(size: u64, ctr: usize) -> Packet {
    Packet {
        msg: format!("packet info: size:{:?}, ctr:{:?}", size, ctr),
        size,
    }
}
