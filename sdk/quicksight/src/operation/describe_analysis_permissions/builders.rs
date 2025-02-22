// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_analysis_permissions::_describe_analysis_permissions_output::DescribeAnalysisPermissionsOutputBuilder;

pub use crate::operation::describe_analysis_permissions::_describe_analysis_permissions_input::DescribeAnalysisPermissionsInputBuilder;

/// Fluent builder constructing a request to `DescribeAnalysisPermissions`.
///
/// <p>Provides the read and write permissions for an analysis.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeAnalysisPermissionsFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::describe_analysis_permissions::builders::DescribeAnalysisPermissionsInputBuilder
            }
impl DescribeAnalysisPermissionsFluentBuilder {
    /// Creates a new `DescribeAnalysisPermissions`.
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
            crate::operation::describe_analysis_permissions::DescribeAnalysisPermissions,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_analysis_permissions::DescribeAnalysisPermissionsError,
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
        crate::operation::describe_analysis_permissions::DescribeAnalysisPermissionsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_analysis_permissions::DescribeAnalysisPermissionsError,
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
    /// <p>The ID of the Amazon Web Services account that contains the analysis whose permissions you're describing. You must be using the Amazon Web Services account that the analysis is in.</p>
    pub fn aws_account_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.aws_account_id(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services account that contains the analysis whose permissions you're describing. You must be using the Amazon Web Services account that the analysis is in.</p>
    pub fn set_aws_account_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_aws_account_id(input);
        self
    }
    /// <p>The ID of the analysis whose permissions you're describing. The ID is part of the analysis URL.</p>
    pub fn analysis_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.analysis_id(input.into());
        self
    }
    /// <p>The ID of the analysis whose permissions you're describing. The ID is part of the analysis URL.</p>
    pub fn set_analysis_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_analysis_id(input);
        self
    }
}
