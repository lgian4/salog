use crate::processor::{log_entry::LogEntry, log_trait::OutputLogTrait};
use log::trace;

pub struct JsonPrettyOutputStrategy;

impl OutputLogTrait for JsonPrettyOutputStrategy {
    fn output(&self, logs: &Vec<LogEntry>) -> Result<(), String> {
        trace!("OutputLogTrait.output");

        for ele in logs {
            println!("{}", ele);
        }
        Ok(())
    }
}
