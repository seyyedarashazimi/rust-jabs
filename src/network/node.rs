use crate::network::packet::Packet;
use std::collections::HashSet;

//----------Components----------//
#[derive(Default, Debug, Clone)]
pub struct NodeName {
    name: String,
}

#[derive(Default, Debug, Clone)]
pub struct Neighbors {
    pub neighbors: Vec<usize>,
}

#[derive(Default, Debug, Clone)]
pub struct Bandwidth {
    pub download: i64,
    pub upload: i64,
}

#[derive(Default, Debug, Clone)]
pub struct NodeType;

#[derive(Default, Debug, Clone)]
pub struct Is26;

#[derive(Default, Debug, Clone)]
pub struct Connected;

#[derive(Default, Debug, Clone)]
pub struct HistoryPackets {
    pub received: HashSet<Packet>,
}

#[derive(Default, Debug, Clone)]
pub struct Uplink {
    pub upload_bandwidth: u64,
    pub latest_uploaded_time_done: f64,
}
