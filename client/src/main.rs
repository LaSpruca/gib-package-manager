#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let client = reqwest::Client::builder().danger_accept_invalid_certs(true).build()?;
    let response = client.get("https://localhost:5000")
        .send()
        .await?
        .text()
        .await?;

    println!("{}", response);

    Ok(())
}