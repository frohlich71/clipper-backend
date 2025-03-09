use axum::{Json, Router};
use axum::extract::DefaultBodyLimit;
use axum_extra::extract::Multipart;
use axum::routing::post;
use serde_json::json;
use tokio::io::AsyncWriteExt;

pub mod services;
use services::ffmpeg::process;

async fn upload_video(mut multipart: Multipart) -> Json<serde_json::Value> {
    let mut output_file = String::new();

    while let Ok(Some(field)) = multipart.next_field().await {
        let file_name = match field.file_name() {
            Some(name) => name.to_string(),
            None => return Json(json!({ "message": "error", "error": "Arquivo sem nome" })),
        };

        let input_path = format!("./uploads/{}", file_name);

        let data = match field.bytes().await {
            Ok(data) => data,
            Err(e) => {
                return Json(json!({ "message": "error", "error": format!("Falha ao ler o arquivo: {}", e) }));
            }
        };

        let mut file = match tokio::fs::File::create(&input_path).await {
            Ok(file) => file,
            Err(e) => {
                return Json(json!({ "message": "error", "error": format!("Falha ao criar o arquivo: {}", e) }));
            }
        };

        if let Err(e) = file.write_all(&data).await {
            return Json(json!({ "message": "error", "error": format!("Falha ao salvar o arquivo: {}", e) }));
        }

        let output_path = format!("./processed/{}", file_name);
        output_file = output_path.clone();

        if let Err(e) = process(&input_path, &output_path).await {
            return Json(json!({ "message": "error", "error": format!("Falha ao processar vídeo: {}", e) }));
        }
    }

    Json(json!({ "message": "success", "output_file": output_file }))
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/upload", post(upload_video).layer(DefaultBodyLimit::max(500 * 1024 * 1024)));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}



