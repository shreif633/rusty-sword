pub mod create_account;

pub async fn start() -> tokio::io::Result<()> {
    let _ = create_account::create_account("admin", "admin", "admin@admin.com");

    Ok(())
}