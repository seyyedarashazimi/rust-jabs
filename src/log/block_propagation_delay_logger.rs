use crate::log::EventLoggerInfo::IsReceiveEvent;
use crate::log::{CSVLogger, EventLoggerInfo, NetworkLogHandler};
use crate::network::message::{DataType, MessageType};
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
        network: &dyn NetworkLogHandler,
    ) -> bool {
        if let IsReceiveEvent(block, _, node, MessageType::DataMessage(DataType::IsBlock), _) = info
        {
            if self.received_by.contains_key(block) {
                self.received_by.get_mut(block).unwrap().insert(*node);
            } else {
                self.received_by.insert(*block, HashSet::new());
                self.received_by.get_mut(block).unwrap().insert(*node);
            }
            let exact_number =
                (network.get_num_of_nodes() as f64) * self.shared_of_nodes_received_block;
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
        network: &dyn NetworkLogHandler,
    ) -> Vec<String> {
        if let IsReceiveEvent(block_index, _, _, _, time) = info {
            return vec![
                time.to_string(),
                (time - network.get_block_creation_time(*block_index)).to_string(),
                block_index.to_string(),
                network.get_block_height(*block_index).to_string(),
                network
                    .get_block_creator(*block_index)
                    .map(|c| c.to_string())
                    .unwrap_or("None".to_string()),
                network.get_block_size(*block_index).to_string(),
            ];
        }
        vec![String::new(); 6]
    }
}
