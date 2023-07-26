use crate::log::EventLoggerInfo;
use crate::simulator::event::Event;

#[derive(Debug)]
pub struct BlockConfirmationEvent {
    block_index: usize,
    node_index: usize,
}

impl BlockConfirmationEvent {
    pub fn new(block_index: usize, node_index: usize) -> Self {
        Self {
            block_index,
            node_index,
        }
    }
}

impl Event for BlockConfirmationEvent {
    fn logger_data(&self, time: f64) -> EventLoggerInfo {
        EventLoggerInfo::IsBlockConfirmationEvent(self.block_index, self.node_index, time)
    }
}
