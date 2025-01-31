// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_snowball_usage::_get_snowball_usage_output::GetSnowballUsageOutputBuilder;

pub use crate::operation::get_snowball_usage::_get_snowball_usage_input::GetSnowballUsageInputBuilder;

/// Fluent builder constructing a request to `GetSnowballUsage`.
///
/// <p>Returns information about the Snow Family service limit for your account, and also the number of Snow devices your account has in use.</p>
/// <p>The default service limit for the number of Snow devices that you can have at one time is 1. If you want to increase your service limit, contact Amazon Web Services Support.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetSnowballUsageFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_snowball_usage::builders::GetSnowballUsageInputBuilder,
}
impl GetSnowballUsageFluentBuilder {
    /// Creates a new `GetSnowballUsage`.
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
            crate::operation::get_snowball_usage::GetSnowballUsage,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::get_snowball_usage::GetSnowballUsageError,
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
        crate::operation::get_snowball_usage::GetSnowballUsageOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::get_snowball_usage::GetSnowballUsageError,
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
}
