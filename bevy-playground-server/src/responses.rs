use axum::{
    http::HeaderValue,
    response::{IntoResponse, Response},
};

pub struct WithContentType<R>(R, HeaderValue);
impl<R> WithContentType<R> {
    pub fn new(response: R, content_type: &'static str) -> Self {
        WithContentType(response, HeaderValue::from_static(content_type))
    }
}
impl<R: IntoResponse> IntoResponse for WithContentType<R> {
    fn into_response(self) -> Response {
        let mut response = self.0.into_response();
        response.headers_mut().insert("Content-Type", self.1);

        response
    }
}

pub struct ErrorResponse<E>(pub E);
impl<E: std::error::Error> IntoResponse for ErrorResponse<E> {
    fn into_response(self) -> Response {
        let body = self.0.to_string();
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

impl<E: std::error::Error> From<E> for ErrorResponse<E> {
    fn from(err: E) -> Self {
        ErrorResponse(err)
    }
}
