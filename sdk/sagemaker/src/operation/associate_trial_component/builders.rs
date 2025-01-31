// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::associate_trial_component::_associate_trial_component_output::AssociateTrialComponentOutputBuilder;

pub use crate::operation::associate_trial_component::_associate_trial_component_input::AssociateTrialComponentInputBuilder;

/// Fluent builder constructing a request to `AssociateTrialComponent`.
///
/// <p>Associates a trial component with a trial. A trial component can be associated with multiple trials. To disassociate a trial component from a trial, call the <code>DisassociateTrialComponent</code> API.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct AssociateTrialComponentFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::associate_trial_component::builders::AssociateTrialComponentInputBuilder,
}
impl AssociateTrialComponentFluentBuilder {
    /// Creates a new `AssociateTrialComponent`.
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
            crate::operation::associate_trial_component::AssociateTrialComponent,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::associate_trial_component::AssociateTrialComponentError,
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
        crate::operation::associate_trial_component::AssociateTrialComponentOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::associate_trial_component::AssociateTrialComponentError,
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
    /// <p>The name of the component to associated with the trial.</p>
    pub fn trial_component_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.trial_component_name(input.into());
        self
    }
    /// <p>The name of the component to associated with the trial.</p>
    pub fn set_trial_component_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_trial_component_name(input);
        self
    }
    /// <p>The name of the trial to associate with.</p>
    pub fn trial_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.trial_name(input.into());
        self
    }
    /// <p>The name of the trial to associate with.</p>
    pub fn set_trial_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_trial_name(input);
        self
    }
}
