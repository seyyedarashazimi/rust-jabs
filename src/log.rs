pub mod block_confirmation_logger;
pub mod block_generation_logger;

use crate::network::resource::NetworkResource;
use crate::network::Network;
use crate::scenario::ScenarioData;

pub trait Logger {
    fn initial_log(&mut self, scenario_data: &ScenarioData) -> csv::Result<()>;
    fn log_before_each_event(
        &mut self,
        info: &LoggerEventInfo,
        ecs: &Network,
        resource: &NetworkResource,
    ) -> csv::Result<()>;
    fn log_after_each_event(
        &mut self,
        info: &LoggerEventInfo,
        ecs: &Network,
        resource: &NetworkResource,
    ) -> csv::Result<()>;
    fn final_log(&mut self, scenario_data: &ScenarioData) -> Result<(), std::io::Error>;
}

pub trait CSVLogger: Logger {
    fn csv_starting_comment(&self, scenario_data: &ScenarioData) -> Vec<String>;
    fn csv_output_condition_before_event(&self, info: &LoggerEventInfo) -> bool;
    fn csv_output_condition_after_event(&self, info: &LoggerEventInfo) -> bool;
    fn csv_output_condition_final_per_node(&self) -> bool;
    fn csv_header_output(&self) -> Vec<String>;
    fn csv_event_output(
        &self,
        info: &LoggerEventInfo,
        ecs: &Network,
        resource: &NetworkResource,
    ) -> Vec<String>;
    fn csv_node_output(&self, _node_index: usize) -> Vec<String> {
        Vec::default()
    }
}

#[derive(Default, PartialEq)]
pub enum LoggerEventInfo {
    IsBlockConfirmationEvent(usize, usize, f64), // block, node, time
    #[default]
    NotLoggerEvent,
}
