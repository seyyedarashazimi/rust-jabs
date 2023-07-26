use crate::consensus::blockchain::local_block_tree::LocalBlockTree;
use crate::log::EventLoggerInfo::IsReceiveEvent;
use crate::log::{CSVLogger, EventLoggerInfo};
use crate::network::message::{DataType, MessageType};
use crate::network::resource::NetworkResource;
use crate::network::Network;

#[derive(Default)]
pub struct BlockchainReorgLogger {
    network_view_block_tree: LocalBlockTree,
    new_block_received: bool,
    previous_head_chain_index: Option<usize>,
    current_node_index: Option<usize>,
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
        ecs: &Network,
        resource: &NetworkResource,
    ) -> bool {
        if let IsReceiveEvent(block, _, node, MessageType::DataMessage(DataType::IsBlock), _) = info
        {
            self.network_view_block_tree.add(*block, &resource.blocks);
            self.previous_head_chain_index =
                Some(ecs.consensus_algorithm[*node].current_main_chain_head_index);
            self.current_node_index = Some(*node);
            self.new_block_received = true;
        }
        false
    }

    fn csv_output_condition_after_event(
        &mut self,
        _: &EventLoggerInfo,
        ecs: &Network,
        resource: &NetworkResource,
    ) -> bool {
        if self.new_block_received {
            self.new_block_received = false;
            if let Some(node_index) = self.current_node_index {
                let current_head_chain_index =
                    ecs.consensus_algorithm[node_index].current_main_chain_head_index;
                if let Some(previous_head_index) = self.previous_head_chain_index {
                    let ancestor_index =
                        self.network_view_block_tree.get_single_ancestor_of_height(
                            current_head_chain_index,
                            resource.blocks[previous_head_index].height,
                            &resource.blocks,
                        );
                    return ancestor_index != self.previous_head_chain_index;
                }
            }
        }
        false
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
        ecs: &Network,
        resource: &NetworkResource,
    ) -> Vec<String> {
        if let IsReceiveEvent(block_index, _, node_index, _, time) = info {
            if let Some(previous_head) = self.previous_head_chain_index {
                let node_chain_head =
                    ecs.consensus_algorithm[*node_index].current_main_chain_head_index;
                let common_ancestor = self.network_view_block_tree.get_common_ancestor(
                    node_chain_head,
                    previous_head,
                    &resource.blocks,
                );
                let reorg_length = resource.blocks[node_chain_head].height
                    - resource.blocks[common_ancestor].height;

                return vec![
                    time.to_string(),
                    node_index.to_string(),
                    resource.blocks[*block_index].height.to_string(),
                    resource.blocks[*block_index]
                        .get_creation_time()
                        .to_string(),
                    resource.blocks[*block_index]
                        .creator
                        .map(|c| c.to_string())
                        .unwrap_or("None".to_string()),
                    reorg_length.to_string(),
                ];
            }
        }
        vec![String::new(); 6]
    }
}
