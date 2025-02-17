use axum::{
    body::Body,
    extract::Request,
    middleware::Next,
    response::IntoResponse,
};
use tokio::fs::OpenOptions;
use tokio::io::{AsyncWriteExt, Result};
use chrono::Local;

pub async fn log_request(req: Request<Body>, next: Next) -> impl IntoResponse {
    let method = req.method().to_string();
    let url = req.uri().to_string();

    let timestamp = Local::now().to_rfc3339();

    let log_message = format!(
        "[{}] {} {}\n", 
        timestamp, 
        method, 
        url
    );

    if let Err(e) = write_log(&log_message).await {
        eprintln!("Failed to write log: {}", e);
    }

    next.run(req).await
}

async fn write_log(log_message: &str) -> Result<()> {
    let file_path = "log.txt"; 
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .await?;

    file.write_all(log_message.as_bytes()).await?;
    Ok(())
}
