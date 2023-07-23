use crate::log::{CSVLogger, EventLoggerInfo};
use crate::network::resource::NetworkResource;
use crate::network::Network;
use crate::scenario::ScenarioData;

#[derive(Default)]
pub struct BlockConfirmationLogger;

impl CSVLogger for BlockConfirmationLogger {
    fn csv_starting_comment(&self, scenario: &ScenarioData) -> Vec<String> {
        vec![
            "# Simulation name: ".to_string(),
            scenario.name.clone(),
            "Number of nodes: ".to_string(),
            scenario.num_of_nodes.to_string(),
            "Network type: ".to_string(),
            scenario.network_type.clone(),
        ]
    }

    fn csv_output_condition_before_event(&self, _: &EventLoggerInfo) -> bool {
        false
    }

    fn csv_output_condition_after_event(&self, info: &EventLoggerInfo) -> bool {
        matches!(info, EventLoggerInfo::IsBlockConfirmationEvent(..))
    }

    fn csv_output_condition_final_per_node(&self) -> bool {
        false
    }

    fn csv_header_output(&self) -> Vec<String> {
        let header_str = vec![
            "Time",
            "NodeIndex",
            "BlockHeight",
            "BlockSize",
            "BlockCreationTime",
            "BlockCreator",
        ];
        header_str.into_iter().map(String::from).collect()
    }

    fn csv_event_output(
        &self,
        info: &EventLoggerInfo,
        _ecs: &Network,
        resource: &NetworkResource,
    ) -> Vec<String> {
        if let EventLoggerInfo::IsBlockConfirmationEvent(block_index, node_index, time) = info {
            vec![
                time.to_string(),
                node_index.to_string(),
                resource.blocks[*block_index].height.to_string(),
                resource.blocks[*block_index].size.to_string(),
                resource.blocks[*block_index]
                    .get_creation_time()
                    .to_string(),
                resource.blocks[*block_index]
                    .creator
                    .map(|c| c.to_string())
                    .unwrap_or("None".to_string()),
            ]
        } else {
            Vec::default()
        }
    }
}
