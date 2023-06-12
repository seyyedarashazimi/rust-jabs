use crate::network::node::{NetworkInterface, NodeType};
use crate::network::p2p::P2PConnections;
use crate::network::packet::Packet;
use crate::network::stats::NetworkStats;
use crate::network::Network;
use crate::simulator::event::Event;
use crate::simulator::Simulator;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

// TODO: Choose a more meaningful name.
pub trait NodeTrait {
    fn process_incoming_packet(&self, packet: Packet);
    fn generate_new_transaction(&self);
    fn crash(&self);
    fn restore(&self);
    // fn broadcast_message(&self, message: Message);
    fn broadcast_message(&self, message: &str);
    // fn get_node_network_interface_data(&self) -> NetworkInterfaceData<T, N, S>;
}

// TODO: Remove circular referencing.
pub struct Node<T: Event> {
    node_id: i32,
    network_interface: Option<Rc<RefCell<NetworkInterface<T>>>>, // todo
    p2p_connections: Rc<RefCell<P2PConnections<T>>>,             // todo
    simulator: Rc<RefCell<Simulator<T>>>,
    network: Weak<RefCell<Network<T>>>, // todo
}

// TODO: add default error returns for undefined methods.
impl<T: Event> NodeTrait for Node<T> {
    fn process_incoming_packet(&self, packet: Packet) {
        todo!()
    }

    fn generate_new_transaction(&self) {
        todo!()
    }

    fn crash(&self) {
        todo!()
    }

    fn restore(&self) {
        todo!()
    }

    fn broadcast_message(&self, message: &str) {
        todo!()
    }
}

impl<T: Event> NetworkStats<T> for Node<T> {
    fn get_latency(&self, from: Node<T>, to: Node<T>) -> f64 {
        todo!()
    }

    fn sample_download_bandwidth(&self, node_type: NodeType) -> i64 {
        todo!()
    }

    fn sample_upload_bandwidth(&self, node_type: NodeType) -> i64 {
        todo!()
    }
}

impl<T: Event> Node<T> {
    pub fn new(
        simulator: Rc<RefCell<Simulator<T>>>,
        network: Weak<RefCell<Network<T>>>,
        node_id: i32,
        download_bandwidth: i64,
        upload_bandwidth: i64,
        p2p_connections: Rc<RefCell<P2PConnections<T>>>,
    ) -> Rc<RefCell<Self>> {
        let node = Rc::new(RefCell::new(Self {
            node_id,
            network_interface: None,
            p2p_connections,
            simulator: simulator.clone(),
            network,
        }));

        let interface = NetworkInterface::new(
            simulator, // hint: clone it if you add another strong ref for p2p
            Rc::downgrade(&node),
            download_bandwidth,
            upload_bandwidth,
        );
        node.borrow_mut().network_interface = Some(interface);
        node
    }
}
