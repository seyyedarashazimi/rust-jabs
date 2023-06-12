use crate::network::node::nodes::Node;
use crate::network::node::NodeType;
use crate::simulator::event::Event;

// TODO: generalize Node and NodeType
pub trait NetworkStats<T: Event> {
    fn get_latency(&self, from: Node<T>, to: Node<T>) -> f64;
    fn sample_download_bandwidth(&self, node_type: NodeType) -> i64;
    fn sample_upload_bandwidth(&self, node_type: NodeType) -> i64;
}
