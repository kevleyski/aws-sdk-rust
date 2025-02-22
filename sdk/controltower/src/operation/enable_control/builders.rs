// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::enable_control::_enable_control_output::EnableControlOutputBuilder;

pub use crate::operation::enable_control::_enable_control_input::EnableControlInputBuilder;

/// Fluent builder constructing a request to `EnableControl`.
///
/// <p>This API call activates a control. It starts an asynchronous operation that creates AWS resources on the specified organizational unit and the accounts it contains. The resources created will vary according to the control that you specify.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct EnableControlFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::enable_control::builders::EnableControlInputBuilder,
}
impl EnableControlFluentBuilder {
    /// Creates a new `EnableControl`.
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
            crate::operation::enable_control::EnableControl,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::enable_control::EnableControlError>,
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
        crate::operation::enable_control::EnableControlOutput,
        aws_smithy_http::result::SdkError<crate::operation::enable_control::EnableControlError>,
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
    /// <p>The ARN of the control. Only <b>Strongly recommended</b> and <b>Elective</b> controls are permitted, with the exception of the <b>Region deny</b> guardrail.</p>
    pub fn control_identifier(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.control_identifier(input.into());
        self
    }
    /// <p>The ARN of the control. Only <b>Strongly recommended</b> and <b>Elective</b> controls are permitted, with the exception of the <b>Region deny</b> guardrail.</p>
    pub fn set_control_identifier(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_control_identifier(input);
        self
    }
    /// <p>The ARN of the organizational unit.</p>
    pub fn target_identifier(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.target_identifier(input.into());
        self
    }
    /// <p>The ARN of the organizational unit.</p>
    pub fn set_target_identifier(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_target_identifier(input);
        self
    }
}
