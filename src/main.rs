mod handlers;
mod repository;
mod services;

use google_cloud_storage::http::objects::upload::{Media, UploadType};

#[tokio::main]
async fn main() {
    const URL: &str = "https://picviewer.umov.me/Pic/GetImage?id=806365645&token=9fec638e362564360fe270a1e775170c";

    match handlers::image_hex::download_image(URL).await {
        Ok(bytes) => {
            let base64_string = services::convert_base64::bytes_to_base64(&bytes);

            services::convert_base64::base64_create_file(base64_string, "teste").unwrap();

            let zip_bytes = services::zip_base64::zip_file("./teste").unwrap();

            let upload_type = UploadType::Simple(Media::new("imagesUMOV/teste.zip"));

            match repository::send_gcs::upload_to_gcs(
                "testerust",
                "imagesUMOV/teste.zip",
                zip_bytes,
                upload_type,
            )
            .await
            {
                Ok(ok) => {
                    println!("{}", ok);
                }
                Err(e) => {
                    eprintln!("Erro: {}", e);
                    return;
                }
            }
        }
        Err(e) => {
            eprintln!("Erro ao baixar imagem: {}", e);
            return;
        }
    }
}
