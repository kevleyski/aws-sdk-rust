// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::batch_evaluate_feature::_batch_evaluate_feature_output::BatchEvaluateFeatureOutputBuilder;

pub use crate::operation::batch_evaluate_feature::_batch_evaluate_feature_input::BatchEvaluateFeatureInputBuilder;

/// Fluent builder constructing a request to `BatchEvaluateFeature`.
///
/// <p>This operation assigns feature variation to user sessions. For each user session, you pass in an <code>entityID</code> that represents the user. Evidently then checks the evaluation rules and assigns the variation.</p>
/// <p>The first rules that are evaluated are the override rules. If the user's <code>entityID</code> matches an override rule, the user is served the variation specified by that rule.</p>
/// <p>Next, if there is a launch of the feature, the user might be assigned to a variation in the launch. The chance of this depends on the percentage of users that are allocated to that launch. If the user is enrolled in the launch, the variation they are served depends on the allocation of the various feature variations used for the launch.</p>
/// <p>If the user is not assigned to a launch, and there is an ongoing experiment for this feature, the user might be assigned to a variation in the experiment. The chance of this depends on the percentage of users that are allocated to that experiment. If the user is enrolled in the experiment, the variation they are served depends on the allocation of the various feature variations used for the experiment. </p>
/// <p>If the user is not assigned to a launch or experiment, they are served the default variation.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct BatchEvaluateFeatureFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::batch_evaluate_feature::builders::BatchEvaluateFeatureInputBuilder,
}
impl BatchEvaluateFeatureFluentBuilder {
    /// Creates a new `BatchEvaluateFeature`.
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
            crate::operation::batch_evaluate_feature::BatchEvaluateFeature,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::batch_evaluate_feature::BatchEvaluateFeatureError,
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
        crate::operation::batch_evaluate_feature::BatchEvaluateFeatureOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::batch_evaluate_feature::BatchEvaluateFeatureError,
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
    /// <p>The name or ARN of the project that contains the feature being evaluated.</p>
    pub fn project(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.project(input.into());
        self
    }
    /// <p>The name or ARN of the project that contains the feature being evaluated.</p>
    pub fn set_project(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_project(input);
        self
    }
    /// Appends an item to `requests`.
    ///
    /// To override the contents of this collection use [`set_requests`](Self::set_requests).
    ///
    /// <p>An array of structures, where each structure assigns a feature variation to one user session.</p>
    pub fn requests(mut self, input: crate::types::EvaluationRequest) -> Self {
        self.inner = self.inner.requests(input);
        self
    }
    /// <p>An array of structures, where each structure assigns a feature variation to one user session.</p>
    pub fn set_requests(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::EvaluationRequest>>,
    ) -> Self {
        self.inner = self.inner.set_requests(input);
        self
    }
}
