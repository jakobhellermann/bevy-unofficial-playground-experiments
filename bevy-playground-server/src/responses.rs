use std::convert::Infallible;

use axum::{
    body::{Bytes, Full},
    http::HeaderValue,
    http::Response,
    response::IntoResponse,
};

pub struct WithContentType<R>(R, HeaderValue);
impl<R> WithContentType<R> {
    pub fn new(response: R, content_type: &'static str) -> Self {
        WithContentType(response, HeaderValue::from_static(content_type))
    }
}
impl<R: IntoResponse> IntoResponse for WithContentType<R> {
    type Body = R::Body;
    type BodyError = R::BodyError;

    fn into_response(self) -> Response<Self::Body> {
        let mut response = self.0.into_response();
        response.headers_mut().insert("Content-Type", self.1);

        response
    }
}

pub struct ErrorResponse<E>(pub E);
impl<E: std::error::Error> IntoResponse for ErrorResponse<E> {
    type Body = Full<Bytes>;
    type BodyError = Infallible;

    fn into_response(self) -> Response<Self::Body> {
        let body = self.0.to_string();
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

impl<E: std::error::Error> From<E> for ErrorResponse<E> {
    fn from(err: E) -> Self {
        ErrorResponse(err)
    }
}
