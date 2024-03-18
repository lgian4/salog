mod processor {
    pub mod log_processor;
    pub mod log_processor_options;
    pub mod output_log {
        pub mod count_output;
        pub mod json_output;
        pub mod json_pretty_output;
        pub mod summary_output;
    }
    pub mod get_log {
        pub mod es_get;
        pub mod file_get;
        pub mod url_get;
    }
    pub mod save_log {
        pub mod es_save;
        pub mod file_save;
    }
    pub mod command;
    pub mod db;
    pub mod log_entry;
    pub mod log_trait;
}

use clap::Parser;
use dotenv::dotenv;
use processor::{command::Cli, log_processor::LogProcessorFactory};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let cli: Cli = Cli::parse();

    let processor = LogProcessorFactory::from_cli(&cli)?;
    processor.run()?;
    Ok(())
}
