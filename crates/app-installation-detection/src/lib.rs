use std::time::Duration;

use axum::body::Body;
use axum::http::request::Parts;
use axum::http::{HeaderValue, Method, Response};
use axum::{extract::Request, routing::get, Router};
use tower::ServiceBuilder;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::Span;

pub fn make_router() -> Router {
    let trace_service = ServiceBuilder::new().layer(
        TraceLayer::new_for_http()
            .make_span_with(|request: &Request<Body>| {
                tracing::info_span!(
                    "http-request",
                    method = request.method().as_str(),
                    uri = request.uri().to_string(),
                )
            })
            .on_request(())
            .on_response(
                |response: &Response<Body>, _latency: Duration, _span: &Span| {
                    tracing::info!(response_status = response.status().as_u16());
                },
            )
            .on_body_chunk(())
            .on_eos(())
            .on_failure(()),
    );

    // Allow requests from localhost and any warp.dev origin (e.g.
    // https://warp.dev, https://app.warp.dev). The suffix check keeps this
    // working for per-environment hosts (staging, dev overrides) without
    // hardcoding each subdomain. Extra origins can be added via
    // `WARP_INSTALL_DETECTION_EXTRA_ORIGINS` (comma-separated).
    let allow_origin_predicate =
        AllowOrigin::predicate(|origin: &HeaderValue, _request_parts: &Parts| {
            is_allowed_origin(origin)
        });

    let cors = CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_origin(allow_origin_predicate);

    Router::new()
        .route_service("/install_detection", get(detect_installation))
        .layer(trace_service)
        .layer(cors)
}

/// Extra allowed CORS origins, comma-separated (e.g. custom dev hosts).
/// Read at request time so tests and dev processes can override per-process.
fn extra_allowed_origins() -> Vec<String> {
    std::env::var("WARP_INSTALL_DETECTION_EXTRA_ORIGINS")
        .map(|raw| {
            raw.split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

fn is_allowed_origin(origin: &HeaderValue) -> bool {
    if origin == "http://localhost:8080" || origin == "http://localhost:8082" {
        return true;
    }
    if extra_allowed_origins()
        .iter()
        .any(|extra| origin.as_bytes() == extra.as_bytes())
    {
        return true;
    }
    let Ok(origin_str) = origin.to_str() else {
        return false;
    };
    let Ok(url) = url::Url::parse(origin_str) else {
        return false;
    };
    let Some(host) = url.host_str() else {
        return false;
    };
    host == "warp.dev" || host.ends_with(".warp.dev")
}

async fn detect_installation() -> &'static str {
    "ok"
}
