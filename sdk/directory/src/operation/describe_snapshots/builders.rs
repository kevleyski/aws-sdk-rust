// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_snapshots::_describe_snapshots_output::DescribeSnapshotsOutputBuilder;

pub use crate::operation::describe_snapshots::_describe_snapshots_input::DescribeSnapshotsInputBuilder;

/// Fluent builder constructing a request to `DescribeSnapshots`.
///
/// <p>Obtains information about the directory snapshots that belong to this account.</p>
/// <p>This operation supports pagination with the use of the <i>NextToken</i> request and response parameters. If more results are available, the <i>DescribeSnapshots.NextToken</i> member contains a token that you pass in the next call to <code>DescribeSnapshots</code> to retrieve the next set of items.</p>
/// <p>You can also specify a maximum number of return results with the <i>Limit</i> parameter.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeSnapshotsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_snapshots::builders::DescribeSnapshotsInputBuilder,
}
impl DescribeSnapshotsFluentBuilder {
    /// Creates a new `DescribeSnapshots`.
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
            crate::operation::describe_snapshots::DescribeSnapshots,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_snapshots::DescribeSnapshotsError,
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
        crate::operation::describe_snapshots::DescribeSnapshotsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_snapshots::DescribeSnapshotsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::describe_snapshots::paginator::DescribeSnapshotsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::describe_snapshots::paginator::DescribeSnapshotsPaginator {
        crate::operation::describe_snapshots::paginator::DescribeSnapshotsPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>The identifier of the directory for which to retrieve snapshot information.</p>
    pub fn directory_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.directory_id(input.into());
        self
    }
    /// <p>The identifier of the directory for which to retrieve snapshot information.</p>
    pub fn set_directory_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_directory_id(input);
        self
    }
    /// Appends an item to `SnapshotIds`.
    ///
    /// To override the contents of this collection use [`set_snapshot_ids`](Self::set_snapshot_ids).
    ///
    /// <p>A list of identifiers of the snapshots to obtain the information for. If this member is null or empty, all snapshots are returned using the <i>Limit</i> and <i>NextToken</i> members.</p>
    pub fn snapshot_ids(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.snapshot_ids(input.into());
        self
    }
    /// <p>A list of identifiers of the snapshots to obtain the information for. If this member is null or empty, all snapshots are returned using the <i>Limit</i> and <i>NextToken</i> members.</p>
    pub fn set_snapshot_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_snapshot_ids(input);
        self
    }
    /// <p>The <i>DescribeSnapshotsResult.NextToken</i> value from a previous call to <code>DescribeSnapshots</code>. Pass null if this is the first call.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The <i>DescribeSnapshotsResult.NextToken</i> value from a previous call to <code>DescribeSnapshots</code>. Pass null if this is the first call.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The maximum number of objects to return.</p>
    pub fn limit(mut self, input: i32) -> Self {
        self.inner = self.inner.limit(input);
        self
    }
    /// <p>The maximum number of objects to return.</p>
    pub fn set_limit(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_limit(input);
        self
    }
}
