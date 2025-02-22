// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains information about where to publish the analytics results.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct AnalyticsS3BucketDestination {
    /// <p>Specifies the file format used when exporting data to Amazon S3.</p>
    #[doc(hidden)]
    pub format: std::option::Option<crate::types::AnalyticsS3ExportFileFormat>,
    /// <p>The account ID that owns the destination S3 bucket. If no account ID is provided, the owner is not validated before exporting data.</p> <note>
    /// <p> Although this value is optional, we strongly recommend that you set it to help prevent problems if the destination bucket ownership changes. </p>
    /// </note>
    #[doc(hidden)]
    pub bucket_account_id: std::option::Option<std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the bucket to which data is exported.</p>
    #[doc(hidden)]
    pub bucket: std::option::Option<std::string::String>,
    /// <p>The prefix to use when exporting data. The prefix is prepended to all results.</p>
    #[doc(hidden)]
    pub prefix: std::option::Option<std::string::String>,
}
impl AnalyticsS3BucketDestination {
    /// <p>Specifies the file format used when exporting data to Amazon S3.</p>
    pub fn format(&self) -> std::option::Option<&crate::types::AnalyticsS3ExportFileFormat> {
        self.format.as_ref()
    }
    /// <p>The account ID that owns the destination S3 bucket. If no account ID is provided, the owner is not validated before exporting data.</p> <note>
    /// <p> Although this value is optional, we strongly recommend that you set it to help prevent problems if the destination bucket ownership changes. </p>
    /// </note>
    pub fn bucket_account_id(&self) -> std::option::Option<&str> {
        self.bucket_account_id.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the bucket to which data is exported.</p>
    pub fn bucket(&self) -> std::option::Option<&str> {
        self.bucket.as_deref()
    }
    /// <p>The prefix to use when exporting data. The prefix is prepended to all results.</p>
    pub fn prefix(&self) -> std::option::Option<&str> {
        self.prefix.as_deref()
    }
}
impl AnalyticsS3BucketDestination {
    /// Creates a new builder-style object to manufacture [`AnalyticsS3BucketDestination`](crate::types::AnalyticsS3BucketDestination).
    pub fn builder() -> crate::types::builders::AnalyticsS3BucketDestinationBuilder {
        crate::types::builders::AnalyticsS3BucketDestinationBuilder::default()
    }
}

/// A builder for [`AnalyticsS3BucketDestination`](crate::types::AnalyticsS3BucketDestination).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct AnalyticsS3BucketDestinationBuilder {
    pub(crate) format: std::option::Option<crate::types::AnalyticsS3ExportFileFormat>,
    pub(crate) bucket_account_id: std::option::Option<std::string::String>,
    pub(crate) bucket: std::option::Option<std::string::String>,
    pub(crate) prefix: std::option::Option<std::string::String>,
}
impl AnalyticsS3BucketDestinationBuilder {
    /// <p>Specifies the file format used when exporting data to Amazon S3.</p>
    pub fn format(mut self, input: crate::types::AnalyticsS3ExportFileFormat) -> Self {
        self.format = Some(input);
        self
    }
    /// <p>Specifies the file format used when exporting data to Amazon S3.</p>
    pub fn set_format(
        mut self,
        input: std::option::Option<crate::types::AnalyticsS3ExportFileFormat>,
    ) -> Self {
        self.format = input;
        self
    }
    /// <p>The account ID that owns the destination S3 bucket. If no account ID is provided, the owner is not validated before exporting data.</p> <note>
    /// <p> Although this value is optional, we strongly recommend that you set it to help prevent problems if the destination bucket ownership changes. </p>
    /// </note>
    pub fn bucket_account_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.bucket_account_id = Some(input.into());
        self
    }
    /// <p>The account ID that owns the destination S3 bucket. If no account ID is provided, the owner is not validated before exporting data.</p> <note>
    /// <p> Although this value is optional, we strongly recommend that you set it to help prevent problems if the destination bucket ownership changes. </p>
    /// </note>
    pub fn set_bucket_account_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.bucket_account_id = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the bucket to which data is exported.</p>
    pub fn bucket(mut self, input: impl Into<std::string::String>) -> Self {
        self.bucket = Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the bucket to which data is exported.</p>
    pub fn set_bucket(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.bucket = input;
        self
    }
    /// <p>The prefix to use when exporting data. The prefix is prepended to all results.</p>
    pub fn prefix(mut self, input: impl Into<std::string::String>) -> Self {
        self.prefix = Some(input.into());
        self
    }
    /// <p>The prefix to use when exporting data. The prefix is prepended to all results.</p>
    pub fn set_prefix(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.prefix = input;
        self
    }
    /// Consumes the builder and constructs a [`AnalyticsS3BucketDestination`](crate::types::AnalyticsS3BucketDestination).
    pub fn build(self) -> crate::types::AnalyticsS3BucketDestination {
        crate::types::AnalyticsS3BucketDestination {
            format: self.format,
            bucket_account_id: self.bucket_account_id,
            bucket: self.bucket,
            prefix: self.prefix,
        }
    }
}
