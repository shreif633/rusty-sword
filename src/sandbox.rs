use sqlx::SqlitePool;
use pwhash::bcrypt;
// use crate::framework::packet::Packet;

async fn create_account(username: &str, password: &str, email: &str) {
    const DB_URL: &str = "sqlite://db/sword.db";
    let password_hash = bcrypt::hash(password).unwrap();
    let db = SqlitePool::connect(DB_URL).await.unwrap();
    sqlx::query("INSERT INTO users (username, password_hash, email) values (?, ?, ?)")
        .bind(username)
        .bind(password_hash)
        .bind(email)
        .execute(&db).await.unwrap();
}

pub async fn start() -> tokio::io::Result<()> {
    // let bytes = [18, 0, 94, 0, 5, 115, 117, 99, 107, 32, 109, 121, 32, 100, 105, 99, 107, 0];
    // let mut packet = Packet::new(&bytes);
    
    // let header = packet.get_header();
    // println!("header {:?}", header);
    // let sub_header = packet.get_u8();
    // println!("sub_header {:?}", sub_header);
    // let unknown = packet.get_buffer(1);
    // println!("unknown {:?}", unknown);
    // let notice = packet.get_string();
    // println!("notice {:?}", notice);
    create_account("admin", "admin", "admin@admin.com").await;

    Ok(())
}