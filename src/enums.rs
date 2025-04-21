pub enum S3Client<'a> {
    Default,
    Client(&'a aws_sdk_s3::Client),
}
