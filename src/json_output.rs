use crate::{log_entry::LogEntry, log_trait::OutputLogTrait};

pub struct JsonOutputStrategy;

impl OutputLogTrait for JsonOutputStrategy {
    fn output(&self, logs: &Vec<LogEntry>) -> Result<(), String> {
        println!("{}", serde_json::to_string(logs).unwrap());

        Ok(())
    }
}
