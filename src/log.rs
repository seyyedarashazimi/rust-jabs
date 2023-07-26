pub mod block_confirmation_logger;
pub mod block_generation_logger;
pub mod blockchain_reorg_logger;

use crate::network::message::MessageType;
use crate::network::resource::NetworkResource;
use crate::network::Network;
use crate::scenario::ScenarioData;
use csv::Writer;
use std::fs::File;
use std::path::Path;

#[derive(Default, PartialEq)]
pub enum EventLoggerInfo {
    IsBlockConfirmationEvent(usize, usize, f64), // block, node, time
    IsReceiveEvent(usize, usize, usize, MessageType, f64), // block, from, node, msg_type, time
    #[default]
    NotLoggerEvent,
}

pub trait CSVLogger {
    fn csv_starting_comment(&mut self, scenario_data: &ScenarioData) -> Vec<String> {
        scenario_data.into()
    }

    fn csv_output_condition_before_event(
        &mut self,
        _info: &EventLoggerInfo,
        _ecs: &Network,
        _resource: &NetworkResource,
    ) -> bool {
        false
    }

    fn csv_output_condition_after_event(
        &mut self,
        _info: &EventLoggerInfo,
        _ecs: &Network,
        _resource: &NetworkResource,
    ) -> bool {
        false
    }

    fn csv_output_condition_final_per_node(&self) -> bool {
        false
    }

    fn csv_header_output(&self) -> Vec<String>;

    fn csv_event_output(
        &self,
        info: &EventLoggerInfo,
        ecs: &Network,
        resource: &NetworkResource,
    ) -> Vec<String>;

    fn csv_node_output(&self, _node_index: usize) -> Vec<String> {
        Vec::default()
    }
}

pub trait Logger {
    fn initial_log(&mut self, scenario: &ScenarioData) -> csv::Result<()>;
    fn log_before_each_event(
        &mut self,
        info: &EventLoggerInfo,
        ecs: &Network,
        resource: &NetworkResource,
    ) -> csv::Result<()>;
    fn log_after_each_event(
        &mut self,
        info: &EventLoggerInfo,
        ecs: &Network,
        resource: &NetworkResource,
    ) -> csv::Result<()>;
    fn final_log(&mut self, scenario_data: &ScenarioData) -> Result<(), std::io::Error>;
}

pub struct EventLogger<C: CSVLogger> {
    csv_logger: C,
    csv_writer: Writer<File>,
}

impl<C: CSVLogger> EventLogger<C> {
    pub fn from_path(path: &Path, csv_logger: C) -> csv::Result<Self> {
        let csv_writer = Writer::from_path(path)?;
        Ok(Self {
            csv_logger,
            csv_writer,
        })
    }
}

impl<C: CSVLogger> Logger for EventLogger<C> {
    fn initial_log(&mut self, scenario: &ScenarioData) -> csv::Result<()> {
        // Write the comment as a regular record, starting with #
        let comment = self.csv_logger.csv_starting_comment(scenario);
        self.csv_writer.write_record(&comment)?;

        // Write the header
        let headers = self.csv_logger.csv_header_output();
        self.csv_writer.write_record(&headers)?;
        Ok(())
    }
    fn log_before_each_event(
        &mut self,
        info: &EventLoggerInfo,
        ecs: &Network,
        resource: &NetworkResource,
    ) -> csv::Result<()> {
        if self
            .csv_logger
            .csv_output_condition_before_event(info, ecs, resource)
        {
            self.csv_writer
                .write_record(self.csv_logger.csv_event_output(info, ecs, resource))?;
        }
        Ok(())
    }
    fn log_after_each_event(
        &mut self,
        info: &EventLoggerInfo,
        ecs: &Network,
        resource: &NetworkResource,
    ) -> csv::Result<()> {
        if self
            .csv_logger
            .csv_output_condition_after_event(info, ecs, resource)
        {
            self.csv_writer
                .write_record(self.csv_logger.csv_event_output(info, ecs, resource))?;
        }
        Ok(())
    }
    fn final_log(&mut self, scenario_data: &ScenarioData) -> Result<(), std::io::Error> {
        if self.csv_logger.csv_output_condition_final_per_node() {
            for node in 0..scenario_data.num_of_nodes {
                self.csv_writer
                    .write_record(self.csv_logger.csv_node_output(node))?;
            }
        }
        self.csv_writer.flush()
    }
}
