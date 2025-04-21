use file_diff::diff;
use std::path::Path;

#[tokio::test]
async fn test_put() {
    let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .load()
        .await;

    let client = aws_sdk_s3::Client::new(&config);

    let put_result = s3x::put(
        s3x::enums::S3Client::Default,
        Path::new("README.md"),
        "sdfsdfgdfg",
    )
    .await;

    assert!(put_result.is_ok());

    let get_result = s3x::get(
        s3x::enums::S3Client::Client(&client),
        "sdfsdfgdfg",
        "foo",
        Path::new("README.new"),
    )
    .await;

    assert!(get_result.is_ok());

    assert!(diff("README.md", "README.new"));
}
