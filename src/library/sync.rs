use super::io::{log_index};
use aws_sdk_s3;
use aws_config::defaults;
use aws_config::BehaviorVersion;
use tokio;
use std::error::Error;

async fn sync_logs_to_s3(bucket_name: &str, bucket_path: &str, path: &str) -> Result<(), Box<dyn Error>> {
    let config = defaults(BehaviorVersion::latest()).load().await;
    let client = aws_sdk_s3::Client::new(&config);

    let body = tokio::fs::read(path).await?;

    client.put_object()
    .bucket(bucket_name)
    .key(bucket_path)
    .body(body.into())
    .send()
    .await?;

    Ok(())
}

pub async fn sync() {
    let bucket_name = "my-climblog-bucket";
    match log_index() {
        Ok(paths) => {
            for path in paths {
                if let Some(filename) = path.file_name().and_then(|f| f.to_str()){
                    let mut bucket_path = String::new();
                    let full_path = "logs/".to_string() + &filename;
                    if filename.contains("climb") {
                        bucket_path = format!("climbs/{}", filename);
                    } else if filename.contains("workout") {
                        bucket_path = format!("workouts/{}", filename);
                    } else if filename.contains("metrics") {
                        bucket_path = format!("metrics/{}", filename);
                    }
                    match sync_logs_to_s3(&bucket_name, &bucket_path, &full_path).await {
                        Ok(()) => println!("Uploaded {}", &bucket_path),
                        Err(e) => eprintln!("error {e} uploading {}", &bucket_path)
                    }
                }
            }
        }
        Err(e) => eprintln!("error {e} getting paths"),
    }
}