//! Event trait including `execute()` method.

pub mod block_confirmation_event;
pub mod block_mining_process;
pub mod generate_block_event;
pub mod receive_event;
pub mod send_event;

use crate::log::EventLoggerInfo;
use crate::network::Network;
use crate::simulator::randomness_engine::RandomnessEngine;
use crate::simulator::Simulator;
use std::fmt::Debug;

/// To enforce `Box<dyn Event>` to have `execute()` method to be called
///  when popped from [`Simulator`].
pub trait Event: Debug {
    fn execute(
        &self,
        _network: &mut dyn Network,
        _simulator: &mut Simulator,
        _rand: &mut RandomnessEngine,
    ) {
    }

    fn logger_data(&self, _time: f64) -> EventLoggerInfo {
        EventLoggerInfo::default()
    }
}
