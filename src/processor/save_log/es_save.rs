use elasticsearch::{BulkOperation, BulkParts};
use log::debug;
use serde_json::{json, Value};
use tokio::runtime::Runtime;

use crate::processor::{db::EsClient, log_entry::LogEntry, log_processor_options::LogProcessorOptions, log_trait::SaveLogTrait};

pub struct ESSaveStrategy<'a> {
    index: String,
    truncate_on_save: bool,
    es_client: &'a EsClient,
}

impl<'a> ESSaveStrategy<'a> {
    pub fn create_from_options(
        index: String,
        option: LogProcessorOptions,
        es_client: &'a EsClient,
    ) -> Result<Self, String> {
        Ok(ESSaveStrategy {
            index,
            truncate_on_save: option.truncate_on_save,
            es_client,
        })
    }
}

impl<'a> SaveLogTrait for ESSaveStrategy<'a> {
    fn save(&self, logs: &Vec<LogEntry>) -> Result<(), String> {
        let runtime =
            Runtime::new().map_err(|err| format!("failed creating worker thread : {}", err))?;
        let result: Result<(), String> = runtime.block_on(async {
            if self.truncate_on_save {
                let es_search_body = json!({
                  "query": {
                    "match_all": {}
                  },

                });
                let response = self
                    .es_client
                    .client
                    .delete_by_query(elasticsearch::DeleteByQueryParts::Index(&[self
                        .index
                        .as_str()]))
                    .body(es_search_body)
                    .send()
                    .await
                    .map_err(|err| format!("Failed opening file: {}", err))?;

                let response = response
                    .text()
                    .await
                    .map_err(|err| format!("Failed to get response text: {}", err))?;
                debug!("truncate_on_save response: {}", response);
            }
            let chunk_size = 1000;

            let chunked_logs: Vec<Vec<LogEntry>> = logs
                .chunks(chunk_size)
                .map(|chunk| chunk.to_vec())
                .collect();

            for chunk in chunked_logs {
                let mut ops: Vec<BulkOperation<Value>> = Vec::with_capacity(chunk_size);

                for log in &chunk {
                    let value = serde_json::to_value(&log).unwrap();
                    ops.push(BulkOperation::create(log.timestamp.clone(), value).into());
                }

                let response = self
                    .es_client
                    .client
                    .bulk(BulkParts::Index(self.index.as_ref()))
                    .body(ops)
                    .send()
                    .await
                    .map_err(|err| format!("{}", err))?;

                let response = response
                    .text()
                    .await
                    .map_err(|err| format!("Failed to get response text: {}", err))?;
                debug!("response: {}", response);
            }

            Ok(())
        });
        result?;

        Ok(())
    }
}
