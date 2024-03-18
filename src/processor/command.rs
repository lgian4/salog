use clap::{Args, Parser};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    // /// What mode to run the program in
    #[command(flatten)]
    pub input: InputCommand,

    /// reverse before limit logs message
    #[arg(long, short)]
    pub reverse: bool,

    /// filter by level
    #[arg(long)]
    pub level: Option<String>,

    /// return logs as json text
    #[arg(short('j'), long, short)]
    pub json: bool,

    ///  remove all existing logs before save
    #[arg(short, long, requires = "save")]
    pub truncate: bool,

    /// save logs to file
    #[arg(short('f'), long, group = "save")]
    pub save_to_file: Option<PathBuf>,

    /// return logs as json text
    #[arg(long, short, group = "output")]
    pub count: bool,

    /// return logs as json text
    #[arg(long, short, group = "output")]
    pub summary: bool,

    /// save logs to elastic search index
    #[arg(short('e'), long, group = "save")]
    pub save_to_es_index: Option<String>,

    /// limit number, only take the n limit from first
    #[arg(long, short)]
    pub limit: Option<i64>,

    #[arg(long)]
    pub date_filter: Option<String>,

    /// show pretty in json output
    #[arg(long, short, group = "output")]
    pub pretty_json: bool,

    /// show pretty in json output
    #[arg(long, short)]
    pub verbose: bool,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
pub struct InputCommand {
    /// INPUT COMMAND : input logs from file
    #[arg(short('F'), long)]
    pub input_file: Option<PathBuf>,

    /// INPUT COMMAND : input logs from url
    #[arg(short('U'), long)]
    pub input_url: Option<String>,

    /// INPUT COMMAND : input logs from elastic search index
    #[arg(short('E'), long)]
    pub input_es_index: Option<String>,
}
