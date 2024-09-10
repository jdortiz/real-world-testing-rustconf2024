mod mongodb_notes_repo;
mod notes;
mod notes_repo;
mod notes_repo_error;
mod routes;

use std::env;

use dotenv::dotenv;
use tokio::net::TcpListener;

use crate::mongodb_notes_repo::MongodbNotesRepo;

#[tokio::main]
async fn main() {
    const SERVER_ADDR: &str = "127.0.0.1:8080";
    println!("Hola Caracola!");

    // Create notes repo
    dotenv().expect("Set your configuration in a .env file");
    let connection_string =
        env::var("MONGODB_CONN").expect("Define MONGODB_CONN=connection_string in your .env");
    let notes_repo = MongodbNotesRepo::new(&connection_string).await.unwrap();

    let listener = TcpListener::bind(SERVER_ADDR)
        .await
        .expect("Unable to create listener");
    axum::serve(listener, routes::app(notes_repo))
        .await
        .unwrap();
}
