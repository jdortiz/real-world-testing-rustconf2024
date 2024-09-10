mod mongodb_notes_repo;
mod notes;
mod notes_repo_error;
mod routes;

use std::env;

use dotenv::dotenv;
use tokio::net::TcpListener;

#[cfg(test)]
use crate::mongodb_notes_repo::MockMongodbNotesRepo as MongodbNotesRepo;
#[cfg(not(test))]
use crate::mongodb_notes_repo::MongodbNotesRepo;

#[tokio::main]
async fn main() {
    const SERVER_ADDR: &str = "127.0.0.1:8080";
    println!("Hola Caracola!");

    // Create notes repo
    dotenv().expect("Set your configuration in a .env file");
    let connection_string =
        env::var("MONGODB_CONN").expect("Define MONGODB_CONN=connection_string in your .env");
    #[cfg(not(test))]
    let notes_repo = MongodbNotesRepo::new(&connection_string).await.unwrap();
    #[cfg(test)]
    let notes_repo = MongodbNotesRepo::new();

    let listener = TcpListener::bind(SERVER_ADDR)
        .await
        .expect("Unable to create listener");
    axum::serve(listener, routes::app(notes_repo))
        .await
        .unwrap();
}
