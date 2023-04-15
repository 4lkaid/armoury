use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let _worker_guard = armoury::init().await?;
    Ok(())
}
