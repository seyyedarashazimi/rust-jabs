use super::Neighbors;
use crate::simulator::randomness_engine::RandomnessEngine;

pub fn assign_random_neighbors(
    neighbors: &mut Vec<Neighbors>,
    rand: &mut RandomnessEngine,
    min_neighbors: usize,
    num_of_nodes: usize,
) {
    for node in 0..num_of_nodes {
        let num_neighbors = min_neighbors;
        let other_nodes: Vec<usize> = (0..num_of_nodes)
            .filter(|&nb| nb != node) // remove itself from neighbors
            .collect();

        // Generate a random subset of other nodes to be neighbors
        let node_neighbors = rand.sample_nodes(&other_nodes, num_neighbors);

        for &n in &node_neighbors {
            // Add neighbor to the node list (avoid duplicate)
            if !neighbors[node].0.contains(&n) {
                neighbors[node].0.push(n);
            }
            // Add the node to the list of its neighbors (bidi, avoid duplicate)
            if !neighbors[n].0.contains(&node) {
                neighbors[n].0.push(node);
            }
        }
    }
}

/// Checks if all neighbors are bidirectional, i.e. checks whether each node is
/// a neighbor for each of its neighbors or not.
///
pub fn is_neighbors_bidirectional(neighbors: &[Neighbors]) -> bool {
    for (node1, neighbors1) in neighbors.iter().enumerate() {
        for (node2, neighbors2) in neighbors.iter().enumerate() {
            if node1 != node2 && (neighbors1.0.contains(&node2) != neighbors2.0.contains(&node1)) {
                return false;
            }
        }
    }
    true
}

#[allow(unused)]
fn generated_neighbors_min_max(neighbors: &[Neighbors]) -> (usize, usize) {
    let min_neighbor_size = neighbors.iter().map(|n| n.0.len()).min().unwrap_or(0);
    let max_neighbor_size = neighbors.iter().map(|n| n.0.len()).max().unwrap_or(0);
    (min_neighbor_size, max_neighbor_size)
}
