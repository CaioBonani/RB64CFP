use google_cloud_auth::credentials::CredentialsFile;
use google_cloud_storage::client::{Client, ClientConfig};
use google_cloud_storage::http::objects::upload::{UploadObjectRequest, UploadType};

pub async fn upload_to_gcs(
    bucket: &str,
    object_path: &str,
    content: Vec<u8>,
    content_type: UploadType,
) -> Result<String, Box<dyn std::error::Error>> {
    // lê as credenciais a partir da variável de ambiente

    let credentials = CredentialsFile::new_from_file("../../cred.json".to_owned()).await?;
    let config = ClientConfig::default()
        .with_credentials(credentials)
        .await?;
    let client = Client::new(config);

    let req = UploadObjectRequest {
        bucket: bucket.to_string(),
        ..Default::default()
    };

    // envia o conteúdo para o bucket
    client.upload_object(&req, content, &content_type).await?;

    let url = format!("https://storage.googleapis.com/{}/{}", bucket, object_path);
    Ok(url)
}
