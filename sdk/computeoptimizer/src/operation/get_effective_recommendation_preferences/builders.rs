// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_effective_recommendation_preferences::_get_effective_recommendation_preferences_output::GetEffectiveRecommendationPreferencesOutputBuilder;

pub use crate::operation::get_effective_recommendation_preferences::_get_effective_recommendation_preferences_input::GetEffectiveRecommendationPreferencesInputBuilder;

/// Fluent builder constructing a request to `GetEffectiveRecommendationPreferences`.
///
/// <p>Returns the recommendation preferences that are in effect for a given resource, such as enhanced infrastructure metrics. Considers all applicable preferences that you might have set at the resource, account, and organization level.</p>
/// <p>When you create a recommendation preference, you can set its status to <code>Active</code> or <code>Inactive</code>. Use this action to view the recommendation preferences that are in effect, or <code>Active</code>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetEffectiveRecommendationPreferencesFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::get_effective_recommendation_preferences::builders::GetEffectiveRecommendationPreferencesInputBuilder
            }
impl GetEffectiveRecommendationPreferencesFluentBuilder {
    /// Creates a new `GetEffectiveRecommendationPreferences`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::get_effective_recommendation_preferences::GetEffectiveRecommendationPreferences, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::get_effective_recommendation_preferences::GetEffectiveRecommendationPreferencesError>
    >{
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
                    pub async fn send(self) -> std::result::Result<crate::operation::get_effective_recommendation_preferences::GetEffectiveRecommendationPreferencesOutput, aws_smithy_http::result::SdkError<crate::operation::get_effective_recommendation_preferences::GetEffectiveRecommendationPreferencesError>>
                     {
        let op = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// <p>The Amazon Resource Name (ARN) of the resource for which to confirm effective recommendation preferences. Only EC2 instance and Auto Scaling group ARNs are currently supported.</p>
    pub fn resource_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.resource_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the resource for which to confirm effective recommendation preferences. Only EC2 instance and Auto Scaling group ARNs are currently supported.</p>
    pub fn set_resource_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_resource_arn(input);
        self
    }
}
