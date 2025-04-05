use eframe::egui;
use google_cloud_storage::http::objects::upload::{Media, UploadType};
use std::borrow::Cow;
use std::sync::mpsc;

mod handlers;
mod repository;
mod services;

// Nosso aplicativo com canal para comunicação entre a tarefa async e a UI.
struct MyApp {
    url: String,
    object_path: String,
    file_name: String,
    result: String,
    uploading: bool,
    result_tx: mpsc::Sender<String>,
    result_rx: mpsc::Receiver<String>,
}

impl Default for MyApp {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            url: "".to_owned(),
            object_path: "".to_owned(),
            file_name: "".to_owned(),
            result: "".to_owned(),
            uploading: false,
            result_tx: tx,
            result_rx: rx,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Verifica se há mensagens do async (resultado do upload)
        while let Ok(msg) = self.result_rx.try_recv() {
            self.result = msg;
            self.uploading = false;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Upload de imagem para GCS");

            ui.horizontal(|ui| {
                ui.label("URL da imagem:");
                ui.text_edit_singleline(&mut self.url);
            });

            ui.horizontal(|ui| {
                ui.label("Caminho no bucket:");
                ui.text_edit_singleline(&mut self.object_path);
            });

            ui.horizontal(|ui| {
                ui.label("Nome do arquivo local:");
                ui.text_edit_singleline(&mut self.file_name);
            });

            if ui.button("Enviar para o bucket").clicked() && !self.uploading {
                self.uploading = true;
                // Clona os dados para o async block (dados owned)
                let url = self.url.clone();
                let object_path = self.object_path.clone();
                let file_name = self.file_name.clone();
                let tx = self.result_tx.clone();

                // Agora, o async block não captura referências de `self`
                tokio::spawn(async move {
                    let res = async {
                        match handlers::image_hex::download_image(&url).await {
                            Ok(bytes) => {
                                let base64_string =
                                    services::convert_base64::bytes_to_base64(&bytes);
                                services::convert_base64::base64_create_file(
                                    base64_string,
                                    &file_name,
                                )
                                .map_err(|e| format!("Erro ao criar arquivo: {}", e))?;
                                match services::zip_base64::zip_file(&file_name) {
                                    Ok(zip_bytes) => {
                                        // Cria o Media usando Owned para evitar problemas de lifetime
                                        let media = Media {
                                            name: Cow::Owned(file_name.clone()),
                                            content_type: Cow::Borrowed("application/zip"),
                                            content_length: Some(zip_bytes.len() as u64),
                                        };
                                        let upload_type = UploadType::Simple(media);
                                        let uploaded_url = repository::send_gcs::upload_to_gcs(
                                            "testerust",
                                            &object_path,
                                            zip_bytes,
                                            upload_type,
                                        )
                                        .await
                                        .map_err(|e| format!("Erro ao enviar: {}", e))?;
                                        Ok(uploaded_url)
                                    }
                                    Err(e) => Err(format!("Erro ao zipar: {}", e)),
                                }
                            }
                            Err(e) => Err(format!("Erro ao baixar imagem: {}", e)),
                        }
                    }
                    .await;
                    // Envia o resultado para o canal
                    match res {
                        Ok(success_url) => {
                            let _ = tx.send(format!("Enviado com sucesso: {}", success_url));
                        }
                        Err(err_msg) => {
                            let _ = tx.send(err_msg);
                        }
                    }
                });
            }

            ui.separator();
            ui.label("Resultado:");
            ui.label(&self.result);
        });
    }
}

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Uploader para GCS",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}
