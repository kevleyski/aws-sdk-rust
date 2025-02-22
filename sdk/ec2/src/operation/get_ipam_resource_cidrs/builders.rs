// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_ipam_resource_cidrs::_get_ipam_resource_cidrs_output::GetIpamResourceCidrsOutputBuilder;

pub use crate::operation::get_ipam_resource_cidrs::_get_ipam_resource_cidrs_input::GetIpamResourceCidrsInputBuilder;

/// Fluent builder constructing a request to `GetIpamResourceCidrs`.
///
/// <p>Returns resource CIDRs managed by IPAM in a given scope. If an IPAM is associated with more than one resource discovery, the resource CIDRs across all of the resource discoveries is returned. A resource discovery is an IPAM component that enables IPAM to manage and monitor resources that belong to the owning account.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetIpamResourceCidrsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_ipam_resource_cidrs::builders::GetIpamResourceCidrsInputBuilder,
}
impl GetIpamResourceCidrsFluentBuilder {
    /// Creates a new `GetIpamResourceCidrs`.
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
            crate::operation::get_ipam_resource_cidrs::GetIpamResourceCidrs,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::get_ipam_resource_cidrs::GetIpamResourceCidrsError,
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
        crate::operation::get_ipam_resource_cidrs::GetIpamResourceCidrsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::get_ipam_resource_cidrs::GetIpamResourceCidrsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::get_ipam_resource_cidrs::paginator::GetIpamResourceCidrsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::get_ipam_resource_cidrs::paginator::GetIpamResourceCidrsPaginator {
        crate::operation::get_ipam_resource_cidrs::paginator::GetIpamResourceCidrsPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// Appends an item to `Filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>One or more filters for the request. For more information about filtering, see <a href="https://docs.aws.amazon.com/cli/latest/userguide/cli-usage-filter.html">Filtering CLI output</a>.</p>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>One or more filters for the request. For more information about filtering, see <a href="https://docs.aws.amazon.com/cli/latest/userguide/cli-usage-filter.html">Filtering CLI output</a>.</p>
    pub fn set_filters(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Filter>>,
    ) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>The maximum number of results to return in the request.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return in the request.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The token for the next page of results.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token for the next page of results.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The ID of the scope that the resource is in.</p>
    pub fn ipam_scope_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.ipam_scope_id(input.into());
        self
    }
    /// <p>The ID of the scope that the resource is in.</p>
    pub fn set_ipam_scope_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_ipam_scope_id(input);
        self
    }
    /// <p>The ID of the IPAM pool that the resource is in.</p>
    pub fn ipam_pool_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.ipam_pool_id(input.into());
        self
    }
    /// <p>The ID of the IPAM pool that the resource is in.</p>
    pub fn set_ipam_pool_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_ipam_pool_id(input);
        self
    }
    /// <p>The ID of the resource.</p>
    pub fn resource_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.resource_id(input.into());
        self
    }
    /// <p>The ID of the resource.</p>
    pub fn set_resource_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_resource_id(input);
        self
    }
    /// <p>The resource type.</p>
    pub fn resource_type(mut self, input: crate::types::IpamResourceType) -> Self {
        self.inner = self.inner.resource_type(input);
        self
    }
    /// <p>The resource type.</p>
    pub fn set_resource_type(
        mut self,
        input: std::option::Option<crate::types::IpamResourceType>,
    ) -> Self {
        self.inner = self.inner.set_resource_type(input);
        self
    }
    /// <p>The resource tag.</p>
    pub fn resource_tag(mut self, input: crate::types::RequestIpamResourceTag) -> Self {
        self.inner = self.inner.resource_tag(input);
        self
    }
    /// <p>The resource tag.</p>
    pub fn set_resource_tag(
        mut self,
        input: std::option::Option<crate::types::RequestIpamResourceTag>,
    ) -> Self {
        self.inner = self.inner.set_resource_tag(input);
        self
    }
    /// <p>The ID of the Amazon Web Services account that owns the resource.</p>
    pub fn resource_owner(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.resource_owner(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services account that owns the resource.</p>
    pub fn set_resource_owner(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_resource_owner(input);
        self
    }
}
