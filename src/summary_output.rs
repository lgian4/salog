use crate::{
    log_entry::{HTTPMethod, LogEntry},
    log_trait::OutputLogTrait,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct SummaryOutputStrategy;

impl OutputLogTrait for SummaryOutputStrategy {
    fn output(&self, logs: &Vec<LogEntry>) -> Result<(), String> {
        let mut method_counts: HashMap<HTTPMethod, usize> = HashMap::new();

        for log in logs {
            let method = &log.http_method;
            let count = method_counts.entry(method.clone()).or_insert(0);
            *count += 1;
        }

        let summary = Summary {
            count: logs.len(),
            date_range: format!(
                "{} - {}",
                &logs.first().map_or("", |log| &log.timestamp),
                &logs.last().map_or("", |log| &log.timestamp)
            ),
            http_method: method_counts,
        };
        println!("{}", serde_json::to_string_pretty(&summary).unwrap());

        Ok(())
    }
}
#[derive(Debug, Serialize, Deserialize)]
struct Summary {
    count: usize,
    date_range: String,
    http_method: HashMap<HTTPMethod, usize>,
}
