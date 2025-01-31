// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::revoke_snapshot_access::_revoke_snapshot_access_output::RevokeSnapshotAccessOutputBuilder;

pub use crate::operation::revoke_snapshot_access::_revoke_snapshot_access_input::RevokeSnapshotAccessInputBuilder;

/// Fluent builder constructing a request to `RevokeSnapshotAccess`.
///
/// <p>Removes the ability of the specified Amazon Web Services account to restore the specified snapshot. If the account is currently restoring the snapshot, the restore will run to completion.</p>
/// <p> For more information about working with snapshots, go to <a href="https://docs.aws.amazon.com/redshift/latest/mgmt/working-with-snapshots.html">Amazon Redshift Snapshots</a> in the <i>Amazon Redshift Cluster Management Guide</i>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct RevokeSnapshotAccessFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::revoke_snapshot_access::builders::RevokeSnapshotAccessInputBuilder,
}
impl RevokeSnapshotAccessFluentBuilder {
    /// Creates a new `RevokeSnapshotAccess`.
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
            crate::operation::revoke_snapshot_access::RevokeSnapshotAccess,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::revoke_snapshot_access::RevokeSnapshotAccessError,
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
        crate::operation::revoke_snapshot_access::RevokeSnapshotAccessOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::revoke_snapshot_access::RevokeSnapshotAccessError,
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
    /// <p>The identifier of the snapshot that the account can no longer access.</p>
    pub fn snapshot_identifier(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.snapshot_identifier(input.into());
        self
    }
    /// <p>The identifier of the snapshot that the account can no longer access.</p>
    pub fn set_snapshot_identifier(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_snapshot_identifier(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the snapshot associated with the message to revoke access.</p>
    pub fn snapshot_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.snapshot_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the snapshot associated with the message to revoke access.</p>
    pub fn set_snapshot_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_snapshot_arn(input);
        self
    }
    /// <p>The identifier of the cluster the snapshot was created from. This parameter is required if your IAM user or role has a policy containing a snapshot resource element that specifies anything other than * for the cluster name.</p>
    pub fn snapshot_cluster_identifier(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.snapshot_cluster_identifier(input.into());
        self
    }
    /// <p>The identifier of the cluster the snapshot was created from. This parameter is required if your IAM user or role has a policy containing a snapshot resource element that specifies anything other than * for the cluster name.</p>
    pub fn set_snapshot_cluster_identifier(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_snapshot_cluster_identifier(input);
        self
    }
    /// <p>The identifier of the Amazon Web Services account that can no longer restore the specified snapshot.</p>
    pub fn account_with_restore_access(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.account_with_restore_access(input.into());
        self
    }
    /// <p>The identifier of the Amazon Web Services account that can no longer restore the specified snapshot.</p>
    pub fn set_account_with_restore_access(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_account_with_restore_access(input);
        self
    }
}
