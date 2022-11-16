mod challenge01;
mod challenge02;

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    challenge01::result().await?;
    println!("");
    challenge02::result().await?;
    Ok(())
}
