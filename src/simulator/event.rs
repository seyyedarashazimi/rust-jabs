//! Event trait including `execute()` method.

pub mod block_confirmation_event;
pub mod block_mining_process;
mod generate_block_event;
mod receive_event;
mod send_event;

use self::receive_event::ReceiveEvent;
use crate::log::EventLoggerInfo;
use crate::network::resource::NetworkResource;
use crate::network::Network;
use crate::simulator::randomness_engine::RandomnessEngine;
use crate::simulator::Simulator;
use std::fmt::Debug;

/// To enforce `Box<dyn Event>` to have `execute()` method to be called
///  when popped from [`Simulator`].
pub trait Event: Debug {
    fn execute(
        &self,
        _ecs: &mut Network,
        _simulator: &mut Simulator,
        _rand: &mut RandomnessEngine,
        _resource: &mut NetworkResource,
    ) {
    }

    fn logger_data(&self) -> EventLoggerInfo {
        EventLoggerInfo::default()
    }
}

// pub trait EventBoxClone {
//     fn event_box_clone(&self) -> Box<dyn Event>;
// }
//
// impl<T> EventBoxClone for T
// where
//     T: 'static + Event + Clone,
// {
//     fn event_box_clone(&self) -> Box<dyn Event> {
//         Box::new(self.clone())
//     }
// }
//
// impl Clone for Box<dyn Event> {
//     fn clone(&self) -> Box<dyn Event> {
//         self.event_box_clone()
//     }
// }
