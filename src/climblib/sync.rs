use super::io::{log_index};
use super::utils::{is_climb, is_workout, is_metrics};

use aws_sdk_s3;
use aws_config::defaults;
use aws_config::BehaviorVersion;
use tokio;
use std::collections::HashSet;
use std::error::Error;
use std::path::{PathBuf, Path};
use tracing::{info, error};

#[derive(PartialEq)]
pub enum AwsActions {
    Sync,
    Pull,
}

pub async fn aws_entrypoint(
    action: AwsActions, 
    bucket_name: &str, 
    dry_run: bool) {
    let config = defaults(BehaviorVersion::latest()).load().await;
    let client = aws_sdk_s3::Client::new(&config);

    let remote_keys: HashSet<String> = match list_aws_files(bucket_name, &client).await {
        Ok(resp) => resp
            .contents()
            .iter()
            .filter_map(|o| o.key().map(|s| s.to_string()))
            .collect(),
        Err(e) => {
            error!("Error listing S3 keys: {e}");
            return;
        }
    };

    let local_paths: HashSet<String> = match log_index() {
        Ok(paths) => paths
                .iter()
                .filter_map(|p| p.file_name()?.to_str().map(|s| s.to_string()))
                .collect(),
        Err(e) => {
            error!("Error getting local logs: {e}");
            return;
        }
    };

    if action == AwsActions::Pull {
        pull(bucket_name, &client, dry_run, &remote_keys, &local_paths).await;
    }
    else if action == AwsActions::Sync {
        sync(bucket_name, &client, dry_run, &remote_keys, &local_paths).await;
    }
}

async fn download_log_from_s3(
    bucket_name: &str, 
    path: &Path,
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
    client: &aws_sdk_s3::Client
) -> Result<aws_sdk_s3::operation::list_objects_v2::ListObjectsV2Output, Box<dyn Error>> {
    let response = client
                    .list_objects_v2()
                    .bucket(bucket_name)
                    .send()
                    .await?;
    Ok(response)
}

async fn upload_log_to_s3(
    bucket_name: &str, 
    key: &str, 
    path: &Path, 
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

async fn pull(
    bucket_name: &str, 
    client: &aws_sdk_s3::Client, 
    dry_run: bool,
    remote_keys: &HashSet<String>,
    local_paths: &HashSet<String>) {

    for key in remote_keys {
        let filename = key.split('/').last().unwrap_or("unknown.json").to_string();
        let exists_locally = local_paths.contains(&filename);
        if !exists_locally {
            let mut local_path = PathBuf::from("logs");
            local_path.push(filename);
            if dry_run {
                info!("Would download {}", &key);
            }
            else {
                match download_log_from_s3(bucket_name, &local_path, &key, client).await {
                    Ok(()) => info!("Downloaded {}", &key),
                    Err(e) => error!("error {e} downloading {}", &key)
                }
            }
        }
    }
}

async fn sync(
    bucket_name: &str, 
    client: &aws_sdk_s3::Client, 
    dry_run: bool,
    remote_keys: &HashSet<String>,
    local_paths: &HashSet<String>) {

    for filename in local_paths {
        let mut path = PathBuf::from("logs");
        path.push(filename);
        let bucket_path = if is_climb(&path) {
            format!("climbs/{}", filename)
        } else if is_workout(&path) {
            format!("workouts/{}", filename)
        } else if is_metrics(&path) {
            format!("metrics/{}", filename)
        } else {
            continue;
        };
        let exists_remotely = remote_keys.contains(&bucket_path);
        if !exists_remotely {
            if dry_run {
                info!("Would upload {}", bucket_path);
            }
            else {
                match upload_log_to_s3(bucket_name, &bucket_path, &path, client).await {
                    Ok(_) => info!("Uploaded {}", bucket_path),
                    Err(e) => error!("error {e} uploading {}", bucket_path),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use tracing_test::traced_test;
    
    #[traced_test]
    #[tokio::test]
    async fn test_sync_dry_run_logs_expected_uploads() {
        let local_paths = HashSet::from([
            "2024-04-01_climb.json".to_string(),
            "2024-04-02_workout.json".to_string(),
        ]);

        let remote_keys = HashSet::from([
            "climbs/2024-04-01_climb.json".to_string(),
        ]);    
        
        let config = defaults(BehaviorVersion::latest()).load().await;
        let client = aws_sdk_s3::Client::new(&config);

        sync("test-bucket", &client, true, &remote_keys, &local_paths).await;

        assert!(logs_contain("Would upload workouts/2024-04-02_workout.json"));
        assert!(!logs_contain("climbs/2024-04-01_climb.json"));
    }

    #[traced_test]
    #[tokio::test]
    async fn test_pull_dry_run_logs_expected_uploads() {
        let local_paths = HashSet::from([
            "2024-04-01_climb.json".to_string(),
        ]);

        let remote_keys = HashSet::from([
            "climbs/2024-04-01_climb.json".to_string(),
            "workouts/2024-04-02_workout.json".to_string(),
        ]);    
        
        let config = defaults(BehaviorVersion::latest()).load().await;
        let client = aws_sdk_s3::Client::new(&config);

        pull("test-bucket", &client, true, &remote_keys, &local_paths).await;

        assert!(logs_contain("Would download workouts/2024-04-02_workout.json"));
        assert!(!logs_contain("climbs/2024-04-01_climb.json"));
    }
}