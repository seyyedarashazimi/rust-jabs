use crate::log::{CSVLogger, EventLoggerInfo};
use crate::network::resource::NetworkResource;
use crate::network::Network;

#[derive(Default)]
pub struct BlockConfirmationLogger;

impl CSVLogger for BlockConfirmationLogger {
    fn csv_output_condition_after_event(
        &mut self,
        info: &EventLoggerInfo,
        _: &Network,
        _: &NetworkResource,
    ) -> bool {
        matches!(info, EventLoggerInfo::IsBlockConfirmationEvent(..))
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
            vec![String::new(); 6]
        }
    }
}
