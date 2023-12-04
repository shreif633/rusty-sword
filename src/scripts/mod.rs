pub mod create_account;

pub async fn start() -> tokio::io::Result<()> {
    create_account::create_account("admin", "admin", "admin@admin.com").await;

    Ok(())
}