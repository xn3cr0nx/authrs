use axum::{http::Request, body::Body};
use axum::{response::IntoResponse, routing::get, Router, TypedHeader};
use axum_tracing_opentelemetry::opentelemetry_tracing_layer;
use opentelemetry::trace::{Span, TraceContextExt, Tracer};
use opentelemetry::{global, Context};
use opentelemetry_http::HeaderExtractor;

use http::HeaderMap;

use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::trace::TraceLayer;

use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    IntoParams, Modify, OpenApi, ToSchema,
};
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
        paths(
            // status,
        ),
        components(
            schemas(),
        ),
        tags(
            (name = "authrs", description = "Authrs API")
        )
    )]
struct ApiDoc;

// pub fn router(tracer: Arc<Mutex<Tracer>>) -> Router {
pub fn router() -> Router {
    Router::new()
        .merge(SwaggerUi::new("/docs/*tail").url("/docs-api/openapi.json", ApiDoc::openapi()))
        .route("/", get(health))
        .route("/sub", get(subspan))
        // .layer(TraceLayer::new_for_http())
        .layer(opentelemetry_tracing_layer())
}

async fn health() -> impl IntoResponse {
    axum::Json(json!({ "status" : "UP" }))
}

// async fn subspan(headers: HeaderMap) -> impl IntoResponse {
async fn subspan(req: Request<Body>) -> impl IntoResponse {
    info!("Headers: {:?}", req.headers());

    let parent_cx = global::get_text_map_propagator(|propagator| {
        propagator.extract(&HeaderExtractor(req.headers()))
    });

    info!("Got ctx: {:?}", parent_cx);

    let mut span = global::tracer("").start_with_context("subspan", &parent_cx);
    span.add_event("handling in subspan", Vec::new());


    let _span = tracing::info_span!("fucking span", "some test");

    axum::Json(json!({ "status" : "Ok" }))
}