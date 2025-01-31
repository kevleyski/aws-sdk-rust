// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_inline_policy_to_permission_set::_put_inline_policy_to_permission_set_output::PutInlinePolicyToPermissionSetOutputBuilder;

pub use crate::operation::put_inline_policy_to_permission_set::_put_inline_policy_to_permission_set_input::PutInlinePolicyToPermissionSetInputBuilder;

/// Fluent builder constructing a request to `PutInlinePolicyToPermissionSet`.
///
/// <p>Attaches an inline policy to a permission set.</p> <note>
/// <p>If the permission set is already referenced by one or more account assignments, you will need to call <code> <code>ProvisionPermissionSet</code> </code> after this action to apply the corresponding IAM policy updates to all assigned accounts.</p>
/// </note>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct PutInlinePolicyToPermissionSetFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::put_inline_policy_to_permission_set::builders::PutInlinePolicyToPermissionSetInputBuilder
            }
impl PutInlinePolicyToPermissionSetFluentBuilder {
    /// Creates a new `PutInlinePolicyToPermissionSet`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::put_inline_policy_to_permission_set::PutInlinePolicyToPermissionSet, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::put_inline_policy_to_permission_set::PutInlinePolicyToPermissionSetError>
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
                    pub async fn send(self) -> std::result::Result<crate::operation::put_inline_policy_to_permission_set::PutInlinePolicyToPermissionSetOutput, aws_smithy_http::result::SdkError<crate::operation::put_inline_policy_to_permission_set::PutInlinePolicyToPermissionSetError>>
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
    /// <p>The ARN of the IAM Identity Center instance under which the operation will be executed. For more information about ARNs, see <a href="/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a> in the <i>AWS General Reference</i>.</p>
    pub fn instance_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.instance_arn(input.into());
        self
    }
    /// <p>The ARN of the IAM Identity Center instance under which the operation will be executed. For more information about ARNs, see <a href="/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a> in the <i>AWS General Reference</i>.</p>
    pub fn set_instance_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_instance_arn(input);
        self
    }
    /// <p>The ARN of the permission set.</p>
    pub fn permission_set_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.permission_set_arn(input.into());
        self
    }
    /// <p>The ARN of the permission set.</p>
    pub fn set_permission_set_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_permission_set_arn(input);
        self
    }
    /// <p>The inline policy to attach to a <code>PermissionSet</code>.</p>
    pub fn inline_policy(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.inline_policy(input.into());
        self
    }
    /// <p>The inline policy to attach to a <code>PermissionSet</code>.</p>
    pub fn set_inline_policy(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_inline_policy(input);
        self
    }
}
