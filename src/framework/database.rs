use bevy::prelude::*;
use std::env;
use sqlx::{SqlitePool, Pool, Sqlite};

#[derive(Resource)]
pub struct Database {
    pub connection: Pool<Sqlite>
}

impl Database {
    pub async fn connect() -> Self {
        let database_url = env::var("DATABASE_URL").unwrap();
        let connection = SqlitePool::connect(&database_url).await.unwrap();
        Database { connection }
    }
}