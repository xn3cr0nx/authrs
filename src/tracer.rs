use axum_tracing_opentelemetry::make_resource;
use opentelemetry::{global, sdk::propagation::TraceContextPropagator, sdk::trace as sdktrace};
use tracing_subscriber::{filter::EnvFilter, layer::SubscriberExt};

use crate::environment::Env;

pub fn init(env: &Env) {
    global::set_text_map_propagator(TraceContextPropagator::new());

    let otel_rsrc = make_resource(env.name.clone(), env!("CARGO_PKG_VERSION").to_string());
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .with_trace_config(
            sdktrace::config()
                .with_resource(otel_rsrc)
                .with_sampler(sdktrace::Sampler::AlwaysOn),
        )
        .install_batch(opentelemetry::runtime::Tokio)
        .expect("cannot configure tracer");

    let otel_layer = tracing_opentelemetry::layer().with_tracer(tracer.clone());

    let fmt_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_timer(tracing_subscriber::fmt::time::uptime());

    let (layer, task) = tracing_loki::layer(
        url::Url::parse(&format!("http://{}:{}", env.loki_host, env.loki_port)).unwrap(),
        vec![("host".into(), env.name.clone().into())].into_iter().collect(),
        vec![].into_iter().collect(),
    )
    .expect("cannot configure loki");

    let subscriber = tracing_subscriber::registry()
        .with(fmt_layer)
        .with(EnvFilter::from_default_env())
        .with(layer)
        .with(otel_layer);
    tracing::subscriber::set_global_default(subscriber).unwrap();

    tokio::spawn(task);
}

pub async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::warn!("signal received, starting graceful shutdown");
    opentelemetry::global::shutdown_tracer_provider();
}
