// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_query_definition::_delete_query_definition_output::DeleteQueryDefinitionOutputBuilder;

pub use crate::operation::delete_query_definition::_delete_query_definition_input::DeleteQueryDefinitionInputBuilder;

/// Fluent builder constructing a request to `DeleteQueryDefinition`.
///
/// <p>Deletes a saved CloudWatch Logs Insights query definition. A query definition contains details about a saved CloudWatch Logs Insights query.</p>
/// <p>Each <code>DeleteQueryDefinition</code> operation can delete one query definition.</p>
/// <p>You must have the <code>logs:DeleteQueryDefinition</code> permission to be able to perform this operation.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteQueryDefinitionFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_query_definition::builders::DeleteQueryDefinitionInputBuilder,
}
impl DeleteQueryDefinitionFluentBuilder {
    /// Creates a new `DeleteQueryDefinition`.
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
            crate::operation::delete_query_definition::DeleteQueryDefinition,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_query_definition::DeleteQueryDefinitionError,
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
        crate::operation::delete_query_definition::DeleteQueryDefinitionOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_query_definition::DeleteQueryDefinitionError,
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
    /// <p>The ID of the query definition that you want to delete. You can use <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_DescribeQueryDefinitions.html">DescribeQueryDefinitions</a> to retrieve the IDs of your saved query definitions.</p>
    pub fn query_definition_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.query_definition_id(input.into());
        self
    }
    /// <p>The ID of the query definition that you want to delete. You can use <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_DescribeQueryDefinitions.html">DescribeQueryDefinitions</a> to retrieve the IDs of your saved query definitions.</p>
    pub fn set_query_definition_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_query_definition_id(input);
        self
    }
}
