pub mod nodes;

use self::nodes::Node;
use crate::network::Network;
use crate::simulator::event::Event;
use crate::simulator::rand::RandomnessEngine;
use crate::simulator::Simulator;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub enum NodeType {}

// TODO: Choose a more meaningful name.
pub trait NetworkInterfaceTrait<T: Event> {
    // type T: Event;
    // type N: Node;
    // type S: NetworkStats<NodeType>;

    fn take_down(&self);
    fn bring_up(&self);
    fn is_network_interface_down(&self) -> bool;
    fn connect_network(
        &self,
        network: Option<Rc<RefCell<Network<T>>>>,
        randomness_engine: Rc<RefCell<RandomnessEngine>>,
    );
}

// TODO: Remove circular referencing.
struct NetworkInterface<T: Event> {
    pub download_bandwidth: i64,
    pub upload_bandwidth: i64,
    pub node: Weak<RefCell<Node<T>>>, // todo
    pub simulator: Rc<RefCell<Simulator<T>>>,
}

impl<T: Event> NetworkInterfaceTrait<T> for NetworkInterface<T> {
    // type T = Event;

    fn take_down(&self) {
        todo!()
    }

    fn bring_up(&self) {
        todo!()
    }

    fn is_network_interface_down(&self) -> bool {
        todo!()
    }

    fn connect_network(
        &self,
        network: Option<Rc<RefCell<Network<T>>>>,
        randomness_engine: Rc<RefCell<RandomnessEngine>>,
    ) {
        todo!()
    }
}

// impl<T: Event> Default for NetworkInterface<T> {
//     fn default() -> Self {
//         todo!()
//     }
// }

impl<T: Event> NetworkInterface<T> {
    fn new(
        simulator: Rc<RefCell<Simulator<T>>>,
        node: Weak<RefCell<Node<T>>>,
        download_bandwidth: i64,
        upload_bandwidth: i64,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            simulator,
            node,
            download_bandwidth,
            upload_bandwidth,
        }))
    }
}
