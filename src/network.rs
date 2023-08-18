//! The ECS design for simulating network.
//!
//! Network is created based on
//! [Entity-Component-System](https://en.wikipedia.org/wiki/Entity_component_system)
//! design pattern.
// #![allow(unused)]

pub mod bitcoin_network;
pub mod ecs;
pub mod message;
pub mod node;
pub mod resource;
pub mod stats;

use crate::log::NetworkLogHandler;
use crate::simulator::event::block_mining_process::BlockMiningProcess;
use crate::simulator::event::generate_block_event::GenerateBlockWithoutTxEvent;
use crate::simulator::event::receive_event::ReceiveEvent;
use crate::simulator::event::send_event::SendEvent;
use crate::simulator::randomness_engine::RandomnessEngine;
use crate::simulator::Simulator;

pub trait Network: NetworkLogHandler {
    // used in events:
    fn generate_new_block_without_tx(
        &mut self,
        simulator: &mut Simulator,
        rand: &mut RandomnessEngine,
        generate_event: &GenerateBlockWithoutTxEvent,
    );
    fn receive(&mut self, simulator: &mut Simulator, receive_event: &ReceiveEvent);
    fn send(
        &mut self,
        simulator: &mut Simulator,
        rand: &mut RandomnessEngine,
        send_event: &SendEvent,
    );
    fn block_mining(
        &mut self,
        simulator: &mut Simulator,
        rand: &mut RandomnessEngine,
        mining_event: &BlockMiningProcess,
    );
}

//----------State----------//
/// The state of network including the ECS world.
pub struct NetworkState {
    /// The Entity-Component-System(ECS) design. Each node is a `usize`
    /// which is an simple index. The ECS is of type `Network` including
    /// all components(raw data) for each node as vectors of values. It
    /// also include shared `Resource` as well among all nodes.
    pub network: Box<dyn Network>,

    pub simulator: Simulator,
    pub rand: RandomnessEngine,
}

pub struct EthereumNetwork;
