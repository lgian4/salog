use log::{debug, trace};
use tokio::runtime::Runtime;

use crate::processor::{
    log_entry::{LogEntry, LogLevel},
    log_processor_options::LogProcessorOptions,
    log_trait::GetLogTrait,
};

pub struct UrlGetStrategy {
    url: String,
    reverse: bool,
    limit: i64,
    date_filter_string: Option<String>,
    date_filter: Option<(i64, i64)>,
    level_filter: Option<LogLevel>,
}
impl UrlGetStrategy {
    pub fn create(url: String, option: LogProcessorOptions) -> Result<Self, String> {
        Ok(UrlGetStrategy {
            url,
            reverse: option.reverse,
            limit: option.limit,
            date_filter: option.date_filter,
            date_filter_string: option.date_filter_string,
            level_filter: option.level_filter,
        })
    }
}

impl UrlGetStrategy {
    async fn parse_data_from_url(&self) -> Result<Vec<LogEntry>, String> {
        trace!("parse_data_from_url");
        let url = get_default_url_from_env(&self.url)?;
        debug!("url: {}", url);
        // Make the HTTP request
        let response = reqwest::get(url)
            .await
            .map_err(|err| format!("failed fetching response : {}", err))?;
        let body: String = response
            .text()
            .await
            .map_err(|err| format!("failed fetching response : {}", err))?;
        debug!("url body: {}", body.len());
        // Parse each line as JSON and collect into Vec<Log>
        let logs: Vec<LogEntry> = body
            .lines()
            .filter_map(|line| serde_json::from_str(line).unwrap())
            .collect();
        debug!("logs: {}", logs.len());
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

impl GetLogTrait for UrlGetStrategy {
    fn get(&self) -> Result<Vec<LogEntry>, String> {
        let runtime =
            Runtime::new().map_err(|err| format!("failed creating worker thread : {}", err))?;
        let logs = runtime.block_on(self.parse_data_from_url())?;
        let logs = self.process_logs_date(logs)?;
        let logs = self.filter_logs_level(logs)?;
        let logs = self.filter_logs_date(logs)?;
        let logs = self.reverse_logs(logs)?;
        let logs = self.limit_logs(logs)?;
        let logs = self.process_logs(logs)?;
        Ok(logs)
    }
}

fn get_default_url_from_env(suffix: &str) -> Result<String, String> {
    trace!("get_default_url_from_env");
    let env_var_name = format!("DEFAULT_URL_{}", suffix);
    debug!("env_var_name {}", env_var_name);
    match std::env::var(&env_var_name) {
        Ok(url) => Ok(url),
        Err(err) => Err(format!(
            "Environment variable {} not found, using default. error: {}",
            suffix, err
        )),
    }
}
