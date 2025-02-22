// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_volume::_delete_volume_output::DeleteVolumeOutputBuilder;

pub use crate::operation::delete_volume::_delete_volume_input::DeleteVolumeInputBuilder;

/// Fluent builder constructing a request to `DeleteVolume`.
///
/// <p>Deletes the specified storage volume that you previously created using the <code>CreateCachediSCSIVolume</code> or <code>CreateStorediSCSIVolume</code> API. This operation is only supported in the cached volume and stored volume types. For stored volume gateways, the local disk that was configured as the storage volume is not deleted. You can reuse the local disk to create another storage volume.</p>
/// <p>Before you delete a volume, make sure there are no iSCSI connections to the volume you are deleting. You should also make sure there is no snapshot in progress. You can use the Amazon Elastic Compute Cloud (Amazon EC2) API to query snapshots on the volume you are deleting and check the snapshot status. For more information, go to <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/ApiReference-query-DescribeSnapshots.html">DescribeSnapshots</a> in the <i>Amazon Elastic Compute Cloud API Reference</i>.</p>
/// <p>In the request, you must provide the Amazon Resource Name (ARN) of the storage volume you want to delete.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteVolumeFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_volume::builders::DeleteVolumeInputBuilder,
}
impl DeleteVolumeFluentBuilder {
    /// Creates a new `DeleteVolume`.
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
            crate::operation::delete_volume::DeleteVolume,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::delete_volume::DeleteVolumeError>,
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
        crate::operation::delete_volume::DeleteVolumeOutput,
        aws_smithy_http::result::SdkError<crate::operation::delete_volume::DeleteVolumeError>,
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
    /// <p>The Amazon Resource Name (ARN) of the volume. Use the <code>ListVolumes</code> operation to return a list of gateway volumes.</p>
    pub fn volume_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.volume_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the volume. Use the <code>ListVolumes</code> operation to return a list of gateway volumes.</p>
    pub fn set_volume_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_volume_arn(input);
        self
    }
}
