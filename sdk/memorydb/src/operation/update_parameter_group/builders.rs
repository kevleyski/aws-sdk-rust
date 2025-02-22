// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_parameter_group::_update_parameter_group_output::UpdateParameterGroupOutputBuilder;

pub use crate::operation::update_parameter_group::_update_parameter_group_input::UpdateParameterGroupInputBuilder;

/// Fluent builder constructing a request to `UpdateParameterGroup`.
///
/// <p>Updates the parameters of a parameter group. You can modify up to 20 parameters in a single request by submitting a list parameter name and value pairs.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateParameterGroupFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_parameter_group::builders::UpdateParameterGroupInputBuilder,
}
impl UpdateParameterGroupFluentBuilder {
    /// Creates a new `UpdateParameterGroup`.
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
            crate::operation::update_parameter_group::UpdateParameterGroup,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::update_parameter_group::UpdateParameterGroupError,
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
        crate::operation::update_parameter_group::UpdateParameterGroupOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::update_parameter_group::UpdateParameterGroupError,
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
    /// <p>The name of the parameter group to update.</p>
    pub fn parameter_group_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.parameter_group_name(input.into());
        self
    }
    /// <p>The name of the parameter group to update.</p>
    pub fn set_parameter_group_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_parameter_group_name(input);
        self
    }
    /// Appends an item to `ParameterNameValues`.
    ///
    /// To override the contents of this collection use [`set_parameter_name_values`](Self::set_parameter_name_values).
    ///
    /// <p>An array of parameter names and values for the parameter update. You must supply at least one parameter name and value; subsequent arguments are optional. A maximum of 20 parameters may be updated per request.</p>
    pub fn parameter_name_values(mut self, input: crate::types::ParameterNameValue) -> Self {
        self.inner = self.inner.parameter_name_values(input);
        self
    }
    /// <p>An array of parameter names and values for the parameter update. You must supply at least one parameter name and value; subsequent arguments are optional. A maximum of 20 parameters may be updated per request.</p>
    pub fn set_parameter_name_values(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::ParameterNameValue>>,
    ) -> Self {
        self.inner = self.inner.set_parameter_name_values(input);
        self
    }
}
