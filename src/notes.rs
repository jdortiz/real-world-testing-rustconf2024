//! Module for the notes data and functionality
use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::notes_repo::NotesRepo;

type AppState = Arc<dyn NotesRepo + Sync + Send + 'static>;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Scope {
    pub project: String,
    pub area: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Note {
    pub title: String,
    pub tags: Vec<String>,
    pub text: String,
    pub scope: Scope,
}

pub async fn create(State(state): State<AppState>, Json(note): Json<Note>) -> (StatusCode, String) {
    match state.create(note).await {
        Ok(msg) => (StatusCode::OK, msg),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("Something went wrong"),
        ),
    }
}
