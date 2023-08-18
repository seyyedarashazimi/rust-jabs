use crate::consensus::algorithm::nakamoto_consensus::NakamotoConsensus;
use crate::consensus::blockchain::local_block_tree::LocalBlockTree;
use crate::network::node::{AlreadySeenBlocks, Downlink, Neighbors, NodeName, Uplink};
use crate::network::stats::eighty_six_countries::Country;

//----------ECS----------//
/// The Entity-Component-System(ECS) design. Each node is solely denoted by a
/// `usize` number.
pub struct BitcoinECS {
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

impl BitcoinECS {
    pub fn create_with_size(num_of_nodes: usize) -> Self {
        let node_name: Vec<Option<NodeName>> = vec![None; num_of_nodes];
        let is_connected: Vec<bool> = vec![bool::default(); num_of_nodes];
        let neighbors: Vec<Neighbors> = vec![Neighbors::default(); num_of_nodes];
        let uplink: Vec<Uplink> = vec![Uplink::default(); num_of_nodes];
        let downlink: Vec<Downlink> = vec![Downlink::default(); num_of_nodes];
        let country: Vec<Country> = vec![Country::default(); num_of_nodes];
        let local_block_tree: Vec<LocalBlockTree> = vec![LocalBlockTree::default(); num_of_nodes];
        let consensus_algorithm: Vec<NakamotoConsensus> =
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
            consensus_algorithm,
            already_seen_blocks,
            hash_power,
        }
    }
}
