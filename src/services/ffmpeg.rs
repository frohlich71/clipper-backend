use std::process::{Command, Output};
use std::fs;
use std::io;

pub async fn process(input: &str, output: &str) -> Result<(), io::Error> {
    // Obtém o diretório de saída a partir do caminho do arquivo de saída
    let output_dir = std::path::Path::new(output)
        .parent()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Caminho de saída inválido"))?;

    // Verifica se o diretório de saída existe, se não, cria
    if !output_dir.exists() {
        fs::create_dir_all(output_dir)?;
    }

    let output_result: Output = Command::new("ffmpeg")
        .args(["-i", input, "-vf", "scale=1080:1920", "-preset", "ultrafast", output])
        .output()?;

    if output_result.status.success() {
        println!("✅ Vídeo processado com sucesso: {}", output);
        Ok(())
    } else {
        let error_message = String::from_utf8_lossy(&output_result.stderr);
        eprintln!("❌ Erro no processamento do vídeo: {}", error_message);
        Err(io::Error::new(io::ErrorKind::Other, format!("FFmpeg error: {}", error_message)))
    }
}