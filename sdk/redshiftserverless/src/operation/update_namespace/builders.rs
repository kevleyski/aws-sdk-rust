// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_namespace::_update_namespace_output::UpdateNamespaceOutputBuilder;

pub use crate::operation::update_namespace::_update_namespace_input::UpdateNamespaceInputBuilder;

/// Fluent builder constructing a request to `UpdateNamespace`.
///
/// <p>Updates a namespace with the specified settings. Unless required, you can't update multiple parameters in one request. For example, you must specify both <code>adminUsername</code> and <code>adminUserPassword</code> to update either field, but you can't update both <code>kmsKeyId</code> and <code>logExports</code> in a single request.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateNamespaceFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_namespace::builders::UpdateNamespaceInputBuilder,
}
impl UpdateNamespaceFluentBuilder {
    /// Creates a new `UpdateNamespace`.
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
            crate::operation::update_namespace::UpdateNamespace,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::update_namespace::UpdateNamespaceError>,
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
        crate::operation::update_namespace::UpdateNamespaceOutput,
        aws_smithy_http::result::SdkError<crate::operation::update_namespace::UpdateNamespaceError>,
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
    /// <p>The name of the namespace to update. You can't update the name of a namespace once it is created.</p>
    pub fn namespace_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.namespace_name(input.into());
        self
    }
    /// <p>The name of the namespace to update. You can't update the name of a namespace once it is created.</p>
    pub fn set_namespace_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_namespace_name(input);
        self
    }
    /// <p>The password of the administrator for the first database created in the namespace. This parameter must be updated together with <code>adminUsername</code>.</p>
    pub fn admin_user_password(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.admin_user_password(input.into());
        self
    }
    /// <p>The password of the administrator for the first database created in the namespace. This parameter must be updated together with <code>adminUsername</code>.</p>
    pub fn set_admin_user_password(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_admin_user_password(input);
        self
    }
    /// <p>The username of the administrator for the first database created in the namespace. This parameter must be updated together with <code>adminUserPassword</code>.</p>
    pub fn admin_username(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.admin_username(input.into());
        self
    }
    /// <p>The username of the administrator for the first database created in the namespace. This parameter must be updated together with <code>adminUserPassword</code>.</p>
    pub fn set_admin_username(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_admin_username(input);
        self
    }
    /// <p>The ID of the Amazon Web Services Key Management Service key used to encrypt your data.</p>
    pub fn kms_key_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.kms_key_id(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services Key Management Service key used to encrypt your data.</p>
    pub fn set_kms_key_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_kms_key_id(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role to set as a default in the namespace. This parameter must be updated together with <code>iamRoles</code>.</p>
    pub fn default_iam_role_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.default_iam_role_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role to set as a default in the namespace. This parameter must be updated together with <code>iamRoles</code>.</p>
    pub fn set_default_iam_role_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_default_iam_role_arn(input);
        self
    }
    /// Appends an item to `iamRoles`.
    ///
    /// To override the contents of this collection use [`set_iam_roles`](Self::set_iam_roles).
    ///
    /// <p>A list of IAM roles to associate with the namespace. This parameter must be updated together with <code>defaultIamRoleArn</code>.</p>
    pub fn iam_roles(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.iam_roles(input.into());
        self
    }
    /// <p>A list of IAM roles to associate with the namespace. This parameter must be updated together with <code>defaultIamRoleArn</code>.</p>
    pub fn set_iam_roles(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_iam_roles(input);
        self
    }
    /// Appends an item to `logExports`.
    ///
    /// To override the contents of this collection use [`set_log_exports`](Self::set_log_exports).
    ///
    /// <p>The types of logs the namespace can export. The export types are <code>userlog</code>, <code>connectionlog</code>, and <code>useractivitylog</code>.</p>
    pub fn log_exports(mut self, input: crate::types::LogExport) -> Self {
        self.inner = self.inner.log_exports(input);
        self
    }
    /// <p>The types of logs the namespace can export. The export types are <code>userlog</code>, <code>connectionlog</code>, and <code>useractivitylog</code>.</p>
    pub fn set_log_exports(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::LogExport>>,
    ) -> Self {
        self.inner = self.inner.set_log_exports(input);
        self
    }
}
