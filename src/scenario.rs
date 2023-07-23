pub mod bitcoin_global_network_scenario;

pub struct ScenarioData {
    pub name: String,
    pub num_of_nodes: usize,
    pub network_type: String,
}

impl ScenarioData {
    pub fn new(name: String, num_of_nodes: usize, network_type: String) -> Self {
        Self {
            name,
            num_of_nodes,
            network_type,
        }
    }
}
