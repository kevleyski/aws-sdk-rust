// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_endpoint::_get_endpoint_output::GetEndpointOutputBuilder;

pub use crate::operation::get_endpoint::_get_endpoint_input::GetEndpointInputBuilder;

/// Fluent builder constructing a request to `GetEndpoint`.
///
/// <p>Gets information about an Device Advisor endpoint.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetEndpointFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_endpoint::builders::GetEndpointInputBuilder,
}
impl GetEndpointFluentBuilder {
    /// Creates a new `GetEndpoint`.
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
            crate::operation::get_endpoint::GetEndpoint,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::get_endpoint::GetEndpointError>,
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
        crate::operation::get_endpoint::GetEndpointOutput,
        aws_smithy_http::result::SdkError<crate::operation::get_endpoint::GetEndpointError>,
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
    /// <p>The thing ARN of the device. This is an optional parameter.</p>
    pub fn thing_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.thing_arn(input.into());
        self
    }
    /// <p>The thing ARN of the device. This is an optional parameter.</p>
    pub fn set_thing_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_thing_arn(input);
        self
    }
    /// <p>The certificate ARN of the device. This is an optional parameter.</p>
    pub fn certificate_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.certificate_arn(input.into());
        self
    }
    /// <p>The certificate ARN of the device. This is an optional parameter.</p>
    pub fn set_certificate_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_certificate_arn(input);
        self
    }
}
