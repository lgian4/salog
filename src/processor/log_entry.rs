use std::{fmt, sync::OnceLock};

use chrono::{DateTime, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;
use strum::EnumString;

#[derive(Debug, Serialize, Deserialize, EnumString, Clone, PartialEq, Eq, Hash)]
pub enum HTTPMethod {
    GET,
    PUT,
    POST,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
    CONNECT,
    TRACE,
    NONE,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    INFO,
    WARN,
    ERROR,
    DEBUG,
    NONE,
}

#[serde_inline_default]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: LogLevel,
    pub message: String,

    #[serde_inline_default(HTTPMethod::NONE)]
    pub http_method: HTTPMethod,

    #[serde(default)]
    pub ip_address: String,

    #[serde(default)]
    pub url: String,

    #[serde(default)]
    pub status_code: String,

    #[serde(default)]
    pub error: String,

    #[serde(default)]
    pub process_time: f64,

    #[serde(default)]
    pub time_unix: Option<i64>,

    #[serde(default)]
    is_process: bool,
}

impl LogEntry {
    pub fn process_date(&mut self) {
        if self.time_unix.is_none() {
            let dt = self.timestamp.parse::<DateTime<Utc>>().ok();
            if let Some(dt) = dt {
                self.time_unix = Some(dt.timestamp_millis());
            }
        }
    }

    pub fn process(&mut self) {
        if self.is_process {
            return;
        }
        let regex = log_regex();

        if let Some(captures) = regex.captures(&self.message) {
            self.ip_address = captures[1].to_string();
            self.http_method = captures[2].parse().unwrap_or(HTTPMethod::NONE);
            self.url = captures[3].to_string();
            self.status_code = captures[4].to_string();
            self.process_time = captures[5].parse().unwrap_or(0f64);
        }

        if self.time_unix.is_none() {
            let dt = self.timestamp.parse::<DateTime<Utc>>().ok();
            if let Some(dt) = dt {
                self.time_unix = Some(dt.timestamp_millis());
            }
        }

        self.is_process = true;
    }
}

fn log_regex() -> &'static Regex {
    // Define the regular expression pattern
    static LOG_REGEX: OnceLock<Regex> = OnceLock::new();

    // Initialize the regular expression using OnceCell
    LOG_REGEX.get_or_init(|| {
        Regex::new(r"^([:\d\w.]+)\s-\s(\w+)\s([\/\w]+)\s(\d+)\s-\s([\d.]+).ms$").unwrap()
    })
}

impl fmt::Display for LogEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{\n")?;
        write!(f, "  \"timestamp\": \"{}\",\n", self.timestamp)?;
        write!(f, "  \"level\": \"{}\",\n", self.level)?;
        write!(f, "  \"message\": \"{}\",\n", self.message)?;

        write!(f, "  \"time_unix\": \"{}\",\n", self.time_unix.unwrap_or(0))?;

        if self.http_method != HTTPMethod::NONE {
            write!(f, "  \"http_method\": \"{}\",\n", self.http_method)?;
        }
        if !self.ip_address.is_empty() {
            write!(f, "  \"ip_address\": \"{}\",\n", self.ip_address)?;
        }
        if !self.url.is_empty() {
            write!(f, "  \"url\": \"{}\",\n", self.url)?;
        }
        if !self.status_code.is_empty() {
            write!(f, "  \"status_code\": \"{}\",\n", self.status_code)?;
        }
        if !self.error.is_empty() {
            write!(f, "  \"error\": \"{}\",\n", self.error)?;
        }

        write!(f, "  \"process_time\": {},\n", self.process_time)?;

        write!(f, "}}")
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string().as_str(),)
    }
}

impl std::fmt::Display for HTTPMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string().as_str(),)
    }
}

impl LogLevel {
    fn to_string(&self) -> String {
        match self {
            LogLevel::DEBUG => "DEBUG".to_string(),
            LogLevel::ERROR => "ERROR".to_string(),
            LogLevel::INFO => "INFO".to_string(),
            LogLevel::WARN => "WARN".to_string(),
            LogLevel::NONE => "none".to_string(),
        }
    }
}

impl HTTPMethod {
    fn to_string(&self) -> String {
        match self {
            HTTPMethod::GET => "GET".to_string(),
            HTTPMethod::PUT => "PUT".to_string(),
            HTTPMethod::POST => "POST".to_string(),
            HTTPMethod::DELETE => "DELETE".to_string(),
            HTTPMethod::PATCH => "PATCH".to_string(),
            HTTPMethod::HEAD => "HEAD".to_string(),
            HTTPMethod::OPTIONS => "OPTIONS".to_string(),
            HTTPMethod::CONNECT => "CONNECT".to_string(),
            HTTPMethod::TRACE => "TRACE".to_string(),
            HTTPMethod::NONE => "none".to_string(),
        }
    }
}
