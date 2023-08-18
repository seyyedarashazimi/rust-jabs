use crate::log::EventLoggerInfo;
use crate::log::EventLoggerInfo::IsReceiveEvent;
use crate::network::message::MessageType;
use crate::network::Network;
use crate::simulator::event::Event;
use crate::simulator::randomness_engine::RandomnessEngine;
use crate::simulator::Simulator;

#[derive(Debug)]
pub struct ReceiveEvent {
    pub block_index: usize,
    pub from: usize,
    pub node: usize,
    pub msg_type: MessageType,
}

impl Event for ReceiveEvent {
    fn execute(
        &self,
        network: &mut dyn Network,
        simulator: &mut Simulator,
        _: &mut RandomnessEngine,
    ) {
        network.receive(simulator, &self);
    }

    fn logger_data(&self, time: f64) -> EventLoggerInfo {
        IsReceiveEvent(self.block_index, self.from, self.node, self.msg_type, time)
    }
}

impl ReceiveEvent {
    pub(crate) fn new(block_index: usize, from: usize, node: usize, msg_type: MessageType) -> Self {
        Self {
            block_index,
            from,
            node,
            msg_type,
        }
    }
}
