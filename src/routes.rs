//! Routes for the HTTP application
use axum::{routing::get, Router};

pub fn app() -> Router {
    Router::new().route("/", get(|| async { "HTTP Caracola" }))
}


#[cfg(test)]
mod tests {
    use axum::{body::Body, http::{Request, StatusCode}};
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    use super::*;

    #[tokio::test]
    async fn root_returns_static_response_and_ok() { 
        let routes = app();
        let request = Request::builder().uri("/").body(Body::empty()).unwrap();

        let response = routes.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body, "HTTP Caracola");
    }
}
