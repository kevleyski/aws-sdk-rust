// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_third_party_firewall_association_status::_get_third_party_firewall_association_status_output::GetThirdPartyFirewallAssociationStatusOutputBuilder;

pub use crate::operation::get_third_party_firewall_association_status::_get_third_party_firewall_association_status_input::GetThirdPartyFirewallAssociationStatusInputBuilder;

/// Fluent builder constructing a request to `GetThirdPartyFirewallAssociationStatus`.
///
/// <p>The onboarding status of a Firewall Manager admin account to third-party firewall vendor tenant.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetThirdPartyFirewallAssociationStatusFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::get_third_party_firewall_association_status::builders::GetThirdPartyFirewallAssociationStatusInputBuilder
            }
impl GetThirdPartyFirewallAssociationStatusFluentBuilder {
    /// Creates a new `GetThirdPartyFirewallAssociationStatus`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::get_third_party_firewall_association_status::GetThirdPartyFirewallAssociationStatus, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::get_third_party_firewall_association_status::GetThirdPartyFirewallAssociationStatusError>
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
                    pub async fn send(self) -> std::result::Result<crate::operation::get_third_party_firewall_association_status::GetThirdPartyFirewallAssociationStatusOutput, aws_smithy_http::result::SdkError<crate::operation::get_third_party_firewall_association_status::GetThirdPartyFirewallAssociationStatusError>>
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
    /// <p>The name of the third-party firewall vendor.</p>
    pub fn third_party_firewall(mut self, input: crate::types::ThirdPartyFirewall) -> Self {
        self.inner = self.inner.third_party_firewall(input);
        self
    }
    /// <p>The name of the third-party firewall vendor.</p>
    pub fn set_third_party_firewall(
        mut self,
        input: std::option::Option<crate::types::ThirdPartyFirewall>,
    ) -> Self {
        self.inner = self.inner.set_third_party_firewall(input);
        self
    }
}
