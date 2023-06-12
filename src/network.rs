mod node;
mod p2p;
mod packet;
mod stats;

use self::node::nodes::Node;
use self::node::NodeType;
use crate::network::stats::NetworkStats;
use crate::simulator::event::Event;
use crate::simulator::rand::RandomnessEngine;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// TODO: Choose a more meaningful name.
// TODO: Break network trait into multiple traits if needed.
// TODO: Add default error returns for undefined methods.
pub trait NetworkTrait: NetworkStats<Self::T> {
    type T: Event;

    fn get_random_node() -> Node<Self::T>;
    fn get_all_nodes() -> Vec<Node<Self::T>>;
    fn get_node(index: i32) -> Node<Self::T>;

    fn add_note(node: Node<Self::T>, node_type: NodeType);

    fn get_random() -> Rc<RefCell<RandomnessEngine>>;

    // fn get_latency(from: Self::N, to: Self::N) -> f64;
    // fn sample_download_bandwidth(node_type: NodeType) -> i64;
    // fn sample_upload_bandwidth(node_type: NodeType) -> i64;
}

// TODO: Remove circular referencing.
pub struct Network<T: Event> {
    nodes: Vec<Rc<RefCell<Node<T>>>>,
    randomness_engine: Rc<RefCell<RandomnessEngine>>, // todo
    network_stats: Rc<RefCell<dyn NetworkStats<T>>>,  // todo
    node_types: HashMap<Node<T>, NodeType>,
}

impl<T: Event> Network<T> {
    pub fn new(
        randomness_engine: Rc<RefCell<RandomnessEngine>>,
        network_stats: Rc<RefCell<dyn NetworkStats<T>>>,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            randomness_engine,
            network_stats,
            nodes: Vec::new(),
            node_types: HashMap::new(),
        }))
    }
}
