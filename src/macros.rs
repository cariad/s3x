#[macro_export]
macro_rules! resolve_client {
    ($client:expr) => {
        match $client {
            enums::S3Client::Default => {
                let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
                    .load()
                    .await;

                &aws_sdk_s3::Client::new(&config)
            }
            enums::S3Client::Client(c) => c,
        }
    };
}
