// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_schemas::_describe_schemas_output::DescribeSchemasOutputBuilder;

pub use crate::operation::describe_schemas::_describe_schemas_input::DescribeSchemasInputBuilder;

/// Fluent builder constructing a request to `DescribeSchemas`.
///
/// <p>Returns information about the schema for the specified endpoint.</p>
/// <p></p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeSchemasFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_schemas::builders::DescribeSchemasInputBuilder,
}
impl DescribeSchemasFluentBuilder {
    /// Creates a new `DescribeSchemas`.
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
            crate::operation::describe_schemas::DescribeSchemas,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::describe_schemas::DescribeSchemasError>,
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
        crate::operation::describe_schemas::DescribeSchemasOutput,
        aws_smithy_http::result::SdkError<crate::operation::describe_schemas::DescribeSchemasError>,
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
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::describe_schemas::paginator::DescribeSchemasPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::describe_schemas::paginator::DescribeSchemasPaginator {
        crate::operation::describe_schemas::paginator::DescribeSchemasPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>The Amazon Resource Name (ARN) string that uniquely identifies the endpoint.</p>
    pub fn endpoint_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.endpoint_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) string that uniquely identifies the endpoint.</p>
    pub fn set_endpoint_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_endpoint_arn(input);
        self
    }
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p>
    /// <p>Default: 100</p>
    /// <p>Constraints: Minimum 20, maximum 100.</p>
    pub fn max_records(mut self, input: i32) -> Self {
        self.inner = self.inner.max_records(input);
        self
    }
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p>
    /// <p>Default: 100</p>
    /// <p>Constraints: Minimum 20, maximum 100.</p>
    pub fn set_max_records(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_records(input);
        self
    }
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    pub fn marker(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.marker(input.into());
        self
    }
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    pub fn set_marker(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_marker(input);
        self
    }
}
