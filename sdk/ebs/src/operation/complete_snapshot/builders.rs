// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::complete_snapshot::_complete_snapshot_output::CompleteSnapshotOutputBuilder;

pub use crate::operation::complete_snapshot::_complete_snapshot_input::CompleteSnapshotInputBuilder;

/// Fluent builder constructing a request to `CompleteSnapshot`.
///
/// <p>Seals and completes the snapshot after all of the required blocks of data have been written to it. Completing the snapshot changes the status to <code>completed</code>. You cannot write new blocks to a snapshot after it has been completed.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CompleteSnapshotFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::complete_snapshot::builders::CompleteSnapshotInputBuilder,
}
impl CompleteSnapshotFluentBuilder {
    /// Creates a new `CompleteSnapshot`.
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
            crate::operation::complete_snapshot::CompleteSnapshot,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::complete_snapshot::CompleteSnapshotError,
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
        crate::operation::complete_snapshot::CompleteSnapshotOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::complete_snapshot::CompleteSnapshotError,
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
    /// <p>The ID of the snapshot.</p>
    pub fn snapshot_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.snapshot_id(input.into());
        self
    }
    /// <p>The ID of the snapshot.</p>
    pub fn set_snapshot_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_snapshot_id(input);
        self
    }
    /// <p>The number of blocks that were written to the snapshot.</p>
    pub fn changed_blocks_count(mut self, input: i32) -> Self {
        self.inner = self.inner.changed_blocks_count(input);
        self
    }
    /// <p>The number of blocks that were written to the snapshot.</p>
    pub fn set_changed_blocks_count(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_changed_blocks_count(input);
        self
    }
    /// <p>An aggregated Base-64 SHA256 checksum based on the checksums of each written block.</p>
    /// <p>To generate the aggregated checksum using the linear aggregation method, arrange the checksums for each written block in ascending order of their block index, concatenate them to form a single string, and then generate the checksum on the entire string using the SHA256 algorithm.</p>
    pub fn checksum(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.checksum(input.into());
        self
    }
    /// <p>An aggregated Base-64 SHA256 checksum based on the checksums of each written block.</p>
    /// <p>To generate the aggregated checksum using the linear aggregation method, arrange the checksums for each written block in ascending order of their block index, concatenate them to form a single string, and then generate the checksum on the entire string using the SHA256 algorithm.</p>
    pub fn set_checksum(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_checksum(input);
        self
    }
    /// <p>The algorithm used to generate the checksum. Currently, the only supported algorithm is <code>SHA256</code>.</p>
    pub fn checksum_algorithm(mut self, input: crate::types::ChecksumAlgorithm) -> Self {
        self.inner = self.inner.checksum_algorithm(input);
        self
    }
    /// <p>The algorithm used to generate the checksum. Currently, the only supported algorithm is <code>SHA256</code>.</p>
    pub fn set_checksum_algorithm(
        mut self,
        input: std::option::Option<crate::types::ChecksumAlgorithm>,
    ) -> Self {
        self.inner = self.inner.set_checksum_algorithm(input);
        self
    }
    /// <p>The aggregation method used to generate the checksum. Currently, the only supported aggregation method is <code>LINEAR</code>.</p>
    pub fn checksum_aggregation_method(
        mut self,
        input: crate::types::ChecksumAggregationMethod,
    ) -> Self {
        self.inner = self.inner.checksum_aggregation_method(input);
        self
    }
    /// <p>The aggregation method used to generate the checksum. Currently, the only supported aggregation method is <code>LINEAR</code>.</p>
    pub fn set_checksum_aggregation_method(
        mut self,
        input: std::option::Option<crate::types::ChecksumAggregationMethod>,
    ) -> Self {
        self.inner = self.inner.set_checksum_aggregation_method(input);
        self
    }
}
