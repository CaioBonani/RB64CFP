use hex;
use reqwest::get;

pub async fn download_image(url: &str) -> Result<Vec<u8>, reqwest::Error> {
    let response = get(url).await?;
    let bytes = response.bytes().await?;
    Ok(bytes.to_vec())
}
