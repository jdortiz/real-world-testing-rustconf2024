//! Module for the notes data and functionality
use axum::{http::StatusCode, Json};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Scope {
    pub project: String,
    pub area: String,
}

#[derive(Debug, Deserialize)]
pub struct Note {
    pub title: String,
    pub tags: Vec<String>,
    pub text: String,
    pub scope: Scope,
}

pub async fn create(Json(note): Json<Note>) -> (StatusCode, String) {
    (StatusCode::OK, String::from(""))
}
