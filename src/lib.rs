//! s3x: faster S3 upload & download.

pub mod enums;
pub mod errors;
mod macros;

use std::path::Path;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn get(
    client: enums::S3Client<'_>,
    bucket: &str,
    key: &str,
    destination: &Path,
) -> Result<(), errors::GetError> {
    let resolved_client = resolve_client!(client);
    let response = resolved_client
        .get_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await
        .map_err(errors::GetError::S3)?;

    let mut file = File::create(&destination)
        .await
        .map_err(errors::GetError::File)?;

    let body = response
        .body
        .collect()
        .await
        .map_err(errors::GetError::Response)?
        .into_bytes();

    file.write_all(&body).await.expect("Failed to write file");

    Ok(())
}

pub async fn put(
    client: enums::S3Client<'_>,
    source: &std::path::Path,
    bucket: &str,
) -> Result<(), errors::PutError> {
    let resolved_client = resolve_client!(client);
    let mut file = File::open(&source).await.map_err(errors::PutError::File)?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)
        .await
        .map_err(errors::PutError::File)?;

    resolved_client
        .put_object()
        .body(buffer.into())
        .bucket(bucket)
        .key("foo")
        .send()
        .await
        .map_err(errors::PutError::S3)?;

    Ok(())
}
