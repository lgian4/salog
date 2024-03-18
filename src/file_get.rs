use log::trace;
use std::{fs, io::Read, path::PathBuf};

use crate::{
    log_entry::{LogEntry, LogLevel},
    log_processor_options::LogProcessorOptions,
    log_trait::GetLogTrait,
};

pub struct FileGetStrategy {
    path: PathBuf,
    reverse: bool,
    limit: i64,
    level_filter: Option<LogLevel>,
    date_filter_string: Option<String>,
    date_filter: Option<(i64, i64)>,
}
impl FileGetStrategy {
    pub fn create(path: PathBuf, option: LogProcessorOptions) -> Result<Self, String> {
        Ok(FileGetStrategy {
            path,
            reverse: option.reverse,
            limit: option.limit,
            date_filter: option.date_filter,
            date_filter_string: option.date_filter_string,
            level_filter: option.level_filter,
        })
    }
}

impl FileGetStrategy {
    fn read_file_contents(&self) -> Result<Vec<LogEntry>, String> {
        trace!("read_file_contents");
        let mut file = fs::OpenOptions::new()
            .read(true)
            .open(&self.path)
            .map_err(|err| format!("Failed opening file: {}", err))?;

        let mut body = String::new();
        file.read_to_string(&mut body)
            .map_err(|err| format!("Failed reading file: {}", err))?;

        let logs: Vec<LogEntry> =
            serde_json::from_str(&body).map_err(|err| format!("Failed parsing file: {}", err))?;
        Ok(logs)
    }
    fn process_logs_date(&self, mut logs: Vec<LogEntry>) -> Result<Vec<LogEntry>, String> {
        trace!("process_logs_date");
        let mut i = 0;
        let len = logs.len();
        for log in &mut logs {
            log.process_date();

            if i % 1000 == 0 {
                trace!("process_logs_date: {} /{}", i, len);
            }
            i = i + 1;
        }
        trace!("process_logs finished");

        Ok(logs)
    }

    fn process_logs(&self, mut logs: Vec<LogEntry>) -> Result<Vec<LogEntry>, String> {
        trace!("process_logs");
        let mut i = 0;
        let len = logs.len();
        for log in &mut logs {
            log.process();
            if i % 1000 == 0 {
                trace!("process_logs: {} /{}", i, len);
            }
            i = i + 1;
        }
        trace!("process_logs finished");

        Ok(logs)
    }

    fn filter_logs_date(&self, logs: Vec<LogEntry>) -> Result<Vec<LogEntry>, String> {
        if let Some(date_filter) = self.date_filter {
            trace!("filter_logs_date");
            trace!(
                " date_string {}",
                self.date_filter_string.clone().unwrap_or("-".to_string())
            );
            let filtered_logs = logs
                .into_iter()
                .filter(|log| {
                    if let Some(time_unix) = log.time_unix {
                        &time_unix > &date_filter.0 && &time_unix < &date_filter.1
                    } else {
                        false
                    }
                })
                .collect();
            Ok(filtered_logs)
        } else {
            trace!("skip filter");
            Ok(logs)
        }
    }
    fn filter_logs_level(&self, logs: Vec<LogEntry>) -> Result<Vec<LogEntry>, String> {
        if let Some(level) = &self.level_filter {
            trace!("filter_logs_level");
            trace!(" level {:?}", level);
            trace!("logs :{}", logs.len());
            let filtered_logs: Vec<LogEntry> =
                logs.into_iter().filter(|log| &log.level == level).collect();
            trace!("logs :{}", filtered_logs.len());
            Ok(filtered_logs)
        } else {
            trace!("skip filter");
            Ok(logs)
        }
    }

    fn reverse_logs(&self, mut logs: Vec<LogEntry>) -> Result<Vec<LogEntry>, String> {
        if self.reverse {
            trace!("reverse_logs");
            logs.reverse();
        } else {
            trace!("skip reverse_logs");
        }
        Ok(logs)
    }

    fn limit_logs(&self, mut logs: Vec<LogEntry>) -> Result<Vec<LogEntry>, String> {
        trace!("limit_logs {}", self.limit);

        logs.truncate(self.limit as usize);
        Ok(logs)
    }
}

impl GetLogTrait for FileGetStrategy {
    fn get(&self) -> Result<Vec<LogEntry>, String> {
        let logs = self.read_file_contents()?;
        let logs = self.process_logs_date(logs)?;
        let logs = self.filter_logs_level(logs)?;
        let logs = self.filter_logs_date(logs)?;
        let logs = self.reverse_logs(logs)?;
        let logs = self.limit_logs(logs)?;
        let logs = self.process_logs(logs)?;
        Ok(logs)
    }
}
