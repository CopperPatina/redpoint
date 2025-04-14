use super::io::{log_index};
use super::utils::{is_climb, is_workout, is_metrics};

use aws_sdk_s3;
use aws_config::defaults;
use aws_config::BehaviorVersion;
use tokio;
use std::error::Error;
use std::path::PathBuf;

#[derive(PartialEq)]
pub enum AwsActions {
    Sync,
    Pull,
}

pub async fn aws_entrypoint(action: AwsActions, bucket_name: &str) {
    let config = defaults(BehaviorVersion::latest()).load().await;
    let client = aws_sdk_s3::Client::new(&config);

    if action == AwsActions::Pull {
        pull(bucket_name, &client).await;
    }
    else if action == AwsActions::Sync {
        sync(bucket_name, &client).await;
    }
}

async fn download_log_from_s3(
    bucket_name: &str, 
    path: &PathBuf,
    key: &str, 
    client: &aws_sdk_s3::Client
) -> Result<(), Box<dyn Error>> {
    let response = client
                    .get_object()
                    .bucket(bucket_name)
                    .key(key)
                    .send()
                    .await?;

    let data = response.body.collect().await?.into_bytes();
    tokio::fs::write(path, data).await?;

    Ok(())
}

async fn list_aws_files(
    bucket_name: &str, 
    prefix: &str, 
    client: &aws_sdk_s3::Client
) -> Result<aws_sdk_s3::operation::list_objects_v2::ListObjectsV2Output, Box<dyn Error>> {
    let response = client
                    .list_objects_v2()
                    .bucket(bucket_name)
                    .prefix(prefix)
                    .send()
                    .await?;
    Ok(response)
}

async fn upload_log_to_s3(
    bucket_name: &str, 
    key: &str, 
    path: &PathBuf, 
    client: &aws_sdk_s3::Client
) -> Result<(), Box<dyn Error>> {

    let body = tokio::fs::read(path).await?;

    client.put_object()
    .bucket(bucket_name)
    .key(key)
    .body(body.into())
    .send()
    .await?;

    Ok(())
}

async fn pull(bucket_name: &str, client: &aws_sdk_s3::Client) {
    let existing_paths = match log_index() {
        Ok(paths) => paths,
        Err(e) => {
            eprintln!("error {e} getting paths");
            return;
        }
    };

    match list_aws_files(bucket_name, "", client).await {
        Ok(response) => {
            for object in response.contents() {
                if let Some(key) = object.key() {
                    let filename = key.split('/').last().unwrap_or("unknown.json").to_string();
                    let exists_locally = existing_paths.iter().any(|path| {
                        path.file_name()
                         .and_then(|f| f.to_str())
                         .map_or(false, |name| name == filename)
                    });
                    if !exists_locally {
                        let mut local_path = PathBuf::from("logs");
                        local_path.push(filename);
                        match download_log_from_s3(bucket_name, &local_path, &key, client).await {
                            Ok(()) => println!("Downloaded {}", &key),
                            Err(e) => eprintln!("error {e} downloading {}", &key)
                        }
                    }
                }
            }
        },
        Err(e) => eprintln!("Error pulling files {}", {e})
    }
}

async fn sync(bucket_name: &str, client: &aws_sdk_s3::Client) {
    match log_index() {
        Ok(paths) => {
            for path in paths {
                if let Some(filename) = path.file_name().and_then(|f| f.to_str()) {
                    let bucket_path = if is_climb(&path) {
                        format!("climbs/{}", filename)
                    } else if is_workout(&path) {
                        format!("workouts/{}", filename)
                    } else if is_metrics(&path) {
                        format!("metrics/{}", filename)
                    } else {
                        continue;
                    };

                    match upload_log_to_s3(bucket_name, &bucket_path, &path, client).await {
                        Ok(_) => println!("Uploaded {}", bucket_path),
                        Err(e) => eprintln!("error {e} uploading {}", bucket_path),
                    }
                }
            }
        }
        Err(e) => eprintln!("error {e} getting paths"),
    }
}