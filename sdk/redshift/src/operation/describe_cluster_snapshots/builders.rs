// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_cluster_snapshots::_describe_cluster_snapshots_output::DescribeClusterSnapshotsOutputBuilder;

pub use crate::operation::describe_cluster_snapshots::_describe_cluster_snapshots_input::DescribeClusterSnapshotsInputBuilder;

/// Fluent builder constructing a request to `DescribeClusterSnapshots`.
///
/// <p>Returns one or more snapshot objects, which contain metadata about your cluster snapshots. By default, this operation returns information about all snapshots of all clusters that are owned by your Amazon Web Services account. No information is returned for snapshots owned by inactive Amazon Web Services accounts.</p>
/// <p>If you specify both tag keys and tag values in the same request, Amazon Redshift returns all snapshots that match any combination of the specified keys and values. For example, if you have <code>owner</code> and <code>environment</code> for tag keys, and <code>admin</code> and <code>test</code> for tag values, all snapshots that have any combination of those values are returned. Only snapshots that you own are returned in the response; shared snapshots are not returned with the tag key and tag value request parameters.</p>
/// <p>If both tag keys and values are omitted from the request, snapshots are returned regardless of whether they have tag keys or values associated with them.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeClusterSnapshotsFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::describe_cluster_snapshots::builders::DescribeClusterSnapshotsInputBuilder
            }
impl DescribeClusterSnapshotsFluentBuilder {
    /// Creates a new `DescribeClusterSnapshots`.
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
            crate::operation::describe_cluster_snapshots::DescribeClusterSnapshots,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_cluster_snapshots::DescribeClusterSnapshotsError,
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
        crate::operation::describe_cluster_snapshots::DescribeClusterSnapshotsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_cluster_snapshots::DescribeClusterSnapshotsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::describe_cluster_snapshots::paginator::DescribeClusterSnapshotsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::describe_cluster_snapshots::paginator::DescribeClusterSnapshotsPaginator
    {
        crate::operation::describe_cluster_snapshots::paginator::DescribeClusterSnapshotsPaginator::new(self.handle, self.inner)
    }
    /// <p>The identifier of the cluster which generated the requested snapshots.</p>
    pub fn cluster_identifier(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.cluster_identifier(input.into());
        self
    }
    /// <p>The identifier of the cluster which generated the requested snapshots.</p>
    pub fn set_cluster_identifier(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_cluster_identifier(input);
        self
    }
    /// <p>The snapshot identifier of the snapshot about which to return information.</p>
    pub fn snapshot_identifier(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.snapshot_identifier(input.into());
        self
    }
    /// <p>The snapshot identifier of the snapshot about which to return information.</p>
    pub fn set_snapshot_identifier(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_snapshot_identifier(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the snapshot associated with the message to describe cluster snapshots.</p>
    pub fn snapshot_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.snapshot_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the snapshot associated with the message to describe cluster snapshots.</p>
    pub fn set_snapshot_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_snapshot_arn(input);
        self
    }
    /// <p>The type of snapshots for which you are requesting information. By default, snapshots of all types are returned.</p>
    /// <p>Valid Values: <code>automated</code> | <code>manual</code> </p>
    pub fn snapshot_type(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.snapshot_type(input.into());
        self
    }
    /// <p>The type of snapshots for which you are requesting information. By default, snapshots of all types are returned.</p>
    /// <p>Valid Values: <code>automated</code> | <code>manual</code> </p>
    pub fn set_snapshot_type(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_snapshot_type(input);
        self
    }
    /// <p>A value that requests only snapshots created at or after the specified time. The time value is specified in ISO 8601 format. For more information about ISO 8601, go to the <a href="http://en.wikipedia.org/wiki/ISO_8601">ISO8601 Wikipedia page.</a> </p>
    /// <p>Example: <code>2012-07-16T18:00:00Z</code> </p>
    pub fn start_time(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.start_time(input);
        self
    }
    /// <p>A value that requests only snapshots created at or after the specified time. The time value is specified in ISO 8601 format. For more information about ISO 8601, go to the <a href="http://en.wikipedia.org/wiki/ISO_8601">ISO8601 Wikipedia page.</a> </p>
    /// <p>Example: <code>2012-07-16T18:00:00Z</code> </p>
    pub fn set_start_time(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.inner = self.inner.set_start_time(input);
        self
    }
    /// <p>A time value that requests only snapshots created at or before the specified time. The time value is specified in ISO 8601 format. For more information about ISO 8601, go to the <a href="http://en.wikipedia.org/wiki/ISO_8601">ISO8601 Wikipedia page.</a> </p>
    /// <p>Example: <code>2012-07-16T18:00:00Z</code> </p>
    pub fn end_time(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.end_time(input);
        self
    }
    /// <p>A time value that requests only snapshots created at or before the specified time. The time value is specified in ISO 8601 format. For more information about ISO 8601, go to the <a href="http://en.wikipedia.org/wiki/ISO_8601">ISO8601 Wikipedia page.</a> </p>
    /// <p>Example: <code>2012-07-16T18:00:00Z</code> </p>
    pub fn set_end_time(mut self, input: std::option::Option<aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_end_time(input);
        self
    }
    /// <p>The maximum number of response records to return in each call. If the number of remaining response records exceeds the specified <code>MaxRecords</code> value, a value is returned in a <code>marker</code> field of the response. You can retrieve the next set of records by retrying the command with the returned marker value. </p>
    /// <p>Default: <code>100</code> </p>
    /// <p>Constraints: minimum 20, maximum 500.</p>
    pub fn max_records(mut self, input: i32) -> Self {
        self.inner = self.inner.max_records(input);
        self
    }
    /// <p>The maximum number of response records to return in each call. If the number of remaining response records exceeds the specified <code>MaxRecords</code> value, a value is returned in a <code>marker</code> field of the response. You can retrieve the next set of records by retrying the command with the returned marker value. </p>
    /// <p>Default: <code>100</code> </p>
    /// <p>Constraints: minimum 20, maximum 500.</p>
    pub fn set_max_records(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_records(input);
        self
    }
    /// <p>An optional parameter that specifies the starting point to return a set of response records. When the results of a <code>DescribeClusterSnapshots</code> request exceed the value specified in <code>MaxRecords</code>, Amazon Web Services returns a value in the <code>Marker</code> field of the response. You can retrieve the next set of response records by providing the returned marker value in the <code>Marker</code> parameter and retrying the request. </p>
    pub fn marker(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.marker(input.into());
        self
    }
    /// <p>An optional parameter that specifies the starting point to return a set of response records. When the results of a <code>DescribeClusterSnapshots</code> request exceed the value specified in <code>MaxRecords</code>, Amazon Web Services returns a value in the <code>Marker</code> field of the response. You can retrieve the next set of response records by providing the returned marker value in the <code>Marker</code> parameter and retrying the request. </p>
    pub fn set_marker(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_marker(input);
        self
    }
    /// <p>The Amazon Web Services account used to create or copy the snapshot. Use this field to filter the results to snapshots owned by a particular account. To describe snapshots you own, either specify your Amazon Web Services account, or do not specify the parameter.</p>
    pub fn owner_account(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.owner_account(input.into());
        self
    }
    /// <p>The Amazon Web Services account used to create or copy the snapshot. Use this field to filter the results to snapshots owned by a particular account. To describe snapshots you own, either specify your Amazon Web Services account, or do not specify the parameter.</p>
    pub fn set_owner_account(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_owner_account(input);
        self
    }
    /// Appends an item to `TagKeys`.
    ///
    /// To override the contents of this collection use [`set_tag_keys`](Self::set_tag_keys).
    ///
    /// <p>A tag key or keys for which you want to return all matching cluster snapshots that are associated with the specified key or keys. For example, suppose that you have snapshots that are tagged with keys called <code>owner</code> and <code>environment</code>. If you specify both of these tag keys in the request, Amazon Redshift returns a response with the snapshots that have either or both of these tag keys associated with them.</p>
    pub fn tag_keys(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.tag_keys(input.into());
        self
    }
    /// <p>A tag key or keys for which you want to return all matching cluster snapshots that are associated with the specified key or keys. For example, suppose that you have snapshots that are tagged with keys called <code>owner</code> and <code>environment</code>. If you specify both of these tag keys in the request, Amazon Redshift returns a response with the snapshots that have either or both of these tag keys associated with them.</p>
    pub fn set_tag_keys(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_tag_keys(input);
        self
    }
    /// Appends an item to `TagValues`.
    ///
    /// To override the contents of this collection use [`set_tag_values`](Self::set_tag_values).
    ///
    /// <p>A tag value or values for which you want to return all matching cluster snapshots that are associated with the specified tag value or values. For example, suppose that you have snapshots that are tagged with values called <code>admin</code> and <code>test</code>. If you specify both of these tag values in the request, Amazon Redshift returns a response with the snapshots that have either or both of these tag values associated with them.</p>
    pub fn tag_values(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.tag_values(input.into());
        self
    }
    /// <p>A tag value or values for which you want to return all matching cluster snapshots that are associated with the specified tag value or values. For example, suppose that you have snapshots that are tagged with values called <code>admin</code> and <code>test</code>. If you specify both of these tag values in the request, Amazon Redshift returns a response with the snapshots that have either or both of these tag values associated with them.</p>
    pub fn set_tag_values(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_tag_values(input);
        self
    }
    /// <p>A value that indicates whether to return snapshots only for an existing cluster. You can perform table-level restore only by using a snapshot of an existing cluster, that is, a cluster that has not been deleted. Values for this parameter work as follows: </p>
    /// <ul>
    /// <li> <p>If <code>ClusterExists</code> is set to <code>true</code>, <code>ClusterIdentifier</code> is required.</p> </li>
    /// <li> <p>If <code>ClusterExists</code> is set to <code>false</code> and <code>ClusterIdentifier</code> isn't specified, all snapshots associated with deleted clusters (orphaned snapshots) are returned. </p> </li>
    /// <li> <p>If <code>ClusterExists</code> is set to <code>false</code> and <code>ClusterIdentifier</code> is specified for a deleted cluster, snapshots associated with that cluster are returned.</p> </li>
    /// <li> <p>If <code>ClusterExists</code> is set to <code>false</code> and <code>ClusterIdentifier</code> is specified for an existing cluster, no snapshots are returned. </p> </li>
    /// </ul>
    pub fn cluster_exists(mut self, input: bool) -> Self {
        self.inner = self.inner.cluster_exists(input);
        self
    }
    /// <p>A value that indicates whether to return snapshots only for an existing cluster. You can perform table-level restore only by using a snapshot of an existing cluster, that is, a cluster that has not been deleted. Values for this parameter work as follows: </p>
    /// <ul>
    /// <li> <p>If <code>ClusterExists</code> is set to <code>true</code>, <code>ClusterIdentifier</code> is required.</p> </li>
    /// <li> <p>If <code>ClusterExists</code> is set to <code>false</code> and <code>ClusterIdentifier</code> isn't specified, all snapshots associated with deleted clusters (orphaned snapshots) are returned. </p> </li>
    /// <li> <p>If <code>ClusterExists</code> is set to <code>false</code> and <code>ClusterIdentifier</code> is specified for a deleted cluster, snapshots associated with that cluster are returned.</p> </li>
    /// <li> <p>If <code>ClusterExists</code> is set to <code>false</code> and <code>ClusterIdentifier</code> is specified for an existing cluster, no snapshots are returned. </p> </li>
    /// </ul>
    pub fn set_cluster_exists(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_cluster_exists(input);
        self
    }
    /// Appends an item to `SortingEntities`.
    ///
    /// To override the contents of this collection use [`set_sorting_entities`](Self::set_sorting_entities).
    ///
    /// <p></p>
    pub fn sorting_entities(mut self, input: crate::types::SnapshotSortingEntity) -> Self {
        self.inner = self.inner.sorting_entities(input);
        self
    }
    /// <p></p>
    pub fn set_sorting_entities(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::SnapshotSortingEntity>>,
    ) -> Self {
        self.inner = self.inner.set_sorting_entities(input);
        self
    }
}
