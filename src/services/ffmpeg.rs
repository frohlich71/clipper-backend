use std::process::Command;

pub async fn process(input_path: &str, output_path: &str) {
    let output = Command::new("ffmpeg")
        .args(&["-i", input_path, "-vf", "scale=1080:1920", "-preset", "ultrafast", output_path])
        .output().expect("Error processing video");

    if output.status.success() {
        println!("Video processed successfully");
    } else {
        println!("Error processing video");
    }
}