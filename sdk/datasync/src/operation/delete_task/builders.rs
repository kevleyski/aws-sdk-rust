// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_task::_delete_task_output::DeleteTaskOutputBuilder;

pub use crate::operation::delete_task::_delete_task_input::DeleteTaskInputBuilder;

/// Fluent builder constructing a request to `DeleteTask`.
///
/// <p>Deletes an DataSync task.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteTaskFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_task::builders::DeleteTaskInputBuilder,
}
impl DeleteTaskFluentBuilder {
    /// Creates a new `DeleteTask`.
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
            crate::operation::delete_task::DeleteTask,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::delete_task::DeleteTaskError>,
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
        crate::operation::delete_task::DeleteTaskOutput,
        aws_smithy_http::result::SdkError<crate::operation::delete_task::DeleteTaskError>,
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
    /// <p>Specifies the Amazon Resource Name (ARN) of the task that you want to delete.</p>
    pub fn task_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.task_arn(input.into());
        self
    }
    /// <p>Specifies the Amazon Resource Name (ARN) of the task that you want to delete.</p>
    pub fn set_task_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_task_arn(input);
        self
    }
}
