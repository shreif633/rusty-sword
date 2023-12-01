use pwhash::{bcrypt, unix};
use sqlx::{query_scalar, query};
use crate::framework::database::Database;

pub struct UserRow {
    pub id: u32,
    pub username: String,
    pub password_hash: String,
    pub email: String
}

pub fn find_user_by_username_and_password(database: &Database, username: &str, password: &str) -> Option<UserRow> {
    let rt = tokio::runtime::Builder::new_current_thread().enable_time().build().unwrap();
    let result = rt.block_on(async move {
        query!("SELECT * FROM users WHERE username = ?", username).fetch_one(&database.connection).await
    });
    if result.is_err() {
        return None;
    }
    let row = result.unwrap();
    if !unix::verify(password, &row.password_hash) {
        return None
    }
    Some(UserRow {
        id: row.id.try_into().unwrap(),
        username: row.username,
        password_hash: row.password_hash,
        email: row.email,
    })
}

pub struct UserCreateChangeset<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub email: &'a str
}

pub fn create_user(database: &Database, changeset: &UserCreateChangeset) -> u32 {
    let password_hash = bcrypt::hash(changeset.password).unwrap();
    let rt = tokio::runtime::Builder::new_current_thread().enable_time().build().unwrap();
    rt.block_on(async move {
        query_scalar!(
            "INSERT INTO users (username, password_hash, email) values (?, ?, ?) RETURNING id", 
            changeset.username, password_hash, changeset.email
        ).fetch_one(&database.connection).await.unwrap() as u32
    })
}