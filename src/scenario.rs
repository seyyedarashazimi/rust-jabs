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

impl From<&ScenarioData> for Vec<String> {
    fn from(value: &ScenarioData) -> Self {
        vec![
            "# Simulation name: ".to_string(),
            value.name.clone(),
            "Number of nodes: ".to_string(),
            value.num_of_nodes.to_string(),
            "Network type: ".to_string(),
            value.network_type.clone(),
        ]
    }
}
