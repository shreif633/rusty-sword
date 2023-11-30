use crate::repositories::user::{UserCreateChangeset, create_user};
use crate::framework::database::Database;

pub async fn create_account(username: &str, password: &str, email: &str) {
    let database = Database::connect().await;
    let changeset = UserCreateChangeset { username, password, email };
    let user_id = create_user(&database, changeset);
    println!("user created with id {}", user_id);
}