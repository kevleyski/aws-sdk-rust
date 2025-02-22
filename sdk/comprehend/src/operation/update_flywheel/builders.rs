// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_flywheel::_update_flywheel_output::UpdateFlywheelOutputBuilder;

pub use crate::operation::update_flywheel::_update_flywheel_input::UpdateFlywheelInputBuilder;

/// Fluent builder constructing a request to `UpdateFlywheel`.
///
/// <p>Update the configuration information for an existing flywheel.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateFlywheelFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_flywheel::builders::UpdateFlywheelInputBuilder,
}
impl UpdateFlywheelFluentBuilder {
    /// Creates a new `UpdateFlywheel`.
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
            crate::operation::update_flywheel::UpdateFlywheel,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::update_flywheel::UpdateFlywheelError>,
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
        crate::operation::update_flywheel::UpdateFlywheelOutput,
        aws_smithy_http::result::SdkError<crate::operation::update_flywheel::UpdateFlywheelError>,
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
    /// <p>The Amazon Resource Number (ARN) of the flywheel to update.</p>
    pub fn flywheel_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.flywheel_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Number (ARN) of the flywheel to update.</p>
    pub fn set_flywheel_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_flywheel_arn(input);
        self
    }
    /// <p>The Amazon Resource Number (ARN) of the active model version.</p>
    pub fn active_model_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.active_model_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Number (ARN) of the active model version.</p>
    pub fn set_active_model_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_active_model_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role that grants Amazon Comprehend permission to access the flywheel data.</p>
    pub fn data_access_role_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.data_access_role_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role that grants Amazon Comprehend permission to access the flywheel data.</p>
    pub fn set_data_access_role_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_data_access_role_arn(input);
        self
    }
    /// <p>Flywheel data security configuration.</p>
    pub fn data_security_config(mut self, input: crate::types::UpdateDataSecurityConfig) -> Self {
        self.inner = self.inner.data_security_config(input);
        self
    }
    /// <p>Flywheel data security configuration.</p>
    pub fn set_data_security_config(
        mut self,
        input: std::option::Option<crate::types::UpdateDataSecurityConfig>,
    ) -> Self {
        self.inner = self.inner.set_data_security_config(input);
        self
    }
}
