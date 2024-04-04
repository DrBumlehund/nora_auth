use axum::{routing::get, Router};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use std::env;
use uuid::Uuid;

mod models;
use self::models::*;
mod schema;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(|| async { "Hello, world!" }));

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_user(
    conn: &mut SqliteConnection,
    username: &String,
    password_hash: &String,
    salt: &String,
    full_name: &String,
) -> String {
    use crate::schema::users;

    let uuid = Uuid::new_v4().to_string();
    let now = Utc::now().naive_utc();
    let new_post = NewUser {
        id: &uuid,
        username: username,
        password_hash: password_hash,
        salt: salt,
        full_name: full_name,
        created_time: &now,
    };

    diesel::insert_into(users::table)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new user");

    uuid
}
