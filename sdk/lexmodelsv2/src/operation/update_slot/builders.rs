// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_slot::_update_slot_output::UpdateSlotOutputBuilder;

pub use crate::operation::update_slot::_update_slot_input::UpdateSlotInputBuilder;

/// Fluent builder constructing a request to `UpdateSlot`.
///
/// <p>Updates the settings for a slot.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateSlotFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_slot::builders::UpdateSlotInputBuilder,
}
impl UpdateSlotFluentBuilder {
    /// Creates a new `UpdateSlot`.
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
            crate::operation::update_slot::UpdateSlot,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::update_slot::UpdateSlotError>,
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
        crate::operation::update_slot::UpdateSlotOutput,
        aws_smithy_http::result::SdkError<crate::operation::update_slot::UpdateSlotError>,
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
    /// <p>The unique identifier for the slot to update.</p>
    pub fn slot_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.slot_id(input.into());
        self
    }
    /// <p>The unique identifier for the slot to update.</p>
    pub fn set_slot_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_slot_id(input);
        self
    }
    /// <p>The new name for the slot.</p>
    pub fn slot_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.slot_name(input.into());
        self
    }
    /// <p>The new name for the slot.</p>
    pub fn set_slot_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_slot_name(input);
        self
    }
    /// <p>The new description for the slot.</p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>The new description for the slot.</p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>The unique identifier of the new slot type to associate with this slot. </p>
    pub fn slot_type_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.slot_type_id(input.into());
        self
    }
    /// <p>The unique identifier of the new slot type to associate with this slot. </p>
    pub fn set_slot_type_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_slot_type_id(input);
        self
    }
    /// <p>A new set of prompts that Amazon Lex sends to the user to elicit a response the provides a value for the slot.</p>
    pub fn value_elicitation_setting(
        mut self,
        input: crate::types::SlotValueElicitationSetting,
    ) -> Self {
        self.inner = self.inner.value_elicitation_setting(input);
        self
    }
    /// <p>A new set of prompts that Amazon Lex sends to the user to elicit a response the provides a value for the slot.</p>
    pub fn set_value_elicitation_setting(
        mut self,
        input: std::option::Option<crate::types::SlotValueElicitationSetting>,
    ) -> Self {
        self.inner = self.inner.set_value_elicitation_setting(input);
        self
    }
    /// <p>New settings that determine how slot values are formatted in Amazon CloudWatch logs. </p>
    pub fn obfuscation_setting(mut self, input: crate::types::ObfuscationSetting) -> Self {
        self.inner = self.inner.obfuscation_setting(input);
        self
    }
    /// <p>New settings that determine how slot values are formatted in Amazon CloudWatch logs. </p>
    pub fn set_obfuscation_setting(
        mut self,
        input: std::option::Option<crate::types::ObfuscationSetting>,
    ) -> Self {
        self.inner = self.inner.set_obfuscation_setting(input);
        self
    }
    /// <p>The unique identifier of the bot that contains the slot.</p>
    pub fn bot_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.bot_id(input.into());
        self
    }
    /// <p>The unique identifier of the bot that contains the slot.</p>
    pub fn set_bot_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_bot_id(input);
        self
    }
    /// <p>The version of the bot that contains the slot. Must always be <code>DRAFT</code>.</p>
    pub fn bot_version(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.bot_version(input.into());
        self
    }
    /// <p>The version of the bot that contains the slot. Must always be <code>DRAFT</code>.</p>
    pub fn set_bot_version(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_bot_version(input);
        self
    }
    /// <p>The identifier of the language and locale that contains the slot. The string must match one of the supported locales. For more information, see <a href="https://docs.aws.amazon.com/lexv2/latest/dg/how-languages.html">Supported languages</a>.</p>
    pub fn locale_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.locale_id(input.into());
        self
    }
    /// <p>The identifier of the language and locale that contains the slot. The string must match one of the supported locales. For more information, see <a href="https://docs.aws.amazon.com/lexv2/latest/dg/how-languages.html">Supported languages</a>.</p>
    pub fn set_locale_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_locale_id(input);
        self
    }
    /// <p>The identifier of the intent that contains the slot.</p>
    pub fn intent_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.intent_id(input.into());
        self
    }
    /// <p>The identifier of the intent that contains the slot.</p>
    pub fn set_intent_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_intent_id(input);
        self
    }
    /// <p>Determines whether the slot accepts multiple values in one response. Multiple value slots are only available in the en-US locale. If you set this value to <code>true</code> in any other locale, Amazon Lex throws a <code>ValidationException</code>.</p>
    /// <p>If the <code>multipleValuesSetting</code> is not set, the default value is <code>false</code>.</p>
    pub fn multiple_values_setting(mut self, input: crate::types::MultipleValuesSetting) -> Self {
        self.inner = self.inner.multiple_values_setting(input);
        self
    }
    /// <p>Determines whether the slot accepts multiple values in one response. Multiple value slots are only available in the en-US locale. If you set this value to <code>true</code> in any other locale, Amazon Lex throws a <code>ValidationException</code>.</p>
    /// <p>If the <code>multipleValuesSetting</code> is not set, the default value is <code>false</code>.</p>
    pub fn set_multiple_values_setting(
        mut self,
        input: std::option::Option<crate::types::MultipleValuesSetting>,
    ) -> Self {
        self.inner = self.inner.set_multiple_values_setting(input);
        self
    }
    /// <p>Specifications for the constituent sub slots and the expression for the composite slot.</p>
    pub fn sub_slot_setting(mut self, input: crate::types::SubSlotSetting) -> Self {
        self.inner = self.inner.sub_slot_setting(input);
        self
    }
    /// <p>Specifications for the constituent sub slots and the expression for the composite slot.</p>
    pub fn set_sub_slot_setting(
        mut self,
        input: std::option::Option<crate::types::SubSlotSetting>,
    ) -> Self {
        self.inner = self.inner.set_sub_slot_setting(input);
        self
    }
}
