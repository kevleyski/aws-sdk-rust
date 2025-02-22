// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_recommendation_report_generation::_start_recommendation_report_generation_output::StartRecommendationReportGenerationOutputBuilder;

pub use crate::operation::start_recommendation_report_generation::_start_recommendation_report_generation_input::StartRecommendationReportGenerationInputBuilder;

/// Fluent builder constructing a request to `StartRecommendationReportGeneration`.
///
/// <p> Starts generating a recommendation report. </p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct StartRecommendationReportGenerationFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::start_recommendation_report_generation::builders::StartRecommendationReportGenerationInputBuilder
            }
impl StartRecommendationReportGenerationFluentBuilder {
    /// Creates a new `StartRecommendationReportGeneration`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::start_recommendation_report_generation::StartRecommendationReportGeneration, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::start_recommendation_report_generation::StartRecommendationReportGenerationError>
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
                    pub async fn send(self) -> std::result::Result<crate::operation::start_recommendation_report_generation::StartRecommendationReportGenerationOutput, aws_smithy_http::result::SdkError<crate::operation::start_recommendation_report_generation::StartRecommendationReportGenerationError>>
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
    /// <p> The output format for the recommendation report file. The default format is Microsoft Excel. </p>
    pub fn output_format(mut self, input: crate::types::OutputFormat) -> Self {
        self.inner = self.inner.output_format(input);
        self
    }
    /// <p> The output format for the recommendation report file. The default format is Microsoft Excel. </p>
    pub fn set_output_format(
        mut self,
        input: std::option::Option<crate::types::OutputFormat>,
    ) -> Self {
        self.inner = self.inner.set_output_format(input);
        self
    }
    /// Appends an item to `groupIdFilter`.
    ///
    /// To override the contents of this collection use [`set_group_id_filter`](Self::set_group_id_filter).
    ///
    /// <p> Groups the resources in the recommendation report with a unique name. </p>
    pub fn group_id_filter(mut self, input: crate::types::Group) -> Self {
        self.inner = self.inner.group_id_filter(input);
        self
    }
    /// <p> Groups the resources in the recommendation report with a unique name. </p>
    pub fn set_group_id_filter(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Group>>,
    ) -> Self {
        self.inner = self.inner.set_group_id_filter(input);
        self
    }
}
