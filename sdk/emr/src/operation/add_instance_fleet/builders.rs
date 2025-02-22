// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::add_instance_fleet::_add_instance_fleet_output::AddInstanceFleetOutputBuilder;

pub use crate::operation::add_instance_fleet::_add_instance_fleet_input::AddInstanceFleetInputBuilder;

/// Fluent builder constructing a request to `AddInstanceFleet`.
///
/// <p>Adds an instance fleet to a running cluster.</p> <note>
/// <p>The instance fleet configuration is available only in Amazon EMR versions 4.8.0 and later, excluding 5.0.x.</p>
/// </note>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct AddInstanceFleetFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::add_instance_fleet::builders::AddInstanceFleetInputBuilder,
}
impl AddInstanceFleetFluentBuilder {
    /// Creates a new `AddInstanceFleet`.
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
            crate::operation::add_instance_fleet::AddInstanceFleet,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::add_instance_fleet::AddInstanceFleetError,
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
        crate::operation::add_instance_fleet::AddInstanceFleetOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::add_instance_fleet::AddInstanceFleetError,
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
    /// <p>The unique identifier of the cluster.</p>
    pub fn cluster_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.cluster_id(input.into());
        self
    }
    /// <p>The unique identifier of the cluster.</p>
    pub fn set_cluster_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_cluster_id(input);
        self
    }
    /// <p>Specifies the configuration of the instance fleet.</p>
    pub fn instance_fleet(mut self, input: crate::types::InstanceFleetConfig) -> Self {
        self.inner = self.inner.instance_fleet(input);
        self
    }
    /// <p>Specifies the configuration of the instance fleet.</p>
    pub fn set_instance_fleet(
        mut self,
        input: std::option::Option<crate::types::InstanceFleetConfig>,
    ) -> Self {
        self.inner = self.inner.set_instance_fleet(input);
        self
    }
}
