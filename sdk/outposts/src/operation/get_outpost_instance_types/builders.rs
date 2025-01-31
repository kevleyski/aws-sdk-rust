// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_outpost_instance_types::_get_outpost_instance_types_output::GetOutpostInstanceTypesOutputBuilder;

pub use crate::operation::get_outpost_instance_types::_get_outpost_instance_types_input::GetOutpostInstanceTypesInputBuilder;

/// Fluent builder constructing a request to `GetOutpostInstanceTypes`.
///
/// <p>Gets the instance types for the specified Outpost.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetOutpostInstanceTypesFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::get_outpost_instance_types::builders::GetOutpostInstanceTypesInputBuilder,
}
impl GetOutpostInstanceTypesFluentBuilder {
    /// Creates a new `GetOutpostInstanceTypes`.
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
            crate::operation::get_outpost_instance_types::GetOutpostInstanceTypes,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::get_outpost_instance_types::GetOutpostInstanceTypesError,
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
        crate::operation::get_outpost_instance_types::GetOutpostInstanceTypesOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::get_outpost_instance_types::GetOutpostInstanceTypesError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::get_outpost_instance_types::paginator::GetOutpostInstanceTypesPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::get_outpost_instance_types::paginator::GetOutpostInstanceTypesPaginator
    {
        crate::operation::get_outpost_instance_types::paginator::GetOutpostInstanceTypesPaginator::new(self.handle, self.inner)
    }
    /// <p> The ID or the Amazon Resource Name (ARN) of the Outpost. </p>
    pub fn outpost_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.outpost_id(input.into());
        self
    }
    /// <p> The ID or the Amazon Resource Name (ARN) of the Outpost. </p>
    pub fn set_outpost_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_outpost_id(input);
        self
    }
    /// <p>The pagination token.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The pagination token.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The maximum page size.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum page size.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
}
