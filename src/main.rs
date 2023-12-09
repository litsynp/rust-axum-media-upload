use std::net::SocketAddr;

use axum::extract::DefaultBodyLimit;
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt, Layer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::media::handlers::upload_media;

mod media;

#[derive(OpenApi)]
// @formatter:off
#[openapi(
    paths(
        media::handlers::upload_media,
    ),
    components(
        schemas(
            media::views::UploadMediaRequest,
        )
    ),
    info(
        title = "Media API",
        version = "0.1.0",
    ),
    tags(
        (name = "media", description = "Media API"),
    )
)]
// @formatter:on
struct ApiDoc;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .pretty()
                .with_filter(filter::LevelFilter::INFO),
        )
        .init();

    let max_body_size_in_bytes = 1024 * 1024 * 10;

    tracing::info!(
        "Starting server on port {} with max body size of {}MB",
        3000,
        max_body_size_in_bytes as f64 / 1024.0 / 1024.0
    );

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/", get(|| async { "Hello, World!" }))
        .route("/media", post(upload_media))
        .layer(DefaultBodyLimit::max(max_body_size_in_bytes))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
