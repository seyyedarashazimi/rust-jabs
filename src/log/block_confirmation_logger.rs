use crate::log::{CSVLogger, Logger, LoggerEventInfo};
use crate::network::resource::NetworkResource;
use crate::network::Network;
use crate::scenario::ScenarioData;
use csv::Writer;
use std::fs::File;
use std::path::Path;

pub struct BlockConfirmationLogger {
    logger_csv: Writer<File>,
}

impl BlockConfirmationLogger {
    pub fn from_path(path: &Path) -> csv::Result<Self> {
        let csv_logger = Writer::from_path(path)?;
        Ok(Self {
            logger_csv: csv_logger,
        })
    }
}

impl Logger for BlockConfirmationLogger {
    fn initial_log(&mut self, scenario: &ScenarioData) -> csv::Result<()> {
        // Write the comment as a regular record, starting with #
        let comment = self.csv_starting_comment(scenario);
        self.logger_csv.write_record(&comment)?;

        // Write the header
        let headers = self.csv_header_output();
        self.logger_csv.write_record(&headers)?;
        Ok(())
    }

    fn log_before_each_event(
        &mut self,
        info: &LoggerEventInfo,
        ecs: &Network,
        resource: &NetworkResource,
    ) -> csv::Result<()> {
        if self.csv_output_condition_before_event(info) {
            self.logger_csv
                .write_record(self.csv_event_output(info, ecs, resource))?;
        }
        Ok(())
    }

    fn log_after_each_event(
        &mut self,
        info: &LoggerEventInfo,
        ecs: &Network,
        resource: &NetworkResource,
    ) -> csv::Result<()> {
        if self.csv_output_condition_after_event(info) {
            self.logger_csv
                .write_record(self.csv_event_output(info, ecs, resource))?;
        }
        Ok(())
    }

    fn final_log(&mut self, scenario_data: &ScenarioData) -> Result<(), std::io::Error> {
        if self.csv_output_condition_final_per_node() {
            for node in 0..scenario_data.num_of_nodes {
                self.logger_csv.write_record(self.csv_node_output(node))?;
            }
        }
        self.logger_csv.flush()
    }
}

impl CSVLogger for BlockConfirmationLogger {
    fn csv_starting_comment(&self, scenario: &ScenarioData) -> Vec<String> {
        vec![
            "# Simulation name: ".to_string(),
            scenario.name.clone(),
            "Number of nodes: ".to_string(),
            scenario.num_of_nodes.to_string(),
            "Network type: ".to_string(),
            scenario.network_type.clone(),
        ]
    }

    fn csv_output_condition_before_event(&self, _: &LoggerEventInfo) -> bool {
        false
    }

    fn csv_output_condition_after_event(&self, info: &LoggerEventInfo) -> bool {
        matches!(info, LoggerEventInfo::IsBlockConfirmationEvent(..))
    }

    fn csv_output_condition_final_per_node(&self) -> bool {
        false
    }

    fn csv_header_output(&self) -> Vec<String> {
        let header_str = vec![
            "Time",
            "NodeIndex",
            "BlockHeight",
            "BlockSize",
            "BlockCreationTime",
            "BlockCreator",
        ];
        header_str.into_iter().map(String::from).collect()
    }

    fn csv_event_output(
        &self,
        info: &LoggerEventInfo,
        _ecs: &Network,
        resource: &NetworkResource,
    ) -> Vec<String> {
        if let LoggerEventInfo::IsBlockConfirmationEvent(block_index, node_index, time) = info {
            vec![
                time.to_string(),
                node_index.to_string(),
                resource.blocks[*block_index].height.to_string(),
                resource.blocks[*block_index].size.to_string(),
                resource.blocks[*block_index]
                    .get_creation_time()
                    .to_string(),
                resource.blocks[*block_index]
                    .creator
                    .map(|c| c.to_string())
                    .unwrap_or("None".to_string()),
            ]
        } else {
            Vec::default()
        }
    }
}
