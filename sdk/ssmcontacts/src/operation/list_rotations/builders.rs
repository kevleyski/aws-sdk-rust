// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_rotations::_list_rotations_output::ListRotationsOutputBuilder;

pub use crate::operation::list_rotations::_list_rotations_input::ListRotationsInputBuilder;

/// Fluent builder constructing a request to `ListRotations`.
///
/// <p>Retrieves a list of on-call rotations.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ListRotationsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_rotations::builders::ListRotationsInputBuilder,
}
impl ListRotationsFluentBuilder {
    /// Creates a new `ListRotations`.
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
            crate::operation::list_rotations::ListRotations,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::list_rotations::ListRotationsError>,
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
        crate::operation::list_rotations::ListRotationsOutput,
        aws_smithy_http::result::SdkError<crate::operation::list_rotations::ListRotationsError>,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_rotations::paginator::ListRotationsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::list_rotations::paginator::ListRotationsPaginator {
        crate::operation::list_rotations::paginator::ListRotationsPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>A filter to include rotations in list results based on their common prefix. For example, entering prod returns a list of all rotation names that begin with <code>prod</code>, such as <code>production</code> and <code>prod-1</code>.</p>
    pub fn rotation_name_prefix(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.rotation_name_prefix(input.into());
        self
    }
    /// <p>A filter to include rotations in list results based on their common prefix. For example, entering prod returns a list of all rotation names that begin with <code>prod</code>, such as <code>production</code> and <code>prod-1</code>.</p>
    pub fn set_rotation_name_prefix(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_rotation_name_prefix(input);
        self
    }
    /// <p>A token to start the list. Use this token to get the next set of results.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>A token to start the list. Use this token to get the next set of results.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
}
