use std::sync::OnceLock;

use log::{debug, info};

use crate::{
    command::Cli,
    count_output::CountOutputStrategy,
    db::EsClient,
    es_get::ESGetStrategy,
    es_save::ESSaveStrategy,
    file_get::FileGetStrategy,
    file_save::FileSaveStrategy,
    json_output::JsonOutputStrategy,
    json_pretty_output::JsonPrettyOutputStrategy,
    log_processor_options::{LogInput, LogOutput, LogProcessorOptions, LogSave},
    log_trait::{GetLogTrait, OutputLogTrait, SaveLogTrait},
    summary_output::SummaryOutputStrategy,
    url_get::UrlGetStrategy,
};

pub struct LogProcessor {
    get_impl: Box<dyn GetLogTrait>,
    save_impl: Option<Box<dyn SaveLogTrait>>,
    output_impl: Option<Box<dyn OutputLogTrait>>,
}

impl LogProcessor {
    pub fn run(&self) -> Result<(), String> {
        let logs = (*self.get_impl)
            .get()
            .map_err(|err| format!("Failed Get Logs: {}", err))?;

        if let Some(save_impl) = &self.save_impl {
            (*save_impl).save(logs.as_ref())?;
        }

        if let Some(output_impl) = &self.output_impl {
            (*output_impl).output(logs.as_ref())?;
        }

        Ok(())
    }
}

pub struct LogProcessorFactory {}

impl LogProcessorFactory {
    pub fn from_cli(cli: &Cli) -> Result<LogProcessor, String> {
        let option = LogProcessorOptions::from_cli(&cli).unwrap();
        let mut level = log::LevelFilter::Off;
        if option.verbose {
            level = log::LevelFilter::Trace;
        }
        env_logger::Builder::from_default_env()
            .filter_level(level)
            .write_style(env_logger::WriteStyle::Always)
            .init();

        info!("option created");
        debug!("{:?}", option);
        static ES_CLIENT: OnceLock<EsClient> = OnceLock::new();

        let es_client = ES_CLIENT.get_or_init(|| EsClient::create().unwrap());

        let get_impl: Result<Box<dyn GetLogTrait>, String> = match &option.input {
            LogInput::File(x) => {
                let file_get = FileGetStrategy::create(x.clone(), option.clone())?;
                Ok(Box::new(file_get))
            }
            LogInput::Url(x) => {
                let url_get = UrlGetStrategy::create(x.clone(), option.clone())?;
                Ok(Box::new(url_get))
            }
            LogInput::EsIndex(x) => {
                let es_get = ESGetStrategy::create(x.clone(), option.clone(), &es_client)?;
                Ok(Box::new(es_get))
            } // cool that i don't need this
              // _ => Err("Input command not implemented".to_string()),
        };
        let get_impl = get_impl?;

        let save_impl: Result<Option<Box<dyn SaveLogTrait>>, String> = match &option.save {
            LogSave::File(x) => {
                let file_save = FileSaveStrategy::create_from_options(x.clone(), option.clone())?;
                Ok(Some(Box::new(file_save)))
            }
            LogSave::EsIndex(x) => {
                let es_save =
                    ESSaveStrategy::create_from_options(x.clone(), option.clone(), &es_client)?;
                Ok(Some(Box::new(es_save)))
            }
            LogSave::None => Ok(None),
        };
        let save_impl = save_impl?;

        let out_impl: Result<Option<Box<dyn OutputLogTrait>>, String> = match &option.output {
            LogOutput::Json => Ok(Some(Box::new(JsonOutputStrategy {}))),
            LogOutput::PrettyJson => Ok(Some(Box::new(JsonPrettyOutputStrategy {}))),
            LogOutput::Count => Ok(Some(Box::new(CountOutputStrategy {}))),
            LogOutput::Summary => Ok(Some(Box::new(SummaryOutputStrategy {}))),
            LogOutput::None => Ok(None),
        };
        let out_impl = out_impl?;
        Ok(LogProcessor {
            get_impl: get_impl,
            output_impl: out_impl,
            save_impl: save_impl,
        })
    }
}
