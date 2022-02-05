mod responses;

mod compile;

use std::net::SocketAddr;

use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::Html;
use axum::routing::{get, get_service, post};
use axum::Json;
use axum::{response::IntoResponse, Router};
use compile::CompilationResult;
use responses::{ErrorResponse, WithContentType};
use tower_http::cors;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::info;

#[tokio::main]
async fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "bevy_playground=debug,tower_http=debug")
    }
    tracing_subscriber::fmt::init();

    let cors_layer = cors::CorsLayer::new()
        .allow_origin(cors::any())
        .allow_methods(cors::any());

    let api_routes = Router::new()
        .route("/compile", post(compile))
        .route("/project/:id/playground.js", get(playground_js))
        .route("/project/:id/playground.wasm", get(playground_wasm))
        .route("/project/:id/playground.html", get(playground_html))
        .layer(cors_layer);

    let serve_dir = get_service(ServeDir::new("../bevy-playground-website/dist"))
        .handle_error(internal_server_error);

    let app = Router::new()
        .nest("/api", api_routes)
        .fallback(serve_dir)
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("started server on http://localhost:{}", 3000);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub struct SourceHash(String);
impl std::fmt::Display for SourceHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

async fn playground_js(
    hash: Path<SourceHash>,
) -> Result<WithContentType<String>, ErrorResponse<compile::Error>> {
    let js = compile::read_output_js(&hash).await?;
    Ok(WithContentType::new(js, "application/javascript"))
}
async fn playground_wasm(
    hash: Path<SourceHash>,
) -> Result<WithContentType<Vec<u8>>, ErrorResponse<compile::Error>> {
    let wasm = compile::read_output_wasm(&hash).await?;
    Ok(WithContentType::new(wasm, "application/wasm"))
}
async fn playground_html(
    hash: Path<SourceHash>,
) -> Result<Html<String>, ErrorResponse<compile::Error>> {
    let html = include_str!("../templates/playground.html").replace("{id}", &hash.0 .0);
    Ok(Html(html))
}

async fn compile(body: String) -> Result<Json<CompilationResult>, ErrorResponse<compile::Error>> {
    let result = compile::compile(&body).await?;
    Ok(Json(result))
}

async fn internal_server_error(error: std::io::Error) -> impl IntoResponse {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Unhandled internal error: {}", error),
    )
}

impl<'de> serde::Deserialize<'de> for SourceHash {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct StringVisitor;
        impl serde::de::Visitor<'_> for StringVisitor {
            type Value = String;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string containing the source id")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if !v.chars().all(char::is_numeric) {
                    return Err(serde::de::Error::custom("expected a numeric string\n"));
                }
                Ok(v.to_string())
            }
        }

        deserializer.deserialize_str(StringVisitor).map(SourceHash)
    }
}
