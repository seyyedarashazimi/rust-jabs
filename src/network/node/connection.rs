use crate::network::Network;

/// Add [`Connected`] component to a node.
///
/// # Arguments
///
/// * `ecs`: Mutable reference to a `Network` type;
/// * `node`: a give node `usize`.
#[allow(unused)]
fn connect_node(ecs: &mut Network, node: usize) -> Result<(), String> {
    if let Some(status) = ecs.is_connected.get_mut(node) {
        *status = true;
        return Ok(());
    }
    Err(format!("Index out of bounds for connect_node: {}", node))
}

/// Remove [`Connected`] component from a node.
///
/// # Arguments
///
/// * `ecs`: Mutable reference to a `Network` type;
/// * `node`: a give node `usize`.
#[allow(unused)]
fn disconnect_node(ecs: &mut Network, node: usize) -> Result<(), String> {
    if let Some(status) = ecs.is_connected.get_mut(node) {
        *status = false;
        return Ok(());
    }
    Err(format!("Index out of bounds for disconnect_node: {}", node))
}

/// Check if the node is online.
///
/// # Arguments
///
/// * `ecs`: Immutable reference to [`Network`];
/// * `node`: `usize` index denoting the node;
///
/// # Return
///
/// true if node exists and is connected.
///
/// # Examples
///
/// ```
/// use rust_jabs::network::Network;
/// use rust_jabs::network::node::connection::node_is_connected;
/// let mut network = Network::create_with_size(1);
/// network.is_connected[0] = true;
///
/// assert_eq!(node_is_connected(&network, 0), true);
/// assert_eq!(node_is_connected(&network, 1), false);
/// assert_eq!(node_is_connected(&network, 2), false);
/// ```
pub fn node_is_connected(ecs: &Network, node: usize) -> bool {
    ecs.is_connected.get(node).map_or(false, |&status| status)
}

pub fn set_all_nodes_connected(ecs: &mut Network, size: usize) {
    assert_eq!(ecs.is_connected.len(), size);
    ecs.is_connected.iter_mut().for_each(|x| *x = true);
}
