use axum_tracing_opentelemetry::{make_resource, otlp};
use opentelemetry::{global, sdk::propagation::TraceContextPropagator, sdk::trace as sdktrace};
use tracing_subscriber::{layer::SubscriberExt, filter::EnvFilter};

pub fn init() {
    global::set_text_map_propagator(TraceContextPropagator::new());

    let otel_rsrc = make_resource(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    // let tracer = otlp::init_tracer(otel_rsrc, |p| {
    //     // otlp::identity(opentelemetry_otlp::new_pipeline().tracing())
    //     otlp::identity(p).with_exporter(opentelemetry_otlp::new_exporter().tonic())
    //     // .install_batch(opentelemetry::runtime::Tokio)
    //     //     .expect("cannot configure tracer")
    // })
    // .expect("cannot configure tracer");

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
    
    let subscriber = tracing_subscriber::registry()
        .with(fmt_layer)
        .with(EnvFilter::from_default_env())
        .with(otel_layer);
    tracing::subscriber::set_global_default(subscriber).unwrap();
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
