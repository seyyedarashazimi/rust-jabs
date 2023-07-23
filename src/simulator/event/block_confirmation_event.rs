use crate::log::EventLoggerInfo;
use crate::simulator::event::Event;

#[derive(Debug)]
pub struct BlockConfirmationEvent {
    block_index: usize,
    node_index: usize,
    time: f64,
}

impl BlockConfirmationEvent {
    pub fn new(block_index: usize, node_index: usize, time: f64) -> Self {
        Self {
            block_index,
            node_index,
            time,
        }
    }
}

impl Event for BlockConfirmationEvent {
    fn logger_data(&self) -> EventLoggerInfo {
        EventLoggerInfo::IsBlockConfirmationEvent(self.block_index, self.node_index, self.time)
    }
}
