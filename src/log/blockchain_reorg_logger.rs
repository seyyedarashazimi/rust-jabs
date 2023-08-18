use crate::consensus::blockchain::local_block_tree::LocalBlockTree;
use crate::log::EventLoggerInfo::IsReceiveEvent;
use crate::log::{CSVLogger, EventLoggerInfo, NetworkLogHandler};
use crate::network::message::{DataType, MessageType};

#[derive(Default)]
pub struct BlockchainReorgLogger {
    pub(crate) network_view_block_tree: LocalBlockTree,
    pub(crate) new_block_received: bool,
    pub(crate) previous_head_chain_index: Option<usize>,
    pub(crate) current_node_index: Option<usize>,
}

impl BlockchainReorgLogger {
    pub fn new() -> Self {
        BlockchainReorgLogger::default()
    }
}

impl CSVLogger for BlockchainReorgLogger {
    fn csv_output_condition_before_event(
        &mut self,
        info: &EventLoggerInfo,
        network: &dyn NetworkLogHandler,
    ) -> bool {
        if let IsReceiveEvent(block, _, node, MessageType::DataMessage(DataType::IsBlock), _) = info
        {
            network.block_reorg_before(self, block, node);
        }
        false
    }

    fn csv_output_condition_after_event(
        &mut self,
        _: &EventLoggerInfo,
        network: &dyn NetworkLogHandler,
    ) -> bool {
        network.block_reorg_after(self)
    }

    fn csv_header_output(&self) -> Vec<String> {
        let header_str = vec![
            "Time",
            "NodeIndex",
            "BlockHeight",
            "BlockCreationTime",
            "BlockCreator",
            "ReorgLength",
        ];
        header_str.into_iter().map(String::from).collect()
    }

    fn csv_event_output(
        &self,
        info: &EventLoggerInfo,
        network: &dyn NetworkLogHandler,
    ) -> Vec<String> {
        if let IsReceiveEvent(block_index, _, node_index, _, time) = info {
            if let Some(previous_head) = self.previous_head_chain_index {
                let reorg_length =
                    network.block_reorg_output_length(&self, previous_head, node_index);

                return vec![
                    time.to_string(),
                    node_index.to_string(),
                    network.get_block_height(*block_index).to_string(),
                    network.get_block_creation_time(*block_index).to_string(),
                    network
                        .get_block_creator(*block_index)
                        .map(|c| c.to_string())
                        .unwrap_or("None".to_string()),
                    reorg_length.to_string(),
                ];
            }
        }
        vec![String::new(); 6]
    }
}
