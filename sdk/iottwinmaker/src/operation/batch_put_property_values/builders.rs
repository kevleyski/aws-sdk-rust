// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::batch_put_property_values::_batch_put_property_values_output::BatchPutPropertyValuesOutputBuilder;

pub use crate::operation::batch_put_property_values::_batch_put_property_values_input::BatchPutPropertyValuesInputBuilder;

/// Fluent builder constructing a request to `BatchPutPropertyValues`.
///
/// <p>Sets values for multiple time series properties.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct BatchPutPropertyValuesFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::batch_put_property_values::builders::BatchPutPropertyValuesInputBuilder,
}
impl BatchPutPropertyValuesFluentBuilder {
    /// Creates a new `BatchPutPropertyValues`.
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
            crate::operation::batch_put_property_values::BatchPutPropertyValues,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::batch_put_property_values::BatchPutPropertyValuesError,
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
        crate::operation::batch_put_property_values::BatchPutPropertyValuesOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::batch_put_property_values::BatchPutPropertyValuesError,
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
    /// <p>The ID of the workspace that contains the properties to set.</p>
    pub fn workspace_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.workspace_id(input.into());
        self
    }
    /// <p>The ID of the workspace that contains the properties to set.</p>
    pub fn set_workspace_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_workspace_id(input);
        self
    }
    /// Appends an item to `entries`.
    ///
    /// To override the contents of this collection use [`set_entries`](Self::set_entries).
    ///
    /// <p>An object that maps strings to the property value entries to set. Each string in the mapping must be unique to this object.</p>
    pub fn entries(mut self, input: crate::types::PropertyValueEntry) -> Self {
        self.inner = self.inner.entries(input);
        self
    }
    /// <p>An object that maps strings to the property value entries to set. Each string in the mapping must be unique to this object.</p>
    pub fn set_entries(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::PropertyValueEntry>>,
    ) -> Self {
        self.inner = self.inner.set_entries(input);
        self
    }
}
