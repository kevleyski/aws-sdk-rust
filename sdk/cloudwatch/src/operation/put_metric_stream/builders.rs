// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_metric_stream::_put_metric_stream_output::PutMetricStreamOutputBuilder;

pub use crate::operation::put_metric_stream::_put_metric_stream_input::PutMetricStreamInputBuilder;

/// Fluent builder constructing a request to `PutMetricStream`.
///
/// <p>Creates or updates a metric stream. Metric streams can automatically stream CloudWatch metrics to Amazon Web Services destinations, including Amazon S3, and to many third-party solutions.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch-Metric-Streams.html"> Using Metric Streams</a>.</p>
/// <p>To create a metric stream, you must be signed in to an account that has the <code>iam:PassRole</code> permission and either the <code>CloudWatchFullAccess</code> policy or the <code>cloudwatch:PutMetricStream</code> permission.</p>
/// <p>When you create or update a metric stream, you choose one of the following:</p>
/// <ul>
/// <li> <p>Stream metrics from all metric namespaces in the account.</p> </li>
/// <li> <p>Stream metrics from all metric namespaces in the account, except for the namespaces that you list in <code>ExcludeFilters</code>.</p> </li>
/// <li> <p>Stream metrics from only the metric namespaces that you list in <code>IncludeFilters</code>.</p> </li>
/// </ul>
/// <p>By default, a metric stream always sends the <code>MAX</code>, <code>MIN</code>, <code>SUM</code>, and <code>SAMPLECOUNT</code> statistics for each metric that is streamed. You can use the <code>StatisticsConfigurations</code> parameter to have the metric stream send additional statistics in the stream. Streaming additional statistics incurs additional costs. For more information, see <a href="https://aws.amazon.com/cloudwatch/pricing/">Amazon CloudWatch Pricing</a>. </p>
/// <p>When you use <code>PutMetricStream</code> to create a new metric stream, the stream is created in the <code>running</code> state. If you use it to update an existing stream, the state of the stream is not changed.</p>
/// <p>If you are using CloudWatch cross-account observability and you create a metric stream in a monitoring account, you can choose whether to include metrics from source accounts in the stream. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch-Unified-Cross-Account.html">CloudWatch cross-account observability</a>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct PutMetricStreamFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::put_metric_stream::builders::PutMetricStreamInputBuilder,
}
impl PutMetricStreamFluentBuilder {
    /// Creates a new `PutMetricStream`.
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
            crate::operation::put_metric_stream::PutMetricStream,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::put_metric_stream::PutMetricStreamError,
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
        crate::operation::put_metric_stream::PutMetricStreamOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::put_metric_stream::PutMetricStreamError,
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
    /// <p>If you are creating a new metric stream, this is the name for the new stream. The name must be different than the names of other metric streams in this account and Region.</p>
    /// <p>If you are updating a metric stream, specify the name of that stream here.</p>
    /// <p>Valid characters are A-Z, a-z, 0-9, "-" and "_".</p>
    pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>If you are creating a new metric stream, this is the name for the new stream. The name must be different than the names of other metric streams in this account and Region.</p>
    /// <p>If you are updating a metric stream, specify the name of that stream here.</p>
    /// <p>Valid characters are A-Z, a-z, 0-9, "-" and "_".</p>
    pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// Appends an item to `IncludeFilters`.
    ///
    /// To override the contents of this collection use [`set_include_filters`](Self::set_include_filters).
    ///
    /// <p>If you specify this parameter, the stream sends only the metrics from the metric namespaces that you specify here.</p>
    /// <p>You cannot include <code>IncludeFilters</code> and <code>ExcludeFilters</code> in the same operation.</p>
    pub fn include_filters(mut self, input: crate::types::MetricStreamFilter) -> Self {
        self.inner = self.inner.include_filters(input);
        self
    }
    /// <p>If you specify this parameter, the stream sends only the metrics from the metric namespaces that you specify here.</p>
    /// <p>You cannot include <code>IncludeFilters</code> and <code>ExcludeFilters</code> in the same operation.</p>
    pub fn set_include_filters(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::MetricStreamFilter>>,
    ) -> Self {
        self.inner = self.inner.set_include_filters(input);
        self
    }
    /// Appends an item to `ExcludeFilters`.
    ///
    /// To override the contents of this collection use [`set_exclude_filters`](Self::set_exclude_filters).
    ///
    /// <p>If you specify this parameter, the stream sends metrics from all metric namespaces except for the namespaces that you specify here.</p>
    /// <p>You cannot include <code>ExcludeFilters</code> and <code>IncludeFilters</code> in the same operation.</p>
    pub fn exclude_filters(mut self, input: crate::types::MetricStreamFilter) -> Self {
        self.inner = self.inner.exclude_filters(input);
        self
    }
    /// <p>If you specify this parameter, the stream sends metrics from all metric namespaces except for the namespaces that you specify here.</p>
    /// <p>You cannot include <code>ExcludeFilters</code> and <code>IncludeFilters</code> in the same operation.</p>
    pub fn set_exclude_filters(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::MetricStreamFilter>>,
    ) -> Self {
        self.inner = self.inner.set_exclude_filters(input);
        self
    }
    /// <p>The ARN of the Amazon Kinesis Data Firehose delivery stream to use for this metric stream. This Amazon Kinesis Data Firehose delivery stream must already exist and must be in the same account as the metric stream.</p>
    pub fn firehose_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.firehose_arn(input.into());
        self
    }
    /// <p>The ARN of the Amazon Kinesis Data Firehose delivery stream to use for this metric stream. This Amazon Kinesis Data Firehose delivery stream must already exist and must be in the same account as the metric stream.</p>
    pub fn set_firehose_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_firehose_arn(input);
        self
    }
    /// <p>The ARN of an IAM role that this metric stream will use to access Amazon Kinesis Data Firehose resources. This IAM role must already exist and must be in the same account as the metric stream. This IAM role must include the following permissions:</p>
    /// <ul>
    /// <li> <p>firehose:PutRecord</p> </li>
    /// <li> <p>firehose:PutRecordBatch</p> </li>
    /// </ul>
    pub fn role_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.role_arn(input.into());
        self
    }
    /// <p>The ARN of an IAM role that this metric stream will use to access Amazon Kinesis Data Firehose resources. This IAM role must already exist and must be in the same account as the metric stream. This IAM role must include the following permissions:</p>
    /// <ul>
    /// <li> <p>firehose:PutRecord</p> </li>
    /// <li> <p>firehose:PutRecordBatch</p> </li>
    /// </ul>
    pub fn set_role_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_role_arn(input);
        self
    }
    /// <p>The output format for the stream. Valid values are <code>json</code> and <code>opentelemetry0.7</code>. For more information about metric stream output formats, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch-metric-streams-formats.html"> Metric streams output formats</a>.</p>
    pub fn output_format(mut self, input: crate::types::MetricStreamOutputFormat) -> Self {
        self.inner = self.inner.output_format(input);
        self
    }
    /// <p>The output format for the stream. Valid values are <code>json</code> and <code>opentelemetry0.7</code>. For more information about metric stream output formats, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch-metric-streams-formats.html"> Metric streams output formats</a>.</p>
    pub fn set_output_format(
        mut self,
        input: std::option::Option<crate::types::MetricStreamOutputFormat>,
    ) -> Self {
        self.inner = self.inner.set_output_format(input);
        self
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A list of key-value pairs to associate with the metric stream. You can associate as many as 50 tags with a metric stream.</p>
    /// <p>Tags can help you organize and categorize your resources. You can also use them to scope user permissions by granting a user permission to access or change only resources with certain tag values.</p>
    /// <p>You can use this parameter only when you are creating a new metric stream. If you are using this operation to update an existing metric stream, any tags you specify in this parameter are ignored. To change the tags of an existing metric stream, use <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_TagResource.html">TagResource</a> or <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_UntagResource.html">UntagResource</a>.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>A list of key-value pairs to associate with the metric stream. You can associate as many as 50 tags with a metric stream.</p>
    /// <p>Tags can help you organize and categorize your resources. You can also use them to scope user permissions by granting a user permission to access or change only resources with certain tag values.</p>
    /// <p>You can use this parameter only when you are creating a new metric stream. If you are using this operation to update an existing metric stream, any tags you specify in this parameter are ignored. To change the tags of an existing metric stream, use <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_TagResource.html">TagResource</a> or <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_UntagResource.html">UntagResource</a>.</p>
    pub fn set_tags(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// Appends an item to `StatisticsConfigurations`.
    ///
    /// To override the contents of this collection use [`set_statistics_configurations`](Self::set_statistics_configurations).
    ///
    /// <p>By default, a metric stream always sends the <code>MAX</code>, <code>MIN</code>, <code>SUM</code>, and <code>SAMPLECOUNT</code> statistics for each metric that is streamed. You can use this parameter to have the metric stream also send additional statistics in the stream. This array can have up to 100 members.</p>
    /// <p>For each entry in this array, you specify one or more metrics and the list of additional statistics to stream for those metrics. The additional statistics that you can stream depend on the stream's <code>OutputFormat</code>. If the <code>OutputFormat</code> is <code>json</code>, you can stream any additional statistic that is supported by CloudWatch, listed in <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/Statistics-definitions.html.html"> CloudWatch statistics definitions</a>. If the <code>OutputFormat</code> is <code>opentelemetry0.7</code>, you can stream percentile statistics such as p95, p99.9, and so on.</p>
    pub fn statistics_configurations(
        mut self,
        input: crate::types::MetricStreamStatisticsConfiguration,
    ) -> Self {
        self.inner = self.inner.statistics_configurations(input);
        self
    }
    /// <p>By default, a metric stream always sends the <code>MAX</code>, <code>MIN</code>, <code>SUM</code>, and <code>SAMPLECOUNT</code> statistics for each metric that is streamed. You can use this parameter to have the metric stream also send additional statistics in the stream. This array can have up to 100 members.</p>
    /// <p>For each entry in this array, you specify one or more metrics and the list of additional statistics to stream for those metrics. The additional statistics that you can stream depend on the stream's <code>OutputFormat</code>. If the <code>OutputFormat</code> is <code>json</code>, you can stream any additional statistic that is supported by CloudWatch, listed in <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/Statistics-definitions.html.html"> CloudWatch statistics definitions</a>. If the <code>OutputFormat</code> is <code>opentelemetry0.7</code>, you can stream percentile statistics such as p95, p99.9, and so on.</p>
    pub fn set_statistics_configurations(
        mut self,
        input: std::option::Option<
            std::vec::Vec<crate::types::MetricStreamStatisticsConfiguration>,
        >,
    ) -> Self {
        self.inner = self.inner.set_statistics_configurations(input);
        self
    }
    /// <p>If you are creating a metric stream in a monitoring account, specify <code>true</code> to include metrics from source accounts in the metric stream.</p>
    pub fn include_linked_accounts_metrics(mut self, input: bool) -> Self {
        self.inner = self.inner.include_linked_accounts_metrics(input);
        self
    }
    /// <p>If you are creating a metric stream in a monitoring account, specify <code>true</code> to include metrics from source accounts in the metric stream.</p>
    pub fn set_include_linked_accounts_metrics(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_include_linked_accounts_metrics(input);
        self
    }
}
