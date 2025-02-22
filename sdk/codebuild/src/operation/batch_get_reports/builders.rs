// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::batch_get_reports::_batch_get_reports_output::BatchGetReportsOutputBuilder;

pub use crate::operation::batch_get_reports::_batch_get_reports_input::BatchGetReportsInputBuilder;

/// Fluent builder constructing a request to `BatchGetReports`.
///
/// <p> Returns an array of reports. </p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct BatchGetReportsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::batch_get_reports::builders::BatchGetReportsInputBuilder,
}
impl BatchGetReportsFluentBuilder {
    /// Creates a new `BatchGetReports`.
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
            crate::operation::batch_get_reports::BatchGetReports,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::batch_get_reports::BatchGetReportsError,
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
        crate::operation::batch_get_reports::BatchGetReportsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::batch_get_reports::BatchGetReportsError,
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
    /// Appends an item to `reportArns`.
    ///
    /// To override the contents of this collection use [`set_report_arns`](Self::set_report_arns).
    ///
    /// <p> An array of ARNs that identify the <code>Report</code> objects to return. </p>
    pub fn report_arns(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.report_arns(input.into());
        self
    }
    /// <p> An array of ARNs that identify the <code>Report</code> objects to return. </p>
    pub fn set_report_arns(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_report_arns(input);
        self
    }
}
