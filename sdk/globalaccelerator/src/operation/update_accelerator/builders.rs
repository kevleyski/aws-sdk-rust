// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_accelerator::_update_accelerator_output::UpdateAcceleratorOutputBuilder;

pub use crate::operation::update_accelerator::_update_accelerator_input::UpdateAcceleratorInputBuilder;

/// Fluent builder constructing a request to `UpdateAccelerator`.
///
/// <p>Update an accelerator. </p> <important>
/// <p>Global Accelerator is a global service that supports endpoints in multiple Amazon Web Services Regions but you must specify the US West (Oregon) Region to create, update, or otherwise work with accelerators. That is, for example, specify <code>--region us-west-2</code> on AWS CLI commands.</p>
/// </important>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateAcceleratorFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_accelerator::builders::UpdateAcceleratorInputBuilder,
}
impl UpdateAcceleratorFluentBuilder {
    /// Creates a new `UpdateAccelerator`.
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
            crate::operation::update_accelerator::UpdateAccelerator,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::update_accelerator::UpdateAcceleratorError,
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
        crate::operation::update_accelerator::UpdateAcceleratorOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::update_accelerator::UpdateAcceleratorError,
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
    /// <p>The Amazon Resource Name (ARN) of the accelerator to update.</p>
    pub fn accelerator_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.accelerator_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the accelerator to update.</p>
    pub fn set_accelerator_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_accelerator_arn(input);
        self
    }
    /// <p>The name of the accelerator. The name can have a maximum of 64 characters, must contain only alphanumeric characters, periods (.), or hyphens (-), and must not begin or end with a hyphen or period.</p>
    pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the accelerator. The name can have a maximum of 64 characters, must contain only alphanumeric characters, periods (.), or hyphens (-), and must not begin or end with a hyphen or period.</p>
    pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The IP address type that an accelerator supports. For a standard accelerator, the value can be IPV4 or DUAL_STACK.</p>
    pub fn ip_address_type(mut self, input: crate::types::IpAddressType) -> Self {
        self.inner = self.inner.ip_address_type(input);
        self
    }
    /// <p>The IP address type that an accelerator supports. For a standard accelerator, the value can be IPV4 or DUAL_STACK.</p>
    pub fn set_ip_address_type(
        mut self,
        input: std::option::Option<crate::types::IpAddressType>,
    ) -> Self {
        self.inner = self.inner.set_ip_address_type(input);
        self
    }
    /// <p>Indicates whether an accelerator is enabled. The value is true or false. The default value is true. </p>
    /// <p>If the value is set to true, the accelerator cannot be deleted. If set to false, the accelerator can be deleted.</p>
    pub fn enabled(mut self, input: bool) -> Self {
        self.inner = self.inner.enabled(input);
        self
    }
    /// <p>Indicates whether an accelerator is enabled. The value is true or false. The default value is true. </p>
    /// <p>If the value is set to true, the accelerator cannot be deleted. If set to false, the accelerator can be deleted.</p>
    pub fn set_enabled(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_enabled(input);
        self
    }
}
