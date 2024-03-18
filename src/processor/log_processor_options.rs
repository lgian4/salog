use chrono::{Duration, Local, Timelike};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::{
    command::{Cli, InputCommand},
    log_entry::LogLevel,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LogInput {
    File(PathBuf),
    Url(String),
    EsIndex(String),
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LogSave {
    File(PathBuf),
    EsIndex(String),
    None,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LogOutput {
    Json,
    PrettyJson,
    Count,
    Summary,
    None,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogProcessorOptions {
    pub input: LogInput,
    pub reverse: bool,
    pub limit: i64,
    pub date_filter_string: Option<String>,
    pub level_filter: Option<LogLevel>,
    pub date_filter: Option<(i64, i64)>,
    pub truncate_on_save: bool,
    pub save: LogSave,
    pub output: LogOutput,
    pub verbose: bool, // es auth data
}

impl LogProcessorOptions {
    /// Constructs `LogProcessorOptions` from the provided `Cli` instance.
    ///
    /// # Arguments
    ///
    /// * `cli` - The `Cli` instance from which to construct `LogProcessorOptions`.
    ///
    /// # Returns
    ///
    /// A new `LogProcessorOptions` instance.
    ///
    /// # Errors
    ///
    /// This function will return an error if required fields are missing or parsing fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use your_crate::{Cli, LogProcessorOptions};
    /// let cli: Cli = Cli::parse();
    /// match LogProcessorOptions::from_cli(&cli) {
    ///     Ok(log_processor_options) => {
    ///         // Handle successful creation of LogProcessorOptions
    ///     },
    ///     Err(err) => {
    ///         // Handle error
    ///         eprintln!("Error: {}", err);
    ///     }
    /// }
    /// ```
    pub fn from_cli(cli: &Cli) -> Result<Self, String> {
        let input = match &cli.input {
            InputCommand {
                input_file: Some(x),
                ..
            } => LogInput::File(x.clone()),
            InputCommand {
                input_url: Some(x), ..
            } => LogInput::Url(x.clone()),
            InputCommand {
                input_es_index: Some(x),
                ..
            } => LogInput::EsIndex(x.clone()),
            _ => return Err("Input command not provided".to_string()),
        };
        let reverse = cli.reverse;
        let limit = cli.limit.unwrap_or(100_000 as i64);
        let date_filter_string = cli.date_filter.clone();
        let date_filter: Option<(i64, i64)> = date_filter_string
            .as_ref()
            .map(|ds| parse_date_filter(ds))
            .transpose()?;

        let level_filter = match &cli.level {
            None => None,
            Some(x) => parse_level_filter(x)?,
        };
        let truncate_on_save = cli.truncate;

        let mut output = LogOutput::None;

        if cli.json {
            output = LogOutput::Json;
        } else if cli.pretty_json {
            output = LogOutput::PrettyJson;
        } else if cli.count {
            output = LogOutput::Count;
        } else if cli.summary {
            output = LogOutput::Summary;
        }

        let mut save = LogSave::None;
        if cli.save_to_file.is_some() {
            save = LogSave::File(cli.save_to_file.clone().unwrap())
        } else if cli.save_to_es_index.is_some() {
            save = LogSave::EsIndex(cli.save_to_es_index.clone().unwrap())
        }
        let verbose = cli.verbose;

        Ok(LogProcessorOptions {
            input: input,
            reverse,
            limit,
            date_filter,
            date_filter_string,
            output,
            truncate_on_save,
            save,
            verbose,
            level_filter,
        })
    }
}

fn parse_level_filter(level: &str) -> Result<Option<LogLevel>, String> {
    match level.to_lowercase().as_str() {
        "debug" | "deb" | "d" => Ok(Some(LogLevel::DEBUG)),
        "error" | "err" | "e" | "ror" => Ok(Some(LogLevel::ERROR)),
        "info" | "in" | "i" | "inf" => Ok(Some(LogLevel::INFO)),
        "none" | "non" | "n" | "no" => Ok(Some(LogLevel::NONE)),
        "warn" | "war" | "w" => Ok(Some(LogLevel::WARN)),
        _ => Ok(None),
    }
}

fn parse_date_filter(date_filter_string: &str) -> Result<(i64, i64), String> {
    let today = Local::now().naive_local();
    let yesterday = today - Duration::try_days(1).unwrap();

    match date_filter_string {
        "yesterday" => Ok((yesterday.and_utc().timestamp(), today.and_utc().timestamp())),
        "today" | "0" => Ok((today.and_utc().timestamp(), today.and_utc().timestamp())),
        nn_nn if nn_nn.contains('_') => {
            let parts: Vec<&str> = nn_nn.split('_').collect();
            if parts.len() == 2 {
                if let (Ok(start), Ok(end)) = (parts[0].parse::<i64>(), parts[1].parse::<i64>()) {
                    return Ok((start * 3600, end * 3600)); // Convert hours to seconds
                } else {
                    panic!("Failed to parse date_filter")
                }
            }
            Err("Failed to parse date_filter nn_nn format".to_string())
        }
        n_dash if n_dash.ends_with('-') => {
            let days_to_subtract = n_dash.trim_end_matches('-').parse::<i64>().unwrap_or(1);
            let start = if days_to_subtract > 0 {
                today
                    - Duration::try_days(days_to_subtract)
                        .expect("failed to parse date_filter. make sure n- is a number")
            } else {
                today
                    - Duration::try_days(-days_to_subtract)
                        .expect("failed to parse date_filter. make sure n- is a number")
            };
            let end = today;
            Ok((
                start
                    .with_hour(0)
                    .unwrap()
                    .with_minute(0)
                    .unwrap()
                    .with_second(0)
                    .unwrap()
                    .and_utc()
                    .timestamp(),
                end.with_hour(23)
                    .unwrap()
                    .with_minute(59)
                    .unwrap()
                    .with_second(59)
                    .unwrap()
                    .and_utc()
                    .timestamp(),
            ))
        }
        _ => Err("Failed to parse date_filter : format not found".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cli_valid_input() {
        // Test with valid Cli input
        let cli = Cli {
            input: InputCommand {
                input_file: Some(PathBuf::new()),
                input_es_index: None,
                input_url: None,
            },
            reverse: false,
            json: false,
            truncate: false,
            save_to_file: None,
            count: false,
            summary: false,
            save_to_es_index: None,
            limit: None,
            date_filter: None,
            pretty_json: false,
            verbose: false,
            level: None,
        };
        let result = LogProcessorOptions::from_cli(&cli);
        assert!(result.is_ok());
        // Add assertions for expected LogProcessorOptions fields
    }

    #[test]
    fn test_from_cli_missing_input_command() {
        // Test with missing input command
        let cli = Cli {
            input: InputCommand {
                input_file: Some(PathBuf::new()),
                input_es_index: None,
                input_url: None,
            },
            reverse: false,
            json: false,
            truncate: false,
            save_to_file: None,
            count: true,
            summary: true,
            save_to_es_index: None,
            limit: None,
            date_filter: Some("what".to_string()),
            pretty_json: false,
            verbose: false,
            level: None,
        };
        let result = LogProcessorOptions::from_cli(&cli);
        assert!(result.is_err());
        assert_eq!(
            result.err(),
            Some("Failed to parse date filter".to_string())
        );
        // Add assertions for expected error message
    }

    // Add more tests for other scenarios...
}
