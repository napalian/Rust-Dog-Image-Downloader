use reqwest;
use std::fs::File;
use std::io::Write;
use std::thread;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get("https://dog.ceo/api/breeds/image/random")
        .await?
        .json::<serde_json::Value>()
        .await?;
    let dog_image = response["message"].as_str().ok_or("No image URL found")?;

    let dog_bytes_response = reqwest::get(dog_image).await?;
    let img_bytes = dog_bytes_response.bytes().await?;
    let mut file = File::create("dog.jpg")?;
    file.write_all(&img_bytes)?;

    Ok(())
}