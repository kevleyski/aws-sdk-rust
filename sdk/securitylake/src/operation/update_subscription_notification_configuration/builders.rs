// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_subscription_notification_configuration::_update_subscription_notification_configuration_output::UpdateSubscriptionNotificationConfigurationOutputBuilder;

pub use crate::operation::update_subscription_notification_configuration::_update_subscription_notification_configuration_input::UpdateSubscriptionNotificationConfigurationInputBuilder;

/// Fluent builder constructing a request to `UpdateSubscriptionNotificationConfiguration`.
///
/// <p>Updates an existing notification method for the subscription (SQS or HTTPs endpoint) or switches the notification subscription endpoint for a subscriber.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateSubscriptionNotificationConfigurationFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::update_subscription_notification_configuration::builders::UpdateSubscriptionNotificationConfigurationInputBuilder
            }
impl UpdateSubscriptionNotificationConfigurationFluentBuilder {
    /// Creates a new `UpdateSubscriptionNotificationConfiguration`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::update_subscription_notification_configuration::UpdateSubscriptionNotificationConfiguration, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::update_subscription_notification_configuration::UpdateSubscriptionNotificationConfigurationError>
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
                    pub async fn send(self) -> std::result::Result<crate::operation::update_subscription_notification_configuration::UpdateSubscriptionNotificationConfigurationOutput, aws_smithy_http::result::SdkError<crate::operation::update_subscription_notification_configuration::UpdateSubscriptionNotificationConfigurationError>>
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
    /// <p>The subscription ID for which the subscription notification is specified. </p>
    pub fn subscription_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.subscription_id(input.into());
        self
    }
    /// <p>The subscription ID for which the subscription notification is specified. </p>
    pub fn set_subscription_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_subscription_id(input);
        self
    }
    /// <p>The subscription endpoint in Security Lake.</p>
    pub fn subscription_endpoint(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.subscription_endpoint(input.into());
        self
    }
    /// <p>The subscription endpoint in Security Lake.</p>
    pub fn set_subscription_endpoint(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_subscription_endpoint(input);
        self
    }
    /// <p>The key name for the subscription notification.</p>
    pub fn https_api_key_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.https_api_key_name(input.into());
        self
    }
    /// <p>The key name for the subscription notification.</p>
    pub fn set_https_api_key_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_https_api_key_name(input);
        self
    }
    /// <p>The key value for the subscription notification.</p>
    pub fn https_api_key_value(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.https_api_key_value(input.into());
        self
    }
    /// <p>The key value for the subscription notification.</p>
    pub fn set_https_api_key_value(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_https_api_key_value(input);
        self
    }
    /// <p>The HTTPS method used for the subscription notification. </p>
    pub fn https_method(mut self, input: crate::types::HttpsMethod) -> Self {
        self.inner = self.inner.https_method(input);
        self
    }
    /// <p>The HTTPS method used for the subscription notification. </p>
    pub fn set_https_method(
        mut self,
        input: std::option::Option<crate::types::HttpsMethod>,
    ) -> Self {
        self.inner = self.inner.set_https_method(input);
        self
    }
    /// <p>Create a new subscription notification for the specified subscription ID in Amazon Security Lake.</p>
    pub fn create_sqs(mut self, input: bool) -> Self {
        self.inner = self.inner.create_sqs(input);
        self
    }
    /// <p>Create a new subscription notification for the specified subscription ID in Amazon Security Lake.</p>
    pub fn set_create_sqs(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_create_sqs(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) specifying the role of the subscriber. For more information about ARNs and how to use them in policies, see, see the <a href="https://docs.aws.amazon.com//security-lake/latest/userguide/subscriber-data-access.html">Managing data access</a> and <a href="https://docs.aws.amazon.com/security-lake/latest/userguide/security-iam-awsmanpol.html">Amazon Web Services Managed Policies</a>in the Amazon Security Lake User Guide.</p>
    pub fn role_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.role_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) specifying the role of the subscriber. For more information about ARNs and how to use them in policies, see, see the <a href="https://docs.aws.amazon.com//security-lake/latest/userguide/subscriber-data-access.html">Managing data access</a> and <a href="https://docs.aws.amazon.com/security-lake/latest/userguide/security-iam-awsmanpol.html">Amazon Web Services Managed Policies</a>in the Amazon Security Lake User Guide.</p>
    pub fn set_role_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_role_arn(input);
        self
    }
}
