use crate::{
    config::{DataType, GlobalOptions, SourceConfig, SourceDescription},
    shutdown::ShutdownSignal,
    sources::util::HttpSourceAuthConfig,
    tls::TlsConfig,
    Pipeline,
};
use futures::{
    FutureExt,
    TryFutureExt,
};
use serde::{Deserialize, Serialize};

#[derive(Derivative, Deserialize, Serialize, Clone, Debug, Default)]
#[serde(deny_unknown_fields)]
struct NginxMetricsConfig {
    endpoints: Vec<String>,
    #[serde(default = "default_scrape_interval_secs")]
    scrape_interval_secs: u64,
    #[serde(default = "default_namespace")]
    namespace: String,
    tls: Option<TlsConfig>,
    #[derivative(Default(value = "None"))]
    auth: Option<HttpSourceAuthConfig>,
}

pub fn default_scrape_interval_secs() -> u64 {
    15
}

pub fn default_namespace() -> String {
    "nginx".to_string()
}

inventory::submit! {
    SourceDescription::new::<NginxMetricsConfig>("nginx_metrics")
}

impl_generate_config_from_default!(NginxMetricsConfig);

#[async_trait::async_trait]
#[typetag::serde(name = "nginx_metrics")]
impl SourceConfig for NginxMetricsConfig {
    async fn build(
        &self,
        _name: &str,
        _globals: &GlobalOptions,
        _shutdown: ShutdownSignal,
        _out: Pipeline,
    ) -> crate::Result<super::Source> {
        Ok(Box::new(async move { Ok(()) }.boxed().compat()))
    }

    fn output_type(&self) -> DataType {
        DataType::Metric
    }

    fn source_type(&self) -> &'static str {
        "nginx_metrics"
    }
}
