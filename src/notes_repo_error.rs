//! Module to define the notes repo errors.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum NotesRepoError {
    #[error("Bad configuration for this repository")]
    BadCofiguration,
    #[error("Unavailable note repository")]
    UnavailableRepo,
    #[error("Repository operation failed")]
    OperationFailed,
}
