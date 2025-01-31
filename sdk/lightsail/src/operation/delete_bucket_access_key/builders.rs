// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_bucket_access_key::_delete_bucket_access_key_output::DeleteBucketAccessKeyOutputBuilder;

pub use crate::operation::delete_bucket_access_key::_delete_bucket_access_key_input::DeleteBucketAccessKeyInputBuilder;

/// Fluent builder constructing a request to `DeleteBucketAccessKey`.
///
/// <p>Deletes an access key for the specified Amazon Lightsail bucket.</p>
/// <p>We recommend that you delete an access key if the secret access key is compromised.</p>
/// <p>For more information about access keys, see <a href="https://lightsail.aws.amazon.com/ls/docs/en_us/articles/amazon-lightsail-creating-bucket-access-keys">Creating access keys for a bucket in Amazon Lightsail</a> in the <i>Amazon Lightsail Developer Guide</i>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteBucketAccessKeyFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_bucket_access_key::builders::DeleteBucketAccessKeyInputBuilder,
}
impl DeleteBucketAccessKeyFluentBuilder {
    /// Creates a new `DeleteBucketAccessKey`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::delete_bucket_access_key::DeleteBucketAccessKey,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_bucket_access_key::DeleteBucketAccessKeyError,
        >,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        Ok(crate::client::customize::CustomizableOperation { handle, operation })
    }

    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> std::result::Result<
        crate::operation::delete_bucket_access_key::DeleteBucketAccessKeyOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_bucket_access_key::DeleteBucketAccessKeyError,
        >,
    > {
        let op = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// <p>The name of the bucket that the access key belongs to.</p>
    pub fn bucket_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.bucket_name(input.into());
        self
    }
    /// <p>The name of the bucket that the access key belongs to.</p>
    pub fn set_bucket_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_bucket_name(input);
        self
    }
    /// <p>The ID of the access key to delete.</p>
    /// <p>Use the <a href="https://docs.aws.amazon.com/lightsail/2016-11-28/api-reference/API_GetBucketAccessKeys.html">GetBucketAccessKeys</a> action to get a list of access key IDs that you can specify.</p>
    pub fn access_key_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.access_key_id(input.into());
        self
    }
    /// <p>The ID of the access key to delete.</p>
    /// <p>Use the <a href="https://docs.aws.amazon.com/lightsail/2016-11-28/api-reference/API_GetBucketAccessKeys.html">GetBucketAccessKeys</a> action to get a list of access key IDs that you can specify.</p>
    pub fn set_access_key_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_access_key_id(input);
        self
    }
}
