use axum::http::{header, HeaderValue};
use tower::{
    layer::util::{Identity, Stack},
    ServiceBuilder,
};
use tower_http::set_header::SetResponseHeaderLayer;

pub fn cors_middleware() -> Stack<
    SetResponseHeaderLayer<HeaderValue, hyper::Body>,
    Stack<SetResponseHeaderLayer<HeaderValue, hyper::Body>, Identity>,
> {
    ServiceBuilder::new()
        .layer(SetResponseHeaderLayer::<_, hyper::Body>::if_not_present(
            header::ACCESS_CONTROL_ALLOW_METHODS,
            HeaderValue::from_static("OPTION, GET, POST, PATCH, DELETE"),
        ))
        .layer(SetResponseHeaderLayer::<_, hyper::Body>::if_not_present(
            header::ACCESS_CONTROL_ALLOW_ORIGIN,
            HeaderValue::from_static("*"),
        ))
        .into_inner()
}
