// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_partner_event_source::_create_partner_event_source_output::CreatePartnerEventSourceOutputBuilder;

pub use crate::operation::create_partner_event_source::_create_partner_event_source_input::CreatePartnerEventSourceInputBuilder;

/// Fluent builder constructing a request to `CreatePartnerEventSource`.
///
/// <p>Called by an SaaS partner to create a partner event source. This operation is not used by Amazon Web Services customers.</p>
/// <p>Each partner event source can be used by one Amazon Web Services account to create a matching partner event bus in that Amazon Web Services account. A SaaS partner must create one partner event source for each Amazon Web Services account that wants to receive those event types. </p>
/// <p>A partner event source creates events based on resources within the SaaS partner's service or application.</p>
/// <p>An Amazon Web Services account that creates a partner event bus that matches the partner event source can use that event bus to receive events from the partner, and then process them using Amazon Web Services Events rules and targets.</p>
/// <p>Partner event source names follow this format:</p>
/// <p> <code> <i>partner_name</i>/<i>event_namespace</i>/<i>event_name</i> </code> </p>
/// <p> <i>partner_name</i> is determined during partner registration and identifies the partner to Amazon Web Services customers. <i>event_namespace</i> is determined by the partner and is a way for the partner to categorize their events. <i>event_name</i> is determined by the partner, and should uniquely identify an event-generating resource within the partner system. The combination of <i>event_namespace</i> and <i>event_name</i> should help Amazon Web Services customers decide whether to create an event bus to receive these events.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreatePartnerEventSourceFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::create_partner_event_source::builders::CreatePartnerEventSourceInputBuilder
            }
impl CreatePartnerEventSourceFluentBuilder {
    /// Creates a new `CreatePartnerEventSource`.
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
            crate::operation::create_partner_event_source::CreatePartnerEventSource,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::create_partner_event_source::CreatePartnerEventSourceError,
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
        crate::operation::create_partner_event_source::CreatePartnerEventSourceOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::create_partner_event_source::CreatePartnerEventSourceError,
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
    /// <p>The name of the partner event source. This name must be unique and must be in the format <code> <i>partner_name</i>/<i>event_namespace</i>/<i>event_name</i> </code>. The Amazon Web Services account that wants to use this partner event source must create a partner event bus with a name that matches the name of the partner event source.</p>
    pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the partner event source. This name must be unique and must be in the format <code> <i>partner_name</i>/<i>event_namespace</i>/<i>event_name</i> </code>. The Amazon Web Services account that wants to use this partner event source must create a partner event bus with a name that matches the name of the partner event source.</p>
    pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The Amazon Web Services account ID that is permitted to create a matching partner event bus for this partner event source.</p>
    pub fn account(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.account(input.into());
        self
    }
    /// <p>The Amazon Web Services account ID that is permitted to create a matching partner event bus for this partner event source.</p>
    pub fn set_account(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_account(input);
        self
    }
}
