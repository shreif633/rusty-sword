use sqlx::{SqlitePool, Pool, Sqlite};
use bevy::prelude::*;

const DB_URL: &str = "sqlite://db/sword.db";

#[derive(Resource)]
pub struct Database {
    pub connection: Pool<Sqlite>
}

impl Database {
    pub async fn connect() -> Self {
        let connection = SqlitePool::connect(DB_URL).await.unwrap();
        Database { connection }
    }
}