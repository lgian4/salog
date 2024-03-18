mod command;
mod count_output;
mod db;
mod es_get;
mod es_save;
mod file_get;
mod file_save;
mod json_output;
mod json_pretty_output;
mod log_entry;
mod log_processor;
mod log_processor_options;
mod log_trait;
mod summary_output;
mod url_get;

use clap::Parser;
use command::Cli;
use dotenv::dotenv;
use log_processor::LogProcessorFactory;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let cli: Cli = Cli::parse();
    // let cli = Cli {
    //     input: InputCommand {
    //         input_file: Some(PathBuf::from_str("log2.json").unwrap()),
    //         input_es_index: None,
    //         input_url: None,
    //     },
    //     reverse: false,
    //     json: true,
    //     truncate: false,
    //     save_to_file: None,
    //     count: false,
    //     summary: false,
    //     save_to_es_index: None,
    //     limit: Some(10),
    //     date_filter: None,
    //     pretty_json: false,
    //     verbose: true,
    // };

    let processor = LogProcessorFactory::from_cli(&cli)?;
    processor.run()?;
    Ok(())
}
