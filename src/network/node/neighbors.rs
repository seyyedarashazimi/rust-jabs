use super::Neighbors;
use crate::network::Network;
use crate::simulator::randomness_engine::RandomnessEngine;

pub fn assign_random_neighbors(
    ecs: &mut Network,
    rand: &mut RandomnessEngine,
    min_neighbors: usize,
) {
    for node in 0..ecs.num_of_nodes {
        let num_neighbors = min_neighbors;
        let other_nodes: Vec<usize> = (0..ecs.num_of_nodes)
            .filter(|&neighbors| neighbors != node) // remove itself from neighbors
            .collect();

        // Generate a random subset of other nodes to be neighbors
        let node_neighbors = rand.sample_nodes(&other_nodes, num_neighbors);

        for &n in &node_neighbors {
            // Add neighbor to the node list (avoid duplicate)
            if !ecs.neighbors[node].list.contains(&n) {
                ecs.neighbors[node].list.push(n);
            }
            // Add the node to the list of its neighbors (bidi, avoid duplicate)
            if !ecs.neighbors[n].list.contains(&node) {
                ecs.neighbors[n].list.push(node);
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
            if node1 != node2
                && (neighbors1.list.contains(&node2) != neighbors2.list.contains(&node1))
            {
                return false;
            }
        }
    }
    true
}

#[allow(unused)]
fn generated_neighbors_min_max(neighbors: &[Neighbors]) -> (usize, usize) {
    let min_neighbor_size = neighbors.iter().map(|n| n.list.len()).min().unwrap_or(0);
    let max_neighbor_size = neighbors.iter().map(|n| n.list.len()).max().unwrap_or(0);
    (min_neighbor_size, max_neighbor_size)
}
