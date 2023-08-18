/// Connect a node.
///
/// # Arguments
///
/// * `is_connected`: Mutable reference to Boolean vector of connection status;
/// * `node`: a give node `usize`.
#[allow(unused)]
fn connect_node(is_connected: &mut Vec<bool>, node: usize) -> Result<(), String> {
    if let Some(status) = is_connected.get_mut(node) {
        *status = true;
        return Ok(());
    }
    Err(format!("Index out of bounds for connect_node: {}", node))
}

/// Disconnect a node.
///
/// # Arguments
///
/// * `is_connected`: Mutable reference to Boolean vector of connection status;
/// * `node`: a give node `usize`.
#[allow(unused)]
fn disconnect_node(is_connected: &mut Vec<bool>, node: usize) -> Result<(), String> {
    if let Some(status) = is_connected.get_mut(node) {
        *status = false;
        return Ok(());
    }
    Err(format!("Index out of bounds for disconnect_node: {}", node))
}

/// Check if a node is online.
///
/// # Arguments
///
/// * `is_connected`: Immutable reference to Boolean vector of connection status;
/// * `node`: `usize` index denoting the node;
///
/// # Return
///
/// true if node exists and is connected.
///
pub fn node_is_connected(is_connected: &Vec<bool>, node: usize) -> bool {
    is_connected.get(node).map_or(false, |&status| status)
}

pub fn set_all_nodes_connected(is_connected: &mut Vec<bool>, size: usize) {
    assert_eq!(is_connected.len(), size);
    is_connected.iter_mut().for_each(|x| *x = true);
}
