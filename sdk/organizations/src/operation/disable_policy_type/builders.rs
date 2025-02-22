// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::disable_policy_type::_disable_policy_type_output::DisablePolicyTypeOutputBuilder;

pub use crate::operation::disable_policy_type::_disable_policy_type_input::DisablePolicyTypeInputBuilder;

/// Fluent builder constructing a request to `DisablePolicyType`.
///
/// <p>Disables an organizational policy type in a root. A policy of a certain type can be attached to entities in a root only if that type is enabled in the root. After you perform this operation, you no longer can attach policies of the specified type to that root or to any organizational unit (OU) or account in that root. You can undo this by using the <code>EnablePolicyType</code> operation.</p>
/// <p>This is an asynchronous request that Amazon Web Services performs in the background. If you disable a policy type for a root, it still appears enabled for the organization if <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_org_support-all-features.html">all features</a> are enabled for the organization. Amazon Web Services recommends that you first use <code>ListRoots</code> to see the status of policy types for a specified root, and then use this operation.</p>
/// <p>This operation can be called only from the organization's management account.</p>
/// <p> To view the status of available policy types in the organization, use <code>DescribeOrganization</code>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DisablePolicyTypeFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::disable_policy_type::builders::DisablePolicyTypeInputBuilder,
}
impl DisablePolicyTypeFluentBuilder {
    /// Creates a new `DisablePolicyType`.
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
            crate::operation::disable_policy_type::DisablePolicyType,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::disable_policy_type::DisablePolicyTypeError,
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
        crate::operation::disable_policy_type::DisablePolicyTypeOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::disable_policy_type::DisablePolicyTypeError,
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
    /// <p>The unique identifier (ID) of the root in which you want to disable a policy type. You can get the ID from the <code>ListRoots</code> operation.</p>
    /// <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a root ID string requires "r-" followed by from 4 to 32 lowercase letters or digits.</p>
    pub fn root_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.root_id(input.into());
        self
    }
    /// <p>The unique identifier (ID) of the root in which you want to disable a policy type. You can get the ID from the <code>ListRoots</code> operation.</p>
    /// <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a root ID string requires "r-" followed by from 4 to 32 lowercase letters or digits.</p>
    pub fn set_root_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_root_id(input);
        self
    }
    /// <p>The policy type that you want to disable in this root. You can specify one of the following values:</p>
    /// <ul>
    /// <li> <p> <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_ai-opt-out.html">AISERVICES_OPT_OUT_POLICY</a> </p> </li>
    /// <li> <p> <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_backup.html">BACKUP_POLICY</a> </p> </li>
    /// <li> <p> <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_scp.html">SERVICE_CONTROL_POLICY</a> </p> </li>
    /// <li> <p> <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_tag-policies.html">TAG_POLICY</a> </p> </li>
    /// </ul>
    pub fn policy_type(mut self, input: crate::types::PolicyType) -> Self {
        self.inner = self.inner.policy_type(input);
        self
    }
    /// <p>The policy type that you want to disable in this root. You can specify one of the following values:</p>
    /// <ul>
    /// <li> <p> <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_ai-opt-out.html">AISERVICES_OPT_OUT_POLICY</a> </p> </li>
    /// <li> <p> <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_backup.html">BACKUP_POLICY</a> </p> </li>
    /// <li> <p> <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_scp.html">SERVICE_CONTROL_POLICY</a> </p> </li>
    /// <li> <p> <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_tag-policies.html">TAG_POLICY</a> </p> </li>
    /// </ul>
    pub fn set_policy_type(mut self, input: std::option::Option<crate::types::PolicyType>) -> Self {
        self.inner = self.inner.set_policy_type(input);
        self
    }
}
