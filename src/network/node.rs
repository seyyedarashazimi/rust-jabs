//! The components each node have and included in network.

use std::collections::HashMap;

pub mod connection;
pub mod link;
pub mod neighbors;

//----------Components----------//

/// Name of each node.
#[derive(Default, Debug, Clone)]
pub struct NodeName {
    _name: String,
}

/// Vector of indices denoting the neighbors of a node.
#[derive(Default, Debug, Clone)]
pub struct Neighbors(pub Vec<usize>);

/// Connection status of each node.
#[derive(Default, Debug, Clone)]
pub struct Connected;

// /// The set of all received packet for each node.
// #[derive(Default, Debug, Clone)]
// pub struct HistoryPackets {
//     pub received: HashSet<Packet>,
// }

/// Link data struct to be used in both Uplink and Downlink types.
#[derive(Debug, Clone)]
pub struct Link {
    pub bandwidth: f64,
    pub latest_loaded_time_done: f64,
}

impl Default for Link {
    fn default() -> Self {
        Self {
            bandwidth: 1.0,
            latest_loaded_time_done: 0.0,
        }
    }
}

/// Uplink data of each node.
#[derive(Default, Debug, Clone)]
pub struct Uplink {
    pub link: Link,
}

/// Downlink data of each node.
#[derive(Default, Debug, Clone)]
pub struct Downlink {
    pub link: Link,
}

#[derive(Default, Debug, Clone)]
pub struct AlreadySeenBlocks(pub HashMap<usize, bool>);

/// Type of each node.(WIP)
// #[derive(Default, Debug, Clone)]
pub enum NodeType {
    IsBitcoin,
    IsEthereum,
}
