//! Module to implement a note repository using MongoDB

use mongodb::{
    options::{ClientOptions, ConnectionString},
    Client,
};

use crate::{notes::Note, notes_repo_error::NotesRepoError};

#[derive(Clone)]
pub struct MongodbNotesRepo {
    client: Client,
}

impl MongodbNotesRepo {
    pub async fn new(client_uri: &str) -> Result<MongodbNotesRepo, NotesRepoError> {
        let mut connection_string =
            ConnectionString::parse(client_uri).map_err(|_| NotesRepoError::BadCofiguration)?;
        connection_string.app_name = Some(String::from("jof-rc2024"));
        let options = ClientOptions::parse(connection_string)
            .await
            .map_err(|_| NotesRepoError::BadCofiguration)?;
        let client = Client::with_options(options).map_err(|_| NotesRepoError::UnavailableRepo)?;

        Ok(MongodbNotesRepo { client })
    }
    pub async fn create(&self, note: Note) -> Result<String, NotesRepoError> {
        let db = self.client.database("NoteKeeper");
        let notes_collection = db.collection::<Note>("Notes");
        match notes_collection.insert_one(&note).await {
            Ok(result) => Ok(format!("Id: {} Note: {note:?}", result.inserted_id)),
            Err(_) => Err(NotesRepoError::OperationFailed),
        }
    }
}
