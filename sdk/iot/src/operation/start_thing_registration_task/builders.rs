// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_thing_registration_task::_start_thing_registration_task_output::StartThingRegistrationTaskOutputBuilder;

pub use crate::operation::start_thing_registration_task::_start_thing_registration_task_input::StartThingRegistrationTaskInputBuilder;

/// Fluent builder constructing a request to `StartThingRegistrationTask`.
///
/// <p>Creates a bulk thing provisioning task.</p>
/// <p>Requires permission to access the <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_awsiot.html#awsiot-actions-as-permissions">StartThingRegistrationTask</a> action.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct StartThingRegistrationTaskFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::start_thing_registration_task::builders::StartThingRegistrationTaskInputBuilder
            }
impl StartThingRegistrationTaskFluentBuilder {
    /// Creates a new `StartThingRegistrationTask`.
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
            crate::operation::start_thing_registration_task::StartThingRegistrationTask,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::start_thing_registration_task::StartThingRegistrationTaskError,
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
        crate::operation::start_thing_registration_task::StartThingRegistrationTaskOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::start_thing_registration_task::StartThingRegistrationTaskError,
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
    /// <p>The provisioning template.</p>
    pub fn template_body(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.template_body(input.into());
        self
    }
    /// <p>The provisioning template.</p>
    pub fn set_template_body(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_template_body(input);
        self
    }
    /// <p>The S3 bucket that contains the input file.</p>
    pub fn input_file_bucket(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.input_file_bucket(input.into());
        self
    }
    /// <p>The S3 bucket that contains the input file.</p>
    pub fn set_input_file_bucket(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_input_file_bucket(input);
        self
    }
    /// <p>The name of input file within the S3 bucket. This file contains a newline delimited JSON file. Each line contains the parameter values to provision one device (thing).</p>
    pub fn input_file_key(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.input_file_key(input.into());
        self
    }
    /// <p>The name of input file within the S3 bucket. This file contains a newline delimited JSON file. Each line contains the parameter values to provision one device (thing).</p>
    pub fn set_input_file_key(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_input_file_key(input);
        self
    }
    /// <p>The IAM role ARN that grants permission the input file.</p>
    pub fn role_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.role_arn(input.into());
        self
    }
    /// <p>The IAM role ARN that grants permission the input file.</p>
    pub fn set_role_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_role_arn(input);
        self
    }
}
