// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_data_cells_filter::_create_data_cells_filter_output::CreateDataCellsFilterOutputBuilder;

pub use crate::operation::create_data_cells_filter::_create_data_cells_filter_input::CreateDataCellsFilterInputBuilder;

/// Fluent builder constructing a request to `CreateDataCellsFilter`.
///
/// <p>Creates a data cell filter to allow one to grant access to certain columns on certain rows.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateDataCellsFilterFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_data_cells_filter::builders::CreateDataCellsFilterInputBuilder,
}
impl CreateDataCellsFilterFluentBuilder {
    /// Creates a new `CreateDataCellsFilter`.
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
            crate::operation::create_data_cells_filter::CreateDataCellsFilter,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::create_data_cells_filter::CreateDataCellsFilterError,
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
        crate::operation::create_data_cells_filter::CreateDataCellsFilterOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::create_data_cells_filter::CreateDataCellsFilterError,
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
    /// <p>A <code>DataCellsFilter</code> structure containing information about the data cells filter.</p>
    pub fn table_data(mut self, input: crate::types::DataCellsFilter) -> Self {
        self.inner = self.inner.table_data(input);
        self
    }
    /// <p>A <code>DataCellsFilter</code> structure containing information about the data cells filter.</p>
    pub fn set_table_data(
        mut self,
        input: std::option::Option<crate::types::DataCellsFilter>,
    ) -> Self {
        self.inner = self.inner.set_table_data(input);
        self
    }
}
