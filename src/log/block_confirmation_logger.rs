use crate::log::{CSVLogger, EventLoggerInfo, NetworkLogHandler};

#[derive(Default)]
pub struct BlockConfirmationLogger;

impl CSVLogger for BlockConfirmationLogger {
    fn csv_output_condition_after_event(
        &mut self,
        info: &EventLoggerInfo,
        _: &dyn NetworkLogHandler,
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
        network: &dyn NetworkLogHandler,
    ) -> Vec<String> {
        if let EventLoggerInfo::IsBlockConfirmationEvent(block_index, node_index, time) = info {
            vec![
                time.to_string(),
                node_index.to_string(),
                network.get_block_height(*block_index).to_string(),
                network.get_block_size(*block_index).to_string(),
                network.get_block_creation_time(*block_index).to_string(),
                network
                    .get_block_creator(*block_index)
                    .map(|c| c.to_string())
                    .unwrap_or("None".to_string()),
            ]
        } else {
            vec![String::new(); 6]
        }
    }
}
