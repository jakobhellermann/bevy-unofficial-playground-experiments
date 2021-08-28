use axum::{http::HeaderValue, http::Response, response::IntoResponse};

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
