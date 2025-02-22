// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_bot::_update_bot_output::UpdateBotOutputBuilder;

pub use crate::operation::update_bot::_update_bot_input::UpdateBotInputBuilder;

/// Fluent builder constructing a request to `UpdateBot`.
///
/// <p>Updates the status of the specified bot, such as starting or stopping the bot from running in your Amazon Chime Enterprise account.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateBotFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_bot::builders::UpdateBotInputBuilder,
}
impl UpdateBotFluentBuilder {
    /// Creates a new `UpdateBot`.
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
            crate::operation::update_bot::UpdateBot,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::update_bot::UpdateBotError>,
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
        crate::operation::update_bot::UpdateBotOutput,
        aws_smithy_http::result::SdkError<crate::operation::update_bot::UpdateBotError>,
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
    /// <p>The Amazon Chime account ID.</p>
    pub fn account_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.account_id(input.into());
        self
    }
    /// <p>The Amazon Chime account ID.</p>
    pub fn set_account_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_account_id(input);
        self
    }
    /// <p>The bot ID.</p>
    pub fn bot_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.bot_id(input.into());
        self
    }
    /// <p>The bot ID.</p>
    pub fn set_bot_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_bot_id(input);
        self
    }
    /// <p>When true, stops the specified bot from running in your account.</p>
    pub fn disabled(mut self, input: bool) -> Self {
        self.inner = self.inner.disabled(input);
        self
    }
    /// <p>When true, stops the specified bot from running in your account.</p>
    pub fn set_disabled(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_disabled(input);
        self
    }
}
