//! The ECS design for simulating network.
//!
//! Network is created based on
//! [Entity-Component-System](https://en.wikipedia.org/wiki/Entity_component_system)
//! design pattern.
// #![allow(unused)]

pub mod message;
pub mod node;
pub mod resource;
pub mod stats;

use self::node::*;
use self::stats::eighty_six_countries::Country;
use crate::consensus::algorithm::nakamoto_consensus::NakamotoConsensus;
use crate::consensus::blockchain::local_block_tree::LocalBlockTree;
use crate::network::resource::NetworkResource;
use crate::simulator::randomness_engine::RandomnessEngine;
use crate::simulator::Simulator;

/// Temporary constant value standing for the debugger. Will be replaced with a
/// sophisticated debugger later.
pub const LOGGER_MODE: bool = false;
pub const FULL_LOGGER_MODE: bool = false;

//----------State----------//
/// The state of network including the ECS world.
pub struct NetworkState {
    /// The Entity-Component-System(ECS) design. Each node is a `usize`
    /// which is an simple index. The ECS is of type `Network` including
    /// all components(raw data) for each node as vectors of values.
    pub ecs: Network,

    /// Resources not dedicated to a single node and shared between them:
    pub simulator: Simulator,
    pub randomness_engine: RandomnessEngine,
    pub resource: NetworkResource,
}

//----------ECS----------//
/// The Entity-Component-System(ECS) design. Each node is solely denoted by a
/// `usize` number.
pub struct Network {
    // components:
    pub node_name: Vec<Option<NodeName>>,
    pub is_connected: Vec<bool>,
    pub neighbors: Vec<Neighbors>,
    pub uplink: Vec<Uplink>,
    pub downlink: Vec<Downlink>,
    pub country: Vec<Country>,
    pub local_block_tree: Vec<LocalBlockTree>,
    pub already_seen_blocks: Vec<AlreadySeenBlocks>,
    pub consensus_algorithm: Vec<NakamotoConsensus>,
    pub hash_power: Vec<Option<f64>>,
    // entities:
    pub num_of_nodes: usize,
}

impl Network {
    pub fn create_with_size(num_of_nodes: usize) -> Self {
        let node_name: Vec<Option<NodeName>> = vec![None; num_of_nodes];
        let is_connected: Vec<bool> = vec![bool::default(); num_of_nodes];
        let neighbors: Vec<Neighbors> = vec![Neighbors::default(); num_of_nodes];
        let uplink: Vec<Uplink> = vec![Uplink::default(); num_of_nodes];
        let downlink: Vec<Downlink> = vec![Downlink::default(); num_of_nodes];
        let country: Vec<Country> = vec![Country::default(); num_of_nodes];
        let local_block_tree: Vec<LocalBlockTree> = vec![LocalBlockTree::default(); num_of_nodes];
        let nakamoto_consensus: Vec<NakamotoConsensus> =
            vec![NakamotoConsensus::default(); num_of_nodes];
        let already_seen_blocks: Vec<AlreadySeenBlocks> =
            vec![AlreadySeenBlocks::default(); num_of_nodes];
        let hash_power: Vec<Option<f64>> = vec![None; num_of_nodes];
        Self {
            node_name,
            is_connected,
            neighbors,
            num_of_nodes,
            uplink,
            downlink,
            country,
            local_block_tree,
            consensus_algorithm: nakamoto_consensus,
            already_seen_blocks,
            hash_power,
        }
    }
}
