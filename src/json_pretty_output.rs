use log::trace;

use crate::{log_entry::LogEntry, log_trait::OutputLogTrait};

pub struct JsonPrettyOutputStrategy;

impl OutputLogTrait for JsonPrettyOutputStrategy {
    fn output(&self, logs: &Vec<LogEntry>) -> Result<(), String> {
        trace!("OutputLogTrait.output");

        // println!("{}", serde_json::to_string_pretty(logs).unwrap());
        for ele in logs {
            println!("{}", ele);
        }
        Ok(())
    }
}
