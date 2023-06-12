use crate::network::node::nodes::Node;
use crate::simulator::event::Event;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

trait P2PConnectionsTrait<T: Event> {
    fn get_node(&self) -> Weak<RefCell<Node<T>>>;
    fn set_node(&mut self, node: Weak<RefCell<Node<T>>>);
    fn get_neighbors(&self) -> Vec<Weak<RefCell<Node<T>>>>;
    fn connect_to_network(&mut self);
    fn request_connection(node: Weak<RefCell<Node<T>>>) -> bool;
}

pub struct P2PConnections<T: Event> {
    node: Weak<RefCell<Node<T>>>,
    neighbors: Vec<Weak<RefCell<Node<T>>>>,
}

impl<T: Event> P2PConnectionsTrait<T> for P2PConnections<T> {
    fn get_node(&self) -> Weak<RefCell<Node<T>>> {
        self.node.clone()
    }

    fn set_node(&mut self, node: Weak<RefCell<Node<T>>>) {
        self.node = node;
    }

    fn get_neighbors(&self) -> Vec<Weak<RefCell<Node<T>>>> {
        self.neighbors.clone()
    }

    fn connect_to_network(&mut self) {
        todo!()
    }

    fn request_connection(node: Weak<RefCell<Node<T>>>) -> bool {
        todo!()
    }
}
