// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_application_states::_list_application_states_output::ListApplicationStatesOutputBuilder;

pub use crate::operation::list_application_states::_list_application_states_input::ListApplicationStatesInputBuilder;

/// Fluent builder constructing a request to `ListApplicationStates`.
///
/// <p>Lists all the migration statuses for your applications. If you use the optional <code>ApplicationIds</code> parameter, only the migration statuses for those applications will be returned.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ListApplicationStatesFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_application_states::builders::ListApplicationStatesInputBuilder,
}
impl ListApplicationStatesFluentBuilder {
    /// Creates a new `ListApplicationStates`.
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
            crate::operation::list_application_states::ListApplicationStates,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::list_application_states::ListApplicationStatesError,
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
        crate::operation::list_application_states::ListApplicationStatesOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::list_application_states::ListApplicationStatesError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_application_states::paginator::ListApplicationStatesPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::list_application_states::paginator::ListApplicationStatesPaginator {
        crate::operation::list_application_states::paginator::ListApplicationStatesPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// Appends an item to `ApplicationIds`.
    ///
    /// To override the contents of this collection use [`set_application_ids`](Self::set_application_ids).
    ///
    /// <p>The configurationIds from the Application Discovery Service that uniquely identifies your applications.</p>
    pub fn application_ids(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.application_ids(input.into());
        self
    }
    /// <p>The configurationIds from the Application Discovery Service that uniquely identifies your applications.</p>
    pub fn set_application_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_application_ids(input);
        self
    }
    /// <p>If a <code>NextToken</code> was returned by a previous call, there are more results available. To retrieve the next page of results, make the call again using the returned token in <code>NextToken</code>.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>If a <code>NextToken</code> was returned by a previous call, there are more results available. To retrieve the next page of results, make the call again using the returned token in <code>NextToken</code>.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>Maximum number of results to be returned per page.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>Maximum number of results to be returned per page.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
}
