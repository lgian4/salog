use crate::processor::{log_entry::LogEntry, log_trait::OutputLogTrait};


pub struct CountOutputStrategy;

impl OutputLogTrait for CountOutputStrategy {
    fn output(&self, logs: &Vec<LogEntry>) -> Result<(), String> {
        println!("{}", logs.len());

        Ok(())
    }
}
