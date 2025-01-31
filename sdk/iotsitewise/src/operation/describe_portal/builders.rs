// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_portal::_describe_portal_output::DescribePortalOutputBuilder;

pub use crate::operation::describe_portal::_describe_portal_input::DescribePortalInputBuilder;

/// Fluent builder constructing a request to `DescribePortal`.
///
/// <p>Retrieves information about a portal.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribePortalFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_portal::builders::DescribePortalInputBuilder,
}
impl DescribePortalFluentBuilder {
    /// Creates a new `DescribePortal`.
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
            crate::operation::describe_portal::DescribePortal,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::describe_portal::DescribePortalError>,
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
        crate::operation::describe_portal::DescribePortalOutput,
        aws_smithy_http::result::SdkError<crate::operation::describe_portal::DescribePortalError>,
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
    /// <p>The ID of the portal.</p>
    pub fn portal_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.portal_id(input.into());
        self
    }
    /// <p>The ID of the portal.</p>
    pub fn set_portal_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_portal_id(input);
        self
    }
}
