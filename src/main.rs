#[macro_use]
extern crate tracing;

use axum::Server;
use opentelemetry::{global, metrics, Context, Key, KeyValue};

mod environment;
mod meter;
mod server;
mod tracer;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let env = environment::parse_env();

    tracer::init();

    let mt = meter::init(&env.name);
    let counter = mt
        .u64_counter("simple.counter")
        .with_description("Total number of nothing.")
        .init();

    debug!("Debug ON");

    let addr = format!("{}:{}", env.host, env.port).parse().unwrap();
    tracing::info!("listening on {}", addr);
    Server::bind(&addr)
        // .serve(server::router(tr).into_make_service())
        .serve(server::router().into_make_service())
        .with_graceful_shutdown(tracer::shutdown_signal())
        .await
        .unwrap()
}

