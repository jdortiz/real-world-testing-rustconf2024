//! Routes for the HTTP application
use axum::{
    http::{StatusCode, Uri},
    routing::{get, post},
    Router,
};

use crate::notes;

pub fn app() -> Router {
    Router::new()
        .route("/", get(|| async { "HTTP Caracola" }))
        .route("/notes", post(notes::create))
        .fallback(fallback_handler)
}

async fn fallback_handler(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route for {}", uri))
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
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

    #[tokio::test]
    async fn nonexisting_url_returns_emply_response_and_not_found() {
        let routes = app();
        let request = Request::builder()
            .uri("/nonexisting")
            .body(Body::empty())
            .unwrap();

        let response = routes.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body, "No route for /nonexisting");
    }

    #[tokio::test]
    async fn post_to_notes_without_content_type_returns_unsuported_media_type() {
        let routes = app();
        let request = Request::builder()
            .method(http::Method::POST)
            .uri("/notes")
            .body(Body::empty())
            .unwrap();

        let response = routes.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    #[tokio::test]
    async fn post_to_notes_with_bad_data_returns_unprocessable_entity() {
        let routes = app();
        let request = Request::builder()
            .method(http::Method::POST)
            .uri("/notes")
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(
                "{\"title\": \"A note\", \"text\": \"An idea\" }",
            ))
            .unwrap();

        let response = routes.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }
}
