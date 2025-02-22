// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_batch_segment_job::_create_batch_segment_job_output::CreateBatchSegmentJobOutputBuilder;

pub use crate::operation::create_batch_segment_job::_create_batch_segment_job_input::CreateBatchSegmentJobInputBuilder;

/// Fluent builder constructing a request to `CreateBatchSegmentJob`.
///
/// <p>Creates a batch segment job. The operation can handle up to 50 million records and the input file must be in JSON format. For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/recommendations-batch.html">Getting batch recommendations and user segments</a>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateBatchSegmentJobFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_batch_segment_job::builders::CreateBatchSegmentJobInputBuilder,
}
impl CreateBatchSegmentJobFluentBuilder {
    /// Creates a new `CreateBatchSegmentJob`.
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
            crate::operation::create_batch_segment_job::CreateBatchSegmentJob,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::create_batch_segment_job::CreateBatchSegmentJobError,
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
        crate::operation::create_batch_segment_job::CreateBatchSegmentJobOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::create_batch_segment_job::CreateBatchSegmentJobError,
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
    /// <p>The name of the batch segment job to create.</p>
    pub fn job_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.job_name(input.into());
        self
    }
    /// <p>The name of the batch segment job to create.</p>
    pub fn set_job_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_job_name(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the solution version you want the batch segment job to use to generate batch segments.</p>
    pub fn solution_version_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.solution_version_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the solution version you want the batch segment job to use to generate batch segments.</p>
    pub fn set_solution_version_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_solution_version_arn(input);
        self
    }
    /// <p>The ARN of the filter to apply to the batch segment job. For more information on using filters, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter-batch.html">Filtering batch recommendations</a>.</p>
    pub fn filter_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.filter_arn(input.into());
        self
    }
    /// <p>The ARN of the filter to apply to the batch segment job. For more information on using filters, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter-batch.html">Filtering batch recommendations</a>.</p>
    pub fn set_filter_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_filter_arn(input);
        self
    }
    /// <p>The number of predicted users generated by the batch segment job for each line of input data.</p>
    pub fn num_results(mut self, input: i32) -> Self {
        self.inner = self.inner.num_results(input);
        self
    }
    /// <p>The number of predicted users generated by the batch segment job for each line of input data.</p>
    pub fn set_num_results(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_num_results(input);
        self
    }
    /// <p>The Amazon S3 path for the input data used to generate the batch segment job.</p>
    pub fn job_input(mut self, input: crate::types::BatchSegmentJobInput) -> Self {
        self.inner = self.inner.job_input(input);
        self
    }
    /// <p>The Amazon S3 path for the input data used to generate the batch segment job.</p>
    pub fn set_job_input(
        mut self,
        input: std::option::Option<crate::types::BatchSegmentJobInput>,
    ) -> Self {
        self.inner = self.inner.set_job_input(input);
        self
    }
    /// <p>The Amazon S3 path for the bucket where the job's output will be stored.</p>
    pub fn job_output(mut self, input: crate::types::BatchSegmentJobOutput) -> Self {
        self.inner = self.inner.job_output(input);
        self
    }
    /// <p>The Amazon S3 path for the bucket where the job's output will be stored.</p>
    pub fn set_job_output(
        mut self,
        input: std::option::Option<crate::types::BatchSegmentJobOutput>,
    ) -> Self {
        self.inner = self.inner.set_job_output(input);
        self
    }
    /// <p>The ARN of the Amazon Identity and Access Management role that has permissions to read and write to your input and output Amazon S3 buckets respectively.</p>
    pub fn role_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.role_arn(input.into());
        self
    }
    /// <p>The ARN of the Amazon Identity and Access Management role that has permissions to read and write to your input and output Amazon S3 buckets respectively.</p>
    pub fn set_role_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_role_arn(input);
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A list of <a href="https://docs.aws.amazon.com/personalize/latest/dev/tagging-resources.html">tags</a> to apply to the batch segment job.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>A list of <a href="https://docs.aws.amazon.com/personalize/latest/dev/tagging-resources.html">tags</a> to apply to the batch segment job.</p>
    pub fn set_tags(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
}
