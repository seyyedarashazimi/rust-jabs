//! The components each node have and included in network.
use crate::network::packet::Packet;
use specs::prelude::{Component, Entity, NullStorage, VecStorage};
use specs_derive::Component;
use std::collections::HashSet;

//----------Components----------//

/// Name of each node.
#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct NodeName {
    name: String,
}

/// Type of each node.(WIP)
#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct NodeType;

/// Vector of indices denoting the neighbors of a node.
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

/// Connection status of each node.
#[derive(Component, Default, Debug)]
#[storage(NullStorage)]
pub struct Connected;

/// The set of all received packet for each node.
#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct HistoryPackets {
    pub received: HashSet<Packet>,
}

/// Uplink data of each node.
#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct Uplink {
    pub upload_bandwidth: u64,
    pub latest_uploaded_time_done: f64,
}
