// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::acknowledge_job::_acknowledge_job_output::AcknowledgeJobOutputBuilder;

pub use crate::operation::acknowledge_job::_acknowledge_job_input::AcknowledgeJobInputBuilder;

/// Fluent builder constructing a request to `AcknowledgeJob`.
///
/// <p>Returns information about a specified job and whether that job has been received by the job worker. Used for custom actions only.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct AcknowledgeJobFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::acknowledge_job::builders::AcknowledgeJobInputBuilder,
}
impl AcknowledgeJobFluentBuilder {
    /// Creates a new `AcknowledgeJob`.
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
            crate::operation::acknowledge_job::AcknowledgeJob,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::acknowledge_job::AcknowledgeJobError>,
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
        crate::operation::acknowledge_job::AcknowledgeJobOutput,
        aws_smithy_http::result::SdkError<crate::operation::acknowledge_job::AcknowledgeJobError>,
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
    /// <p>The unique system-generated ID of the job for which you want to confirm receipt.</p>
    pub fn job_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.job_id(input.into());
        self
    }
    /// <p>The unique system-generated ID of the job for which you want to confirm receipt.</p>
    pub fn set_job_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_job_id(input);
        self
    }
    /// <p>A system-generated random number that AWS CodePipeline uses to ensure that the job is being worked on by only one job worker. Get this number from the response of the <code>PollForJobs</code> request that returned this job.</p>
    pub fn nonce(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.nonce(input.into());
        self
    }
    /// <p>A system-generated random number that AWS CodePipeline uses to ensure that the job is being worked on by only one job worker. Get this number from the response of the <code>PollForJobs</code> request that returned this job.</p>
    pub fn set_nonce(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_nonce(input);
        self
    }
}
