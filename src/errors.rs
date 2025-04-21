/// An error that occurred when getting an object from S3.
#[derive(Debug)]
pub enum GetError {
    File(tokio::io::Error),
    S3(aws_sdk_s3::error::SdkError<aws_sdk_s3::operation::get_object::GetObjectError>),
    Response(aws_smithy_types::byte_stream::error::Error),
}

/// An error that occurred when putting an object into S3.
#[derive(Debug)]
pub enum PutError {
    File(tokio::io::Error),
    S3(aws_sdk_s3::error::SdkError<aws_sdk_s3::operation::put_object::PutObjectError>),
}
