// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_auto_ml_job::_create_auto_ml_job_output::CreateAutoMlJobOutputBuilder;

pub use crate::operation::create_auto_ml_job::_create_auto_ml_job_input::CreateAutoMlJobInputBuilder;

/// Fluent builder constructing a request to `CreateAutoMLJob`.
///
/// <p>Creates an Autopilot job.</p>
/// <p>Find the best-performing model after you run an Autopilot job by calling <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_DescribeAutoMLJob.html">DescribeAutoMLJob</a>.</p>
/// <p>For information about how to use Autopilot, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/autopilot-automate-model-development.html">Automate Model Development with Amazon SageMaker Autopilot</a>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateAutoMLJobFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_auto_ml_job::builders::CreateAutoMlJobInputBuilder,
}
impl CreateAutoMLJobFluentBuilder {
    /// Creates a new `CreateAutoMLJob`.
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
            crate::operation::create_auto_ml_job::CreateAutoMLJob,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::create_auto_ml_job::CreateAutoMLJobError,
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
        crate::operation::create_auto_ml_job::CreateAutoMlJobOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::create_auto_ml_job::CreateAutoMLJobError,
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
    /// <p>Identifies an Autopilot job. The name must be unique to your account and is case insensitive.</p>
    pub fn auto_ml_job_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.auto_ml_job_name(input.into());
        self
    }
    /// <p>Identifies an Autopilot job. The name must be unique to your account and is case insensitive.</p>
    pub fn set_auto_ml_job_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_auto_ml_job_name(input);
        self
    }
    /// Appends an item to `InputDataConfig`.
    ///
    /// To override the contents of this collection use [`set_input_data_config`](Self::set_input_data_config).
    ///
    /// <p>An array of channel objects that describes the input data and its location. Each channel is a named input source. Similar to <code>InputDataConfig</code> supported by <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_HyperParameterTrainingJobDefinition.html">HyperParameterTrainingJobDefinition</a>. Format(s) supported: CSV, Parquet. A minimum of 500 rows is required for the training dataset. There is not a minimum number of rows required for the validation dataset.</p>
    pub fn input_data_config(mut self, input: crate::types::AutoMlChannel) -> Self {
        self.inner = self.inner.input_data_config(input);
        self
    }
    /// <p>An array of channel objects that describes the input data and its location. Each channel is a named input source. Similar to <code>InputDataConfig</code> supported by <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_HyperParameterTrainingJobDefinition.html">HyperParameterTrainingJobDefinition</a>. Format(s) supported: CSV, Parquet. A minimum of 500 rows is required for the training dataset. There is not a minimum number of rows required for the validation dataset.</p>
    pub fn set_input_data_config(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::AutoMlChannel>>,
    ) -> Self {
        self.inner = self.inner.set_input_data_config(input);
        self
    }
    /// <p>Provides information about encryption and the Amazon S3 output path needed to store artifacts from an AutoML job. Format(s) supported: CSV.</p>
    pub fn output_data_config(mut self, input: crate::types::AutoMlOutputDataConfig) -> Self {
        self.inner = self.inner.output_data_config(input);
        self
    }
    /// <p>Provides information about encryption and the Amazon S3 output path needed to store artifacts from an AutoML job. Format(s) supported: CSV.</p>
    pub fn set_output_data_config(
        mut self,
        input: std::option::Option<crate::types::AutoMlOutputDataConfig>,
    ) -> Self {
        self.inner = self.inner.set_output_data_config(input);
        self
    }
    /// <p>Defines the type of supervised learning problem available for the candidates. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/autopilot-datasets-problem-types.html#autopilot-problem-types"> Amazon SageMaker Autopilot problem types</a>.</p>
    pub fn problem_type(mut self, input: crate::types::ProblemType) -> Self {
        self.inner = self.inner.problem_type(input);
        self
    }
    /// <p>Defines the type of supervised learning problem available for the candidates. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/autopilot-datasets-problem-types.html#autopilot-problem-types"> Amazon SageMaker Autopilot problem types</a>.</p>
    pub fn set_problem_type(
        mut self,
        input: std::option::Option<crate::types::ProblemType>,
    ) -> Self {
        self.inner = self.inner.set_problem_type(input);
        self
    }
    /// <p>Defines the objective metric used to measure the predictive quality of an AutoML job. You provide an <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_AutoMLJobObjective.html">AutoMLJobObjective$MetricName</a> and Autopilot infers whether to minimize or maximize it. For <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_CreateAutoMLJobV2.html">CreateAutoMLJobV2</a>, only <code>Accuracy</code> is supported.</p>
    pub fn auto_ml_job_objective(mut self, input: crate::types::AutoMlJobObjective) -> Self {
        self.inner = self.inner.auto_ml_job_objective(input);
        self
    }
    /// <p>Defines the objective metric used to measure the predictive quality of an AutoML job. You provide an <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_AutoMLJobObjective.html">AutoMLJobObjective$MetricName</a> and Autopilot infers whether to minimize or maximize it. For <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_CreateAutoMLJobV2.html">CreateAutoMLJobV2</a>, only <code>Accuracy</code> is supported.</p>
    pub fn set_auto_ml_job_objective(
        mut self,
        input: std::option::Option<crate::types::AutoMlJobObjective>,
    ) -> Self {
        self.inner = self.inner.set_auto_ml_job_objective(input);
        self
    }
    /// <p>A collection of settings used to configure an AutoML job.</p>
    pub fn auto_ml_job_config(mut self, input: crate::types::AutoMlJobConfig) -> Self {
        self.inner = self.inner.auto_ml_job_config(input);
        self
    }
    /// <p>A collection of settings used to configure an AutoML job.</p>
    pub fn set_auto_ml_job_config(
        mut self,
        input: std::option::Option<crate::types::AutoMlJobConfig>,
    ) -> Self {
        self.inner = self.inner.set_auto_ml_job_config(input);
        self
    }
    /// <p>The ARN of the role that is used to access the data.</p>
    pub fn role_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.role_arn(input.into());
        self
    }
    /// <p>The ARN of the role that is used to access the data.</p>
    pub fn set_role_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_role_arn(input);
        self
    }
    /// <p>Generates possible candidates without training the models. A candidate is a combination of data preprocessors, algorithms, and algorithm parameter settings.</p>
    pub fn generate_candidate_definitions_only(mut self, input: bool) -> Self {
        self.inner = self.inner.generate_candidate_definitions_only(input);
        self
    }
    /// <p>Generates possible candidates without training the models. A candidate is a combination of data preprocessors, algorithms, and algorithm parameter settings.</p>
    pub fn set_generate_candidate_definitions_only(
        mut self,
        input: std::option::Option<bool>,
    ) -> Self {
        self.inner = self.inner.set_generate_candidate_definitions_only(input);
        self
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>An array of key-value pairs. You can use tags to categorize your Amazon Web Services resources in different ways, for example, by purpose, owner, or environment. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web ServicesResources</a>. Tag keys must be unique per resource.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>An array of key-value pairs. You can use tags to categorize your Amazon Web Services resources in different ways, for example, by purpose, owner, or environment. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web ServicesResources</a>. Tag keys must be unique per resource.</p>
    pub fn set_tags(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>Specifies how to generate the endpoint name for an automatic one-click Autopilot model deployment.</p>
    pub fn model_deploy_config(mut self, input: crate::types::ModelDeployConfig) -> Self {
        self.inner = self.inner.model_deploy_config(input);
        self
    }
    /// <p>Specifies how to generate the endpoint name for an automatic one-click Autopilot model deployment.</p>
    pub fn set_model_deploy_config(
        mut self,
        input: std::option::Option<crate::types::ModelDeployConfig>,
    ) -> Self {
        self.inner = self.inner.set_model_deploy_config(input);
        self
    }
}
