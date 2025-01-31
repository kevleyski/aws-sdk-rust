// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::change_tags_for_resource::_change_tags_for_resource_output::ChangeTagsForResourceOutputBuilder;

pub use crate::operation::change_tags_for_resource::_change_tags_for_resource_input::ChangeTagsForResourceInputBuilder;

/// Fluent builder constructing a request to `ChangeTagsForResource`.
///
/// <p>Adds, edits, or deletes tags for a health check or a hosted zone.</p>
/// <p>For information about using tags for cost allocation, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Using Cost Allocation Tags</a> in the <i>Billing and Cost Management User Guide</i>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ChangeTagsForResourceFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::change_tags_for_resource::builders::ChangeTagsForResourceInputBuilder,
}
impl ChangeTagsForResourceFluentBuilder {
    /// Creates a new `ChangeTagsForResource`.
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
            crate::operation::change_tags_for_resource::ChangeTagsForResource,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::change_tags_for_resource::ChangeTagsForResourceError,
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
        crate::operation::change_tags_for_resource::ChangeTagsForResourceOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::change_tags_for_resource::ChangeTagsForResourceError,
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
    /// <p>The type of the resource.</p>
    /// <ul>
    /// <li> <p>The resource type for health checks is <code>healthcheck</code>.</p> </li>
    /// <li> <p>The resource type for hosted zones is <code>hostedzone</code>.</p> </li>
    /// </ul>
    pub fn resource_type(mut self, input: crate::types::TagResourceType) -> Self {
        self.inner = self.inner.resource_type(input);
        self
    }
    /// <p>The type of the resource.</p>
    /// <ul>
    /// <li> <p>The resource type for health checks is <code>healthcheck</code>.</p> </li>
    /// <li> <p>The resource type for hosted zones is <code>hostedzone</code>.</p> </li>
    /// </ul>
    pub fn set_resource_type(
        mut self,
        input: std::option::Option<crate::types::TagResourceType>,
    ) -> Self {
        self.inner = self.inner.set_resource_type(input);
        self
    }
    /// <p>The ID of the resource for which you want to add, change, or delete tags.</p>
    pub fn resource_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.resource_id(input.into());
        self
    }
    /// <p>The ID of the resource for which you want to add, change, or delete tags.</p>
    pub fn set_resource_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_resource_id(input);
        self
    }
    /// Appends an item to `AddTags`.
    ///
    /// To override the contents of this collection use [`set_add_tags`](Self::set_add_tags).
    ///
    /// <p>A complex type that contains a list of the tags that you want to add to the specified health check or hosted zone and/or the tags that you want to edit <code>Value</code> for.</p>
    /// <p>You can add a maximum of 10 tags to a health check or a hosted zone.</p>
    pub fn add_tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.add_tags(input);
        self
    }
    /// <p>A complex type that contains a list of the tags that you want to add to the specified health check or hosted zone and/or the tags that you want to edit <code>Value</code> for.</p>
    /// <p>You can add a maximum of 10 tags to a health check or a hosted zone.</p>
    pub fn set_add_tags(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.inner = self.inner.set_add_tags(input);
        self
    }
    /// Appends an item to `RemoveTagKeys`.
    ///
    /// To override the contents of this collection use [`set_remove_tag_keys`](Self::set_remove_tag_keys).
    ///
    /// <p>A complex type that contains a list of the tags that you want to delete from the specified health check or hosted zone. You can specify up to 10 keys.</p>
    pub fn remove_tag_keys(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.remove_tag_keys(input.into());
        self
    }
    /// <p>A complex type that contains a list of the tags that you want to delete from the specified health check or hosted zone. You can specify up to 10 keys.</p>
    pub fn set_remove_tag_keys(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_remove_tag_keys(input);
        self
    }
}
