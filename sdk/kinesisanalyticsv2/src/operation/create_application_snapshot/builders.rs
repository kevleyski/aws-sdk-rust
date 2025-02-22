// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_application_snapshot::_create_application_snapshot_output::CreateApplicationSnapshotOutputBuilder;

pub use crate::operation::create_application_snapshot::_create_application_snapshot_input::CreateApplicationSnapshotInputBuilder;

/// Fluent builder constructing a request to `CreateApplicationSnapshot`.
///
/// <p>Creates a snapshot of the application's state data.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateApplicationSnapshotFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::create_application_snapshot::builders::CreateApplicationSnapshotInputBuilder
            }
impl CreateApplicationSnapshotFluentBuilder {
    /// Creates a new `CreateApplicationSnapshot`.
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
            crate::operation::create_application_snapshot::CreateApplicationSnapshot,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::create_application_snapshot::CreateApplicationSnapshotError,
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
        crate::operation::create_application_snapshot::CreateApplicationSnapshotOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::create_application_snapshot::CreateApplicationSnapshotError,
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
    /// <p>The name of an existing application</p>
    pub fn application_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.application_name(input.into());
        self
    }
    /// <p>The name of an existing application</p>
    pub fn set_application_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_application_name(input);
        self
    }
    /// <p>An identifier for the application snapshot.</p>
    pub fn snapshot_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.snapshot_name(input.into());
        self
    }
    /// <p>An identifier for the application snapshot.</p>
    pub fn set_snapshot_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_snapshot_name(input);
        self
    }
}
