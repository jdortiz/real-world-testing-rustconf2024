//! Routes for the HTTP application
use std::sync::Arc;

use axum::{
    http::{StatusCode, Uri},
    routing::{get, post},
    Router,
};

use crate::{notes, notes_repo::NotesRepo};

pub fn app<R>(notes_repo: R) -> Router
where
    R: NotesRepo + Sync + Send + 'static,
{
    let state = Arc::new(notes_repo);
    Router::new()
        .route("/", get(|| async { "HTTP Caracola" }))
        .route("/notes", post(notes::create))
        .fallback(fallback_handler)
        .with_state(state)
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
    use mockall::predicate::eq;
    use notes::{Note, Scope};
    use tower::ServiceExt;

    use crate::notes_repo::MockNotesRepo;

    use super::*;

    #[tokio::test]
    async fn root_returns_static_response_and_ok() {
        let notes_repo = MockNotesRepo::new();
        let routes = app(notes_repo);
        let request = Request::builder().uri("/").body(Body::empty()).unwrap();

        let response = routes.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body, "HTTP Caracola");
    }

    #[tokio::test]
    async fn nonexisting_url_returns_emply_response_and_not_found() {
        let notes_repo = MockNotesRepo::new();
        let routes = app(notes_repo);
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
        let notes_repo = MockNotesRepo::new();
        let routes = app(notes_repo);
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
        let notes_repo = MockNotesRepo::new();
        let routes = app(notes_repo);
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

    #[tokio::test]
    async fn post_to_notes_with_data_creates_note_in_repo() {
        let mut notes_repo = MockNotesRepo::new();
        notes_repo
            .expect_create()
            .with(eq(Note {
                title: String::from("A note"),
                tags: vec![],
                text: String::from("An idea"),
                scope: Scope {
                    project: String::from("world domination"),
                    area: String::from("mind control"),
                },
            }))
            .once()
            .returning(|_| Ok(String::from("")));
        let routes = app(notes_repo);
        let request = Request::builder()
            .method(http::Method::POST)
            .uri("/notes")
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(
                "{\"title\": \"A note\", \"tags\": [], \"text\": \"An idea\", \"scope\": { \"project\": \"world domination\", \"area\": \"mind control\" } }",
            ))
            .unwrap();

        let _response = routes.oneshot(request).await.unwrap();

        // What does go here?
    }
}
