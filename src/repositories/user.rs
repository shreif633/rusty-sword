use pwhash::bcrypt;
use sqlx::query_scalar;

use crate::framework::database::Database;

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