//! The ECS design for simulating network.
//!
//! Network is created based on
//! [Entity-Component-System](https://en.wikipedia.org/wiki/Entity_component_system)
//! design pattern.
// #![allow(unused)]

pub mod node;
pub mod packet;
pub mod stats;

use self::node::*;
use self::packet::Packet;
use self::stats::eighty_six_countries::Country::{self, *};
use crate::simulator::randomness_engine::RandomnessEngine;
use crate::simulator::Simulator;

/// Temporary constant value standing for the debugger. Will be replaced with a
/// sophisticated debugger later.
pub const LOGGER_MODE: bool = false;

//----------State----------//
/// The state of network including the ECS `World`.
pub struct NetworkState {
    /// The Entity-Component-System(ECS) design. Each node is a `usize`
    /// which is an index + generation. The ECS is of type `Network` including
    /// all components(raw data) for each node as vectors of values.
    pub ecs: Network,
    pub simulator: Simulator,
    pub randomness_engine: RandomnessEngine,
    pub packets: Vec<Packet>,
}

//----------ECS----------//
/// The Entity-Component-System(ECS) design. Each node is solely denoted by a
/// `usize` number.
pub struct Network {
    // components:
    pub node_name: Vec<Option<NodeName>>,
    pub is_connected: Vec<bool>,
    pub neighbors: Vec<Neighbors>,
    pub history: Vec<HistoryPackets>,
    pub uplink: Vec<Uplink>,
    pub downlink: Vec<Downlink>,
    pub country: Vec<Country>,
    // entities:
    pub num_of_nodes: usize,
    // node: Vec<usize>,
}

impl Network {
    pub fn create_with_size(num_of_nodes: usize) -> Self {
        let node_name: Vec<Option<NodeName>> = vec![Some(NodeName::default()); num_of_nodes];
        let is_connected: Vec<bool> = vec![false; num_of_nodes];
        let neighbors: Vec<Neighbors> = vec![Neighbors::default(); num_of_nodes];
        let history: Vec<HistoryPackets> = vec![HistoryPackets::default(); num_of_nodes];
        let uplink: Vec<Uplink> = vec![Uplink::default(); num_of_nodes];
        let downlink: Vec<Downlink> = vec![Downlink::default(); num_of_nodes];
        let country: Vec<Country> = vec![Albania; num_of_nodes];
        Self {
            node_name,
            is_connected,
            neighbors,
            history,
            num_of_nodes,
            uplink,
            downlink,
            country,
        }
    }
}

//----------Functions----------//
pub fn simulate_propagation(
    ecs: &mut Network,
    simulator: &mut Simulator,
    rand: &mut RandomnessEngine,
    packets: &[Packet],
) {
    while simulator.is_there_more_events() {
        simulator.execute_next_event(ecs, rand, packets);
    }
}
