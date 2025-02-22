// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_firewall_rule_group_policy::_put_firewall_rule_group_policy_output::PutFirewallRuleGroupPolicyOutputBuilder;

pub use crate::operation::put_firewall_rule_group_policy::_put_firewall_rule_group_policy_input::PutFirewallRuleGroupPolicyInputBuilder;

/// Fluent builder constructing a request to `PutFirewallRuleGroupPolicy`.
///
/// <p>Attaches an Identity and Access Management (Amazon Web Services IAM) policy for sharing the rule group. You can use the policy to share the rule group using Resource Access Manager (RAM). </p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct PutFirewallRuleGroupPolicyFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::put_firewall_rule_group_policy::builders::PutFirewallRuleGroupPolicyInputBuilder
            }
impl PutFirewallRuleGroupPolicyFluentBuilder {
    /// Creates a new `PutFirewallRuleGroupPolicy`.
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
            crate::operation::put_firewall_rule_group_policy::PutFirewallRuleGroupPolicy,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::put_firewall_rule_group_policy::PutFirewallRuleGroupPolicyError,
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
        crate::operation::put_firewall_rule_group_policy::PutFirewallRuleGroupPolicyOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::put_firewall_rule_group_policy::PutFirewallRuleGroupPolicyError,
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
    /// <p>The ARN (Amazon Resource Name) for the rule group that you want to share.</p>
    pub fn arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.arn(input.into());
        self
    }
    /// <p>The ARN (Amazon Resource Name) for the rule group that you want to share.</p>
    pub fn set_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_arn(input);
        self
    }
    /// <p>The Identity and Access Management (Amazon Web Services IAM) policy to attach to the rule group.</p>
    pub fn firewall_rule_group_policy(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.firewall_rule_group_policy(input.into());
        self
    }
    /// <p>The Identity and Access Management (Amazon Web Services IAM) policy to attach to the rule group.</p>
    pub fn set_firewall_rule_group_policy(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_firewall_rule_group_policy(input);
        self
    }
}
