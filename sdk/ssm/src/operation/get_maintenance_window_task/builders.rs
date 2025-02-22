// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_maintenance_window_task::_get_maintenance_window_task_output::GetMaintenanceWindowTaskOutputBuilder;

pub use crate::operation::get_maintenance_window_task::_get_maintenance_window_task_input::GetMaintenanceWindowTaskInputBuilder;

/// Fluent builder constructing a request to `GetMaintenanceWindowTask`.
///
/// <p>Retrieves the details of a maintenance window task.</p> <note>
/// <p>For maintenance window tasks without a specified target, you can't supply values for <code>--max-errors</code> and <code>--max-concurrency</code>. Instead, the system inserts a placeholder value of <code>1</code>, which may be reported in the response to this command. These values don't affect the running of your task and can be ignored.</p>
/// </note>
/// <p>To retrieve a list of tasks in a maintenance window, instead use the <code>DescribeMaintenanceWindowTasks</code> command.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetMaintenanceWindowTaskFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::get_maintenance_window_task::builders::GetMaintenanceWindowTaskInputBuilder
            }
impl GetMaintenanceWindowTaskFluentBuilder {
    /// Creates a new `GetMaintenanceWindowTask`.
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
            crate::operation::get_maintenance_window_task::GetMaintenanceWindowTask,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::get_maintenance_window_task::GetMaintenanceWindowTaskError,
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
        crate::operation::get_maintenance_window_task::GetMaintenanceWindowTaskOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::get_maintenance_window_task::GetMaintenanceWindowTaskError,
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
    /// <p>The maintenance window ID that includes the task to retrieve.</p>
    pub fn window_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.window_id(input.into());
        self
    }
    /// <p>The maintenance window ID that includes the task to retrieve.</p>
    pub fn set_window_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_window_id(input);
        self
    }
    /// <p>The maintenance window task ID to retrieve.</p>
    pub fn window_task_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.window_task_id(input.into());
        self
    }
    /// <p>The maintenance window task ID to retrieve.</p>
    pub fn set_window_task_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_window_task_id(input);
        self
    }
}
