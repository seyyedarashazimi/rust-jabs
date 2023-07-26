use crate::log::EventLoggerInfo::IsReceiveEvent;
use crate::log::{CSVLogger, EventLoggerInfo};
use crate::network::message::{DataType, MessageType};
use crate::network::resource::NetworkResource;
use crate::network::Network;
use std::collections::{HashMap, HashSet};

pub struct BlockPropagationDelayLogger {
    shared_of_nodes_received_block: f64,
    received_by: HashMap<usize, HashSet<usize>>, // <block_index, Set<node_index>>
}

impl BlockPropagationDelayLogger {
    pub fn new(shared_of_nodes_received_block: f64) -> Self {
        Self {
            shared_of_nodes_received_block,
            received_by: HashMap::new(),
        }
    }
}

impl CSVLogger for BlockPropagationDelayLogger {
    fn csv_output_condition_after_event(
        &mut self,
        info: &EventLoggerInfo,
        ecs: &Network,
        _: &NetworkResource,
    ) -> bool {
        if let IsReceiveEvent(block, _, node, MessageType::DataMessage(DataType::IsBlock), _) = info
        {
            if self.received_by.contains_key(block) {
                self.received_by.get_mut(block).unwrap().insert(*node);
            } else {
                self.received_by.insert(*block, HashSet::new());
                self.received_by.get_mut(block).unwrap().insert(*node);
            }
            let exact_number = (ecs.num_of_nodes as f64) * self.shared_of_nodes_received_block;
            return self.received_by.get(block).unwrap().len() == (exact_number as usize);
        }
        false
    }

    fn csv_header_output(&self) -> Vec<String> {
        let header_str = vec![
            "Time",
            "PropagationDelay",
            "BlockIndex",
            "BlockHeight",
            "BlockCreator",
            "BlockSize",
        ];
        header_str.into_iter().map(String::from).collect()
    }

    fn csv_event_output(
        &self,
        info: &EventLoggerInfo,
        _: &Network,
        resource: &NetworkResource,
    ) -> Vec<String> {
        if let IsReceiveEvent(block_index, _, _, _, time) = info {
            return vec![
                time.to_string(),
                (time - resource.blocks[*block_index].get_creation_time()).to_string(),
                block_index.to_string(),
                resource.blocks[*block_index].height.to_string(),
                resource.blocks[*block_index]
                    .creator
                    .map(|c| c.to_string())
                    .unwrap_or("None".to_string()),
                resource.blocks[*block_index].size.to_string(),
            ];
        }
        vec![String::new(); 6]
    }
}
