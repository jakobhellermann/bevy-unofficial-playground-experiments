mod middleware;
mod responses;

mod compile;

use std::{convert::Infallible, net::SocketAddr};

use axum::error_handling::HandleErrorExt;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::{handler::get, response::IntoResponse, service, Router};
use middleware::cors_middleware;
use responses::{ErrorResponse, WithContentType};
use tower_http::{services::ServeDir, trace::TraceLayer};

#[tokio::main]
async fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "bevy_playground_server=debug,tower_http=debug")
    }
    tracing_subscriber::fmt::init();

    let api_routes = Router::new()
        .route("/compile", post(compile))
        .route("/project/:id/playground.js", get(playground_js))
        .route("/project/:id/playground.wasm", get(playground_wasm))
        .layer(cors_middleware());

    let serve_dir = service::get(
        ServeDir::new("../bevy-playground-website/dist").handle_error(internal_server_error),
    );

    let app = Router::new()
        .nest("/api", api_routes)
        .fallback(serve_dir)
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

struct SourceHash(String);

async fn playground_js(
    hash: Path<SourceHash>,
) -> Result<WithContentType<String>, ErrorResponse<compile::Error>> {
    let js = compile::read_output_js(&hash.0 .0)?;
    Ok(WithContentType::new(js, "application/javascript"))
}
async fn playground_wasm(
    hash: Path<String>,
) -> Result<WithContentType<Vec<u8>>, ErrorResponse<compile::Error>> {
    let wasm = compile::read_output_wasm(&hash)?;
    Ok(WithContentType::new(wasm, "application/wasm"))
}

async fn compile(body: String) -> Result<String, ErrorResponse<compile::Error>> {
    let hash = tokio::task::spawn_blocking(move || compile::compile(&body))
        .await
        .unwrap()?;

    Ok(hash)
}

fn internal_server_error(error: impl std::error::Error) -> Result<impl IntoResponse, Infallible> {
    Ok((
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Unhandled internal error: {}", error),
    ))
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
