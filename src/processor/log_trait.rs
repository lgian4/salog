use super::log_entry::LogEntry;

pub trait GetLogTrait {
    fn get(&self) -> Result<Vec<LogEntry>, String>;
}

pub trait SaveLogTrait {
    fn save(&self, logs: &Vec<LogEntry>) -> Result<(), String>;
}

pub trait OutputLogTrait {
    fn output(&self, logs: &Vec<LogEntry>) -> Result<(), String>;
}
