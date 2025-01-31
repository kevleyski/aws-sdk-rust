// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_profiling_group::_create_profiling_group_output::CreateProfilingGroupOutputBuilder;

pub use crate::operation::create_profiling_group::_create_profiling_group_input::CreateProfilingGroupInputBuilder;

/// Fluent builder constructing a request to `CreateProfilingGroup`.
///
/// <p>Creates a profiling group.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateProfilingGroupFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_profiling_group::builders::CreateProfilingGroupInputBuilder,
}
impl CreateProfilingGroupFluentBuilder {
    /// Creates a new `CreateProfilingGroup`.
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
            crate::operation::create_profiling_group::CreateProfilingGroup,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::create_profiling_group::CreateProfilingGroupError,
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
        crate::operation::create_profiling_group::CreateProfilingGroupOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::create_profiling_group::CreateProfilingGroupError,
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
    /// <p>The name of the profiling group to create.</p>
    pub fn profiling_group_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.profiling_group_name(input.into());
        self
    }
    /// <p>The name of the profiling group to create.</p>
    pub fn set_profiling_group_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_profiling_group_name(input);
        self
    }
    /// <p> The compute platform of the profiling group. Use <code>AWSLambda</code> if your application runs on AWS Lambda. Use <code>Default</code> if your application runs on a compute platform that is not AWS Lambda, such an Amazon EC2 instance, an on-premises server, or a different platform. If not specified, <code>Default</code> is used. </p>
    pub fn compute_platform(mut self, input: crate::types::ComputePlatform) -> Self {
        self.inner = self.inner.compute_platform(input);
        self
    }
    /// <p> The compute platform of the profiling group. Use <code>AWSLambda</code> if your application runs on AWS Lambda. Use <code>Default</code> if your application runs on a compute platform that is not AWS Lambda, such an Amazon EC2 instance, an on-premises server, or a different platform. If not specified, <code>Default</code> is used. </p>
    pub fn set_compute_platform(
        mut self,
        input: std::option::Option<crate::types::ComputePlatform>,
    ) -> Self {
        self.inner = self.inner.set_compute_platform(input);
        self
    }
    /// <p> Amazon CodeGuru Profiler uses this universally unique identifier (UUID) to prevent the accidental creation of duplicate profiling groups if there are failures and retries. </p>
    pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p> Amazon CodeGuru Profiler uses this universally unique identifier (UUID) to prevent the accidental creation of duplicate profiling groups if there are failures and retries. </p>
    pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p> Specifies whether profiling is enabled or disabled for the created profiling group. </p>
    pub fn agent_orchestration_config(
        mut self,
        input: crate::types::AgentOrchestrationConfig,
    ) -> Self {
        self.inner = self.inner.agent_orchestration_config(input);
        self
    }
    /// <p> Specifies whether profiling is enabled or disabled for the created profiling group. </p>
    pub fn set_agent_orchestration_config(
        mut self,
        input: std::option::Option<crate::types::AgentOrchestrationConfig>,
    ) -> Self {
        self.inner = self.inner.set_agent_orchestration_config(input);
        self
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p> A list of tags to add to the created profiling group. </p>
    pub fn tags(
        mut self,
        k: impl Into<std::string::String>,
        v: impl Into<std::string::String>,
    ) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p> A list of tags to add to the created profiling group. </p>
    pub fn set_tags(
        mut self,
        input: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
}
