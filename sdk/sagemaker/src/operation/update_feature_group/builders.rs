// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_feature_group::_update_feature_group_output::UpdateFeatureGroupOutputBuilder;

pub use crate::operation::update_feature_group::_update_feature_group_input::UpdateFeatureGroupInputBuilder;

/// Fluent builder constructing a request to `UpdateFeatureGroup`.
///
/// <p>Updates the feature group.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateFeatureGroupFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_feature_group::builders::UpdateFeatureGroupInputBuilder,
}
impl UpdateFeatureGroupFluentBuilder {
    /// Creates a new `UpdateFeatureGroup`.
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
            crate::operation::update_feature_group::UpdateFeatureGroup,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::update_feature_group::UpdateFeatureGroupError,
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
        crate::operation::update_feature_group::UpdateFeatureGroupOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::update_feature_group::UpdateFeatureGroupError,
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
    /// <p>The name of the feature group that you're updating.</p>
    pub fn feature_group_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.feature_group_name(input.into());
        self
    }
    /// <p>The name of the feature group that you're updating.</p>
    pub fn set_feature_group_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_feature_group_name(input);
        self
    }
    /// Appends an item to `FeatureAdditions`.
    ///
    /// To override the contents of this collection use [`set_feature_additions`](Self::set_feature_additions).
    ///
    /// <p>Updates the feature group. Updating a feature group is an asynchronous operation. When you get an HTTP 200 response, you've made a valid request. It takes some time after you've made a valid request for Feature Store to update the feature group.</p>
    pub fn feature_additions(mut self, input: crate::types::FeatureDefinition) -> Self {
        self.inner = self.inner.feature_additions(input);
        self
    }
    /// <p>Updates the feature group. Updating a feature group is an asynchronous operation. When you get an HTTP 200 response, you've made a valid request. It takes some time after you've made a valid request for Feature Store to update the feature group.</p>
    pub fn set_feature_additions(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::FeatureDefinition>>,
    ) -> Self {
        self.inner = self.inner.set_feature_additions(input);
        self
    }
}
