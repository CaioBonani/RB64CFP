mod handlers;
mod services;

#[tokio::main]
async fn main() {
    const URL: &str = "https://picviewer.umov.me/Pic/GetImage?id=806365645&token=9fec638e362564360fe270a1e775170c";

    match handlers::image_hex::download_image(URL).await {
        Ok(bytes) => {
            let base64_string = services::convert_base64::bytes_to_base64(&bytes);

            services::convert_base64::base64_create_file(base64_string, "teste").unwrap();

            services::zip_base64::zip_file("./teste", "./teste.zip").unwrap();
        }
        Err(e) => {
            eprintln!("Erro ao baixar imagem: {}", e);
            return;
        }
    }
}
