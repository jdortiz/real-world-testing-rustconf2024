//! Module for the notes repository abstraction.
#[cfg(test)]
use mockall::automock;

use crate::{notes::Note, notes_repo_error::NotesRepoError};

#[cfg_attr(test, automock)]
#[async_trait::async_trait]
pub trait NotesRepo {
    async fn create(&self, note: Note) -> Result<String, NotesRepoError>;
}
