use elasticsearch::{
    auth::Credentials,
    cert::CertificateValidation,
    http::transport::{SingleNodeConnectionPool, TransportBuilder},
    Elasticsearch,
};
use reqwest::Url;
use std::env::var;
pub struct EsClient {
    pub client: Elasticsearch,
}

impl EsClient {
    fn get_env_var(var_name: &str) -> Result<String, String> {
        var(var_name).map_err(|err| format!("failed to get {} in env: {}", var_name, err))
    }

    pub fn create() -> Result<Self, String> {
        let host = EsClient::get_env_var("ELASTIC_HOST")?;

        let user = EsClient::get_env_var("ELASTIC_USER")?;
        let pass = EsClient::get_env_var("ELASTIC_PASS")?;
        let elastic_use_cert_validation = EsClient::get_env_var("ELASTIC_PASS")?;

        let url = Url::parse(&host)
            .map_err(|err| format!("failed to create elastic url host : {}", err))?;
        let elastic_use_cert_validation: bool = match elastic_use_cert_validation.as_str() {
            "True" | "TRUE" | "true" | "t" | "1" => true,
            _ => false,
        };

        let conn_pool = SingleNodeConnectionPool::new(url);

        let credentials = Credentials::Basic(user, pass);
        let mut transport_builder = TransportBuilder::new(conn_pool)
            .auth(credentials)
            .disable_proxy();
        if !elastic_use_cert_validation {
            transport_builder = transport_builder.cert_validation(CertificateValidation::None);
        }

        let transport = transport_builder.build().unwrap();
        let client = Elasticsearch::new(transport);

        Ok(EsClient { client })
    }
}
