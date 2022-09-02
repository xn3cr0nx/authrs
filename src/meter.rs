use opentelemetry::global;
use opentelemetry::metrics::Meter;
use opentelemetry::sdk::metrics::selectors;
use opentelemetry::util::tokio_interval_stream;
use opentelemetry_otlp::{ExportConfig, Protocol, WithExportConfig};
use std::time::Duration;

// pub fn init(service: & 'static str) -> Meter {
pub fn init(service: &str) -> Meter {
    let export_config = ExportConfig {
        endpoint: "http://localhost:4317".to_string(),
        timeout: Duration::from_secs(3),
        protocol: Protocol::Grpc,
    };

    opentelemetry_otlp::new_pipeline()
        .metrics(tokio::spawn, tokio_interval_stream)
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_export_config(export_config),
            // can also config it using with_* functions like the tracing part above.
        )
        // .with_stateful(true)
        .with_period(Duration::from_secs(3))
        .with_timeout(Duration::from_secs(10))
        .with_aggregator_selector(selectors::simple::Selector::Exact)
        .build()
        .expect("cannot configure meter");

    let meter = global::meter(Box::leak(service.to_string().into_boxed_str()));
    meter
}
