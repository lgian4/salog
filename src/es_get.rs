use elasticsearch::SearchParts;
use log::{debug, trace};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::runtime::Runtime;

use crate::{
    db::EsClient,
    log_entry::{LogEntry, LogLevel},
    log_processor_options::LogProcessorOptions,
    log_trait::GetLogTrait,
};

pub struct ESGetStrategy<'a> {
    index: String,
    reverse: bool,
    limit: i64,
    date_filter_string: Option<String>,
    date_filter: Option<(i64, i64)>,
    level_filter: Option<LogLevel>,
    es_client: &'a EsClient,
}
impl<'a> ESGetStrategy<'a> {
    pub fn create(
        index: String,
        option: LogProcessorOptions,
        es_client: &'a EsClient,
    ) -> Result<Self, String> {
        Ok(ESGetStrategy {
            index,
            reverse: option.reverse,
            limit: option.limit,
            date_filter: option.date_filter,
            date_filter_string: option.date_filter_string,
            level_filter: option.level_filter,
            es_client,
        })
    }

    fn search_body_builder(&self) -> serde_json::Value {
        let mut es_search_body = json!({"query": {},});

        if self.reverse == true {
            es_search_body["sort"] = json!(   [
              {
                "time_unix": {
                  "order": "desc"
                }
              }
            ]);
        }
        let mut filter_values: Vec<Value> = Vec::new();

        if let Some(date_filter) = self.date_filter {
            trace!(
                "filter_logs : date_string {}",
                self.date_filter_string.clone().unwrap_or("-".to_string())
            );
            filter_values.push(json!({
            "range":{
                "time_unix": {
                    "gte": date_filter.0,
                    "lte": date_filter.1
                }
            }}));
        }
        if let Some(level) = &self.level_filter {
            trace!("filter_logs : level {}", level.to_string());
            filter_values.push(json!(            {
                "term": {
                    "level": level.to_string()
                }
            }));
        }
        if filter_values.len() == 1 {
            es_search_body["query"] = filter_values.first().unwrap().clone();
        } else if filter_values.len() > 1 {
            es_search_body["query"] = json!({
                "bool": {
                    "must":
                        filter_values

                }
            })
        } else {
            es_search_body["query"] = json!({"match_all": {}})
        }
        debug!("{}", es_search_body);

        es_search_body
    }
}

impl<'a> GetLogTrait for ESGetStrategy<'a> {
    fn get(&self) -> Result<Vec<LogEntry>, String> {
        let es_search_body = self.search_body_builder();

        let runtime =
            Runtime::new().map_err(|err| format!("failed creating worker thread : {}", err))?;
        let result_json: String = runtime.block_on(async {
            let result = self
                .es_client
                .client
                .search(SearchParts::Index(&[self.index.as_str()]))
                .size(self.limit)
                .body(es_search_body)
                .send()
                .await
                .map_err(|err| format!("Failed opening file: {}", err))?;

            result
                .text()
                .await
                .map_err(|err| format!("Failed to get response text: {}", err))
        })?;

        let response: serde_json::Value = serde_json::from_str(&result_json).unwrap();

        let logs: Result<Vec<LogEntry>, String> = response["hits"]["hits"]
            .as_array()
            .ok_or_else(|| "Failed to get hits array".into())
            .and_then(|hits| {
                hits.iter()
                    .map(|hit| {
                        let hit: ElasticsearchHit =
                            serde_json::from_value(hit.clone()).map_err(|err| format!("{err}"))?;
                        let log_entry: LogEntry = serde_json::from_value(hit._source.clone())
                            .map_err(|err| format!("{err}"))?;
                        Ok(log_entry)
                    })
                    .collect()
            });
        let logs = logs?;

        Ok(logs)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ElasticsearchHit {
    _index: String,
    _id: String,
    _score: Option<f64>,
    _source: serde_json::Value,
}
