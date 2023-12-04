use bevy::prelude::*;

use crate::repositories::user::UserRow;

#[derive(Component)]
pub struct User {
    pub id: i32,
}

impl From<&UserRow> for User {
    fn from(user_row: &UserRow) -> Self {
        User { 
            id: user_row.id
        }
    }
}