use std::{fs, io::Write, path::PathBuf};
use crate::processor::{log_entry::LogEntry, log_processor_options::LogProcessorOptions, log_trait::SaveLogTrait};

pub struct FileSaveStrategy {
    path: PathBuf,
    truncate_on_save: bool,
}

impl FileSaveStrategy {
    pub fn create_from_options(path: PathBuf, option: LogProcessorOptions) -> Result<Self, String> {
        Ok(FileSaveStrategy {
            path,
            truncate_on_save: option.truncate_on_save,
        })
    }
}

impl SaveLogTrait for FileSaveStrategy {
    fn save(&self, logs: &Vec<LogEntry>) -> Result<(), String> {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(self.truncate_on_save)
            .open(self.path.clone())
            .map_err(|err| format!("failed to open file : {}", err))?;

        write!(file, "{}", serde_json::to_string(logs).unwrap())
            .map_err(|err| format!("failed to save file : {}", err))?;

        Ok(())
    }
}
