// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_metric_attribution::_update_metric_attribution_output::UpdateMetricAttributionOutputBuilder;

pub use crate::operation::update_metric_attribution::_update_metric_attribution_input::UpdateMetricAttributionInputBuilder;

/// Fluent builder constructing a request to `UpdateMetricAttribution`.
///
/// <p>Updates a metric attribution.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateMetricAttributionFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::update_metric_attribution::builders::UpdateMetricAttributionInputBuilder,
}
impl UpdateMetricAttributionFluentBuilder {
    /// Creates a new `UpdateMetricAttribution`.
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
            crate::operation::update_metric_attribution::UpdateMetricAttribution,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::update_metric_attribution::UpdateMetricAttributionError,
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
        crate::operation::update_metric_attribution::UpdateMetricAttributionOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::update_metric_attribution::UpdateMetricAttributionError,
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
    /// Appends an item to `addMetrics`.
    ///
    /// To override the contents of this collection use [`set_add_metrics`](Self::set_add_metrics).
    ///
    /// <p>Add new metric attributes to the metric attribution.</p>
    pub fn add_metrics(mut self, input: crate::types::MetricAttribute) -> Self {
        self.inner = self.inner.add_metrics(input);
        self
    }
    /// <p>Add new metric attributes to the metric attribution.</p>
    pub fn set_add_metrics(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::MetricAttribute>>,
    ) -> Self {
        self.inner = self.inner.set_add_metrics(input);
        self
    }
    /// Appends an item to `removeMetrics`.
    ///
    /// To override the contents of this collection use [`set_remove_metrics`](Self::set_remove_metrics).
    ///
    /// <p>Remove metric attributes from the metric attribution.</p>
    pub fn remove_metrics(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.remove_metrics(input.into());
        self
    }
    /// <p>Remove metric attributes from the metric attribution.</p>
    pub fn set_remove_metrics(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_remove_metrics(input);
        self
    }
    /// <p>An output config for the metric attribution.</p>
    pub fn metrics_output_config(mut self, input: crate::types::MetricAttributionOutput) -> Self {
        self.inner = self.inner.metrics_output_config(input);
        self
    }
    /// <p>An output config for the metric attribution.</p>
    pub fn set_metrics_output_config(
        mut self,
        input: std::option::Option<crate::types::MetricAttributionOutput>,
    ) -> Self {
        self.inner = self.inner.set_metrics_output_config(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) for the metric attribution to update.</p>
    pub fn metric_attribution_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.metric_attribution_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) for the metric attribution to update.</p>
    pub fn set_metric_attribution_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_metric_attribution_arn(input);
        self
    }
}
