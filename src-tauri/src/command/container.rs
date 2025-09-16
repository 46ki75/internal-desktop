use std::process::Stdio;
use tokio::process::Command;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RunContainerRequest {
    pub image: String,
    pub command: Vec<String>,
}

#[tauri::command]
pub async fn run_container(request: RunContainerRequest) -> Result<String, String> {
    // シンプルな実装: runc を呼び出す
    // runc がインストールされている必要がある
    let output = Command::new("runc")
        .arg("run")
        .arg("--rm")  // ワンショット
        .arg(&request.image)
        .args(&request.command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| format!("Failed to run container: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if output.status.success() {
        Ok(stdout.to_string())
    } else {
        Err(format!("Container failed: {}", stderr))
    }
}