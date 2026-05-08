use axum::Router;
use axum::extract::Request;
use tower::ServiceBuilder;
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, RequestId, SetRequestIdLayer},
    trace::TraceLayer,
};

pub fn apply_http_middleware(router: Router) -> Router {
    let middleware = ServiceBuilder::new()
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request| {
                let request_id = request
                    .extensions()
                    .get::<RequestId>()
                    .and_then(|request_id| request_id.header_value().to_str().ok())
                    .unwrap_or("missing");

                tracing::info_span!(
                    "http_request",
                    method = %request.method(),
                    uri = %request.uri(),
                    request_id = %request_id
                )
            }),
        );

    router.layer(middleware)
}
