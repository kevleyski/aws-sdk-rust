// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_service_updates::_describe_service_updates_output::DescribeServiceUpdatesOutputBuilder;

pub use crate::operation::describe_service_updates::_describe_service_updates_input::DescribeServiceUpdatesInputBuilder;

/// Fluent builder constructing a request to `DescribeServiceUpdates`.
///
/// <p>Returns details of the service updates</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeServiceUpdatesFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_service_updates::builders::DescribeServiceUpdatesInputBuilder,
}
impl DescribeServiceUpdatesFluentBuilder {
    /// Creates a new `DescribeServiceUpdates`.
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
            crate::operation::describe_service_updates::DescribeServiceUpdates,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_service_updates::DescribeServiceUpdatesError,
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
        crate::operation::describe_service_updates::DescribeServiceUpdatesOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_service_updates::DescribeServiceUpdatesError,
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
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::describe_service_updates::paginator::DescribeServiceUpdatesPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::describe_service_updates::paginator::DescribeServiceUpdatesPaginator
    {
        crate::operation::describe_service_updates::paginator::DescribeServiceUpdatesPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>The unique ID of the service update to describe.</p>
    pub fn service_update_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.service_update_name(input.into());
        self
    }
    /// <p>The unique ID of the service update to describe.</p>
    pub fn set_service_update_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_service_update_name(input);
        self
    }
    /// Appends an item to `ClusterNames`.
    ///
    /// To override the contents of this collection use [`set_cluster_names`](Self::set_cluster_names).
    ///
    /// <p>The list of cluster names to identify service updates to apply</p>
    pub fn cluster_names(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.cluster_names(input.into());
        self
    }
    /// <p>The list of cluster names to identify service updates to apply</p>
    pub fn set_cluster_names(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_cluster_names(input);
        self
    }
    /// Appends an item to `Status`.
    ///
    /// To override the contents of this collection use [`set_status`](Self::set_status).
    ///
    /// <p>The status(es) of the service updates to filter on</p>
    pub fn status(mut self, input: crate::types::ServiceUpdateStatus) -> Self {
        self.inner = self.inner.status(input);
        self
    }
    /// <p>The status(es) of the service updates to filter on</p>
    pub fn set_status(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::ServiceUpdateStatus>>,
    ) -> Self {
        self.inner = self.inner.set_status(input);
        self
    }
    /// <p>The maximum number of records to include in the response. If more records exist than the specified MaxResults value, a token is included in the response so that the remaining results can be retrieved.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of records to include in the response. If more records exist than the specified MaxResults value, a token is included in the response so that the remaining results can be retrieved.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>An optional argument to pass in case the total number of records exceeds the value of MaxResults. If nextToken is returned, there are more results available. The value of nextToken is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. </p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>An optional argument to pass in case the total number of records exceeds the value of MaxResults. If nextToken is returned, there are more results available. The value of nextToken is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. </p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
}
