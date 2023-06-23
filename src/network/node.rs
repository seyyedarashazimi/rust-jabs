use crate::network::packet::Packet;
use specs::prelude::*;
use specs_derive::*;
use std::collections::HashSet;

//----------Components----------//
#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct NodeName {
    name: String,
}

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct Neighbors {
    pub neighbors: Vec<Entity>,
}

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct Bandwidth {
    pub download: i64,
    pub upload: i64,
}

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct NodeType;

#[derive(Component, Default, Debug)]
#[storage(NullStorage)]
pub struct Is26;

#[derive(Component, Default, Debug)]
#[storage(NullStorage)]
pub struct Connected;

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct HistoryPackets {
    pub received: HashSet<Packet>,
}

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct Uplink {
    pub upload_bandwidth: u64,
    pub latest_uploaded_time_done: f64,
}
