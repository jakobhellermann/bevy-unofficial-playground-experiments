mod middleware;
mod responses;

use std::{convert::Infallible, net::SocketAddr};

use axum::http::StatusCode;
use axum::{handler::get, response::IntoResponse, service, Router};
use middleware::cors_middleware;
use responses::WithContentType;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let api_routes = Router::new()
        .route("/project/:id/playground.js", get(playground_js))
        .route("/project/:id/playground.wasm", get(playground_wasm))
        .layer(cors_middleware());

    let serve_dir = service::get(
        ServeDir::new("../bevy-playground-website/dist").handle_error(internal_server_error),
    );

    let app = Router::new().nest("/", serve_dir).nest("/api", api_routes);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn playground_js() -> WithContentType<&'static str> {
    let body = include_str!("../../bevy-builder/build/bevy-project.js");
    WithContentType::new(body, "application/javascript")
}
async fn playground_wasm() -> WithContentType<&'static [u8]> {
    let body = include_bytes!("../../bevy-builder/build/bevy-project_bg.wasm");
    WithContentType::new(body, "application/wasm")
}

fn internal_server_error(error: impl std::error::Error) -> Result<impl IntoResponse, Infallible> {
    Ok((
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Unhandled internal error: {}", error),
    ))
}
