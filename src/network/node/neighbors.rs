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

        // Add the node to the list of its neighbors
        for &n in &node_neighbors {
            ecs.neighbors[n].list.push(node);
        }

        // Insert the Neighbors component to the node
        *ecs.neighbors
            .get_mut(node)
            .expect("Failed to insert neighbors") = Neighbors {
            list: node_neighbors,
        }
    }
}

pub fn is_neighbors_bidirectional(neighbors: &[Neighbors]) -> bool {
    neighbors
        .iter()
        .enumerate()
        .zip(neighbors.iter().enumerate())
        .all(|((node1, neighbors1), (node2, neighbors2))| {
            neighbors1.list.contains(&node2) == neighbors2.list.contains(&node1)
        })
}

#[allow(unused)]
fn generated_neighbors_min_max(neighbors: &[Neighbors]) -> (usize, usize) {
    let min_neighbor_size = neighbors.iter().map(|n| n.list.len()).min().unwrap_or(0);
    let max_neighbor_size = neighbors.iter().map(|n| n.list.len()).max().unwrap_or(0);
    (min_neighbor_size, max_neighbor_size)
}
