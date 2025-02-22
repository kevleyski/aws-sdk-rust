// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_application_input_processing_configuration::_delete_application_input_processing_configuration_output::DeleteApplicationInputProcessingConfigurationOutputBuilder;

pub use crate::operation::delete_application_input_processing_configuration::_delete_application_input_processing_configuration_input::DeleteApplicationInputProcessingConfigurationInputBuilder;

/// Fluent builder constructing a request to `DeleteApplicationInputProcessingConfiguration`.
///
/// <note>
/// <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p>
/// </note>
/// <p>Deletes an <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_InputProcessingConfiguration.html">InputProcessingConfiguration</a> from an input.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteApplicationInputProcessingConfigurationFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::delete_application_input_processing_configuration::builders::DeleteApplicationInputProcessingConfigurationInputBuilder
            }
impl DeleteApplicationInputProcessingConfigurationFluentBuilder {
    /// Creates a new `DeleteApplicationInputProcessingConfiguration`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::delete_application_input_processing_configuration::DeleteApplicationInputProcessingConfiguration, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::delete_application_input_processing_configuration::DeleteApplicationInputProcessingConfigurationError>
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
                    pub async fn send(self) -> std::result::Result<crate::operation::delete_application_input_processing_configuration::DeleteApplicationInputProcessingConfigurationOutput, aws_smithy_http::result::SdkError<crate::operation::delete_application_input_processing_configuration::DeleteApplicationInputProcessingConfigurationError>>
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
    /// <p>The Kinesis Analytics application name.</p>
    pub fn application_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.application_name(input.into());
        self
    }
    /// <p>The Kinesis Analytics application name.</p>
    pub fn set_application_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_application_name(input);
        self
    }
    /// <p>The version ID of the Kinesis Analytics application.</p>
    pub fn current_application_version_id(mut self, input: i64) -> Self {
        self.inner = self.inner.current_application_version_id(input);
        self
    }
    /// <p>The version ID of the Kinesis Analytics application.</p>
    pub fn set_current_application_version_id(mut self, input: std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_current_application_version_id(input);
        self
    }
    /// <p>The ID of the input configuration from which to delete the input processing configuration. You can get a list of the input IDs for an application by using the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_DescribeApplication.html">DescribeApplication</a> operation.</p>
    pub fn input_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.input_id(input.into());
        self
    }
    /// <p>The ID of the input configuration from which to delete the input processing configuration. You can get a list of the input IDs for an application by using the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_DescribeApplication.html">DescribeApplication</a> operation.</p>
    pub fn set_input_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_input_id(input);
        self
    }
}
