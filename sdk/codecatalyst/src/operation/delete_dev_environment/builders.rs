// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_dev_environment::_delete_dev_environment_output::DeleteDevEnvironmentOutputBuilder;

pub use crate::operation::delete_dev_environment::_delete_dev_environment_input::DeleteDevEnvironmentInputBuilder;

/// Fluent builder constructing a request to `DeleteDevEnvironment`.
///
/// <p>Deletes a Dev Environment. </p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteDevEnvironmentFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_dev_environment::builders::DeleteDevEnvironmentInputBuilder,
}
impl DeleteDevEnvironmentFluentBuilder {
    /// Creates a new `DeleteDevEnvironment`.
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
            crate::operation::delete_dev_environment::DeleteDevEnvironment,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_dev_environment::DeleteDevEnvironmentError,
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
        crate::operation::delete_dev_environment::DeleteDevEnvironmentOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_dev_environment::DeleteDevEnvironmentError,
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
    /// <p>The name of the space.</p>
    pub fn space_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.space_name(input.into());
        self
    }
    /// <p>The name of the space.</p>
    pub fn set_space_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_space_name(input);
        self
    }
    /// <p>The name of the project in the space.</p>
    pub fn project_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.project_name(input.into());
        self
    }
    /// <p>The name of the project in the space.</p>
    pub fn set_project_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_project_name(input);
        self
    }
    /// <p>The system-generated unique ID of the Dev Environment you want to delete. To retrieve a list of Dev Environment IDs, use <code>ListDevEnvironments</code>.</p>
    pub fn id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.id(input.into());
        self
    }
    /// <p>The system-generated unique ID of the Dev Environment you want to delete. To retrieve a list of Dev Environment IDs, use <code>ListDevEnvironments</code>.</p>
    pub fn set_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_id(input);
        self
    }
}
