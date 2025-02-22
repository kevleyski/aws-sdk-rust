// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::add_layer_version_permission::_add_layer_version_permission_output::AddLayerVersionPermissionOutputBuilder;

pub use crate::operation::add_layer_version_permission::_add_layer_version_permission_input::AddLayerVersionPermissionInputBuilder;

/// Fluent builder constructing a request to `AddLayerVersionPermission`.
///
/// <p>Adds permissions to the resource-based policy of a version of an <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">Lambda layer</a>. Use this action to grant layer usage permission to other accounts. You can grant permission to a single account, all accounts in an organization, or all Amazon Web Services accounts. </p>
/// <p>To revoke permission, call <code>RemoveLayerVersionPermission</code> with the statement ID that you specified when you added it.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct AddLayerVersionPermissionFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::add_layer_version_permission::builders::AddLayerVersionPermissionInputBuilder
            }
impl AddLayerVersionPermissionFluentBuilder {
    /// Creates a new `AddLayerVersionPermission`.
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
            crate::operation::add_layer_version_permission::AddLayerVersionPermission,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::add_layer_version_permission::AddLayerVersionPermissionError,
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
        crate::operation::add_layer_version_permission::AddLayerVersionPermissionOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::add_layer_version_permission::AddLayerVersionPermissionError,
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
    /// <p>The name or Amazon Resource Name (ARN) of the layer.</p>
    pub fn layer_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.layer_name(input.into());
        self
    }
    /// <p>The name or Amazon Resource Name (ARN) of the layer.</p>
    pub fn set_layer_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_layer_name(input);
        self
    }
    /// <p>The version number.</p>
    pub fn version_number(mut self, input: i64) -> Self {
        self.inner = self.inner.version_number(input);
        self
    }
    /// <p>The version number.</p>
    pub fn set_version_number(mut self, input: std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_version_number(input);
        self
    }
    /// <p>An identifier that distinguishes the policy from others on the same layer version.</p>
    pub fn statement_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.statement_id(input.into());
        self
    }
    /// <p>An identifier that distinguishes the policy from others on the same layer version.</p>
    pub fn set_statement_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_statement_id(input);
        self
    }
    /// <p>The API action that grants access to the layer. For example, <code>lambda:GetLayerVersion</code>.</p>
    pub fn action(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.action(input.into());
        self
    }
    /// <p>The API action that grants access to the layer. For example, <code>lambda:GetLayerVersion</code>.</p>
    pub fn set_action(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_action(input);
        self
    }
    /// <p>An account ID, or <code>*</code> to grant layer usage permission to all accounts in an organization, or all Amazon Web Services accounts (if <code>organizationId</code> is not specified). For the last case, make sure that you really do want all Amazon Web Services accounts to have usage permission to this layer. </p>
    pub fn principal(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.principal(input.into());
        self
    }
    /// <p>An account ID, or <code>*</code> to grant layer usage permission to all accounts in an organization, or all Amazon Web Services accounts (if <code>organizationId</code> is not specified). For the last case, make sure that you really do want all Amazon Web Services accounts to have usage permission to this layer. </p>
    pub fn set_principal(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_principal(input);
        self
    }
    /// <p>With the principal set to <code>*</code>, grant permission to all accounts in the specified organization.</p>
    pub fn organization_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.organization_id(input.into());
        self
    }
    /// <p>With the principal set to <code>*</code>, grant permission to all accounts in the specified organization.</p>
    pub fn set_organization_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_organization_id(input);
        self
    }
    /// <p>Only update the policy if the revision ID matches the ID specified. Use this option to avoid modifying a policy that has changed since you last read it.</p>
    pub fn revision_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.revision_id(input.into());
        self
    }
    /// <p>Only update the policy if the revision ID matches the ID specified. Use this option to avoid modifying a policy that has changed since you last read it.</p>
    pub fn set_revision_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_revision_id(input);
        self
    }
}
