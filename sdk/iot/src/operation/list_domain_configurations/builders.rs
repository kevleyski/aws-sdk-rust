// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_domain_configurations::_list_domain_configurations_output::ListDomainConfigurationsOutputBuilder;

pub use crate::operation::list_domain_configurations::_list_domain_configurations_input::ListDomainConfigurationsInputBuilder;

/// Fluent builder constructing a request to `ListDomainConfigurations`.
///
/// <p>Gets a list of domain configurations for the user. This list is sorted alphabetically by domain configuration name.</p>
/// <p>Requires permission to access the <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_awsiot.html#awsiot-actions-as-permissions">ListDomainConfigurations</a> action.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ListDomainConfigurationsFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::list_domain_configurations::builders::ListDomainConfigurationsInputBuilder
            }
impl ListDomainConfigurationsFluentBuilder {
    /// Creates a new `ListDomainConfigurations`.
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
            crate::operation::list_domain_configurations::ListDomainConfigurations,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::list_domain_configurations::ListDomainConfigurationsError,
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
        crate::operation::list_domain_configurations::ListDomainConfigurationsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::list_domain_configurations::ListDomainConfigurationsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_domain_configurations::paginator::ListDomainConfigurationsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::list_domain_configurations::paginator::ListDomainConfigurationsPaginator
    {
        crate::operation::list_domain_configurations::paginator::ListDomainConfigurationsPaginator::new(self.handle, self.inner)
    }
    /// <p>The marker for the next set of results.</p>
    pub fn marker(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.marker(input.into());
        self
    }
    /// <p>The marker for the next set of results.</p>
    pub fn set_marker(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_marker(input);
        self
    }
    /// <p>The result page size.</p>
    pub fn page_size(mut self, input: i32) -> Self {
        self.inner = self.inner.page_size(input);
        self
    }
    /// <p>The result page size.</p>
    pub fn set_page_size(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_page_size(input);
        self
    }
    /// <p>The type of service delivered by the endpoint.</p>
    pub fn service_type(mut self, input: crate::types::ServiceType) -> Self {
        self.inner = self.inner.service_type(input);
        self
    }
    /// <p>The type of service delivered by the endpoint.</p>
    pub fn set_service_type(
        mut self,
        input: std::option::Option<crate::types::ServiceType>,
    ) -> Self {
        self.inner = self.inner.set_service_type(input);
        self
    }
}
