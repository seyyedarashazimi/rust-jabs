pub mod block_confirmation_logger;
pub mod block_generation_logger;
pub mod block_propagation_delay_logger;
pub mod blockchain_reorg_logger;

use crate::log::blockchain_reorg_logger::BlockchainReorgLogger;
use crate::network::message::MessageType;
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
        _network: &dyn NetworkLogHandler,
    ) -> bool {
        false
    }
    fn csv_output_condition_after_event(
        &mut self,
        _info: &EventLoggerInfo,
        _network: &dyn NetworkLogHandler,
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
        network: &dyn NetworkLogHandler,
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
        network: &dyn NetworkLogHandler,
    ) -> csv::Result<()>;

    fn log_after_each_event(
        &mut self,
        info: &EventLoggerInfo,
        network: &dyn NetworkLogHandler,
    ) -> csv::Result<()>;

    fn final_log(&mut self, scenario_data: &ScenarioData) -> Result<(), std::io::Error>;
}

/// A trait used for a network to provide log data to `Logger` objects.
pub trait NetworkLogHandler {
    fn get_block_creation_time(&self, block_index: usize) -> f64;
    fn get_block_creator(&self, block_index: usize) -> Option<usize>;
    fn get_block_height(&self, block_index: usize) -> i32;
    fn get_block_size(&self, block_index: usize) -> u64;
    fn get_block_parents(&self, block_index: usize) -> &Vec<usize>;
    fn get_num_of_nodes(&self) -> usize;

    // reorg logger methods:
    fn block_reorg_before(
        &self,
        _reorg_logger: &mut BlockchainReorgLogger,
        _block_index: &usize,
        _node_index: &usize,
    ) {
    }
    fn block_reorg_after(&self, _reorg_logger: &mut BlockchainReorgLogger) -> bool {
        false
    }
    fn block_reorg_output_length(
        &self,
        _reorg_logger: &BlockchainReorgLogger,
        _previous_head: usize,
        _node_index: &usize,
    ) -> i32 {
        0
    }
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
        network: &dyn NetworkLogHandler,
    ) -> csv::Result<()> {
        if self
            .csv_logger
            .csv_output_condition_before_event(info, network)
        {
            self.csv_writer
                .write_record(self.csv_logger.csv_event_output(info, network))?;
        }
        Ok(())
    }
    fn log_after_each_event(
        &mut self,
        info: &EventLoggerInfo,
        network: &dyn NetworkLogHandler,
    ) -> csv::Result<()> {
        if self
            .csv_logger
            .csv_output_condition_after_event(info, network)
        {
            self.csv_writer
                .write_record(self.csv_logger.csv_event_output(info, network))?;
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
