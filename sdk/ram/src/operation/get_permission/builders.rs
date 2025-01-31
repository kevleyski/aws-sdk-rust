// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_permission::_get_permission_output::GetPermissionOutputBuilder;

pub use crate::operation::get_permission::_get_permission_input::GetPermissionInputBuilder;

/// Fluent builder constructing a request to `GetPermission`.
///
/// <p>Gets the contents of an RAM permission in JSON format.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetPermissionFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_permission::builders::GetPermissionInputBuilder,
}
impl GetPermissionFluentBuilder {
    /// Creates a new `GetPermission`.
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
            crate::operation::get_permission::GetPermission,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::get_permission::GetPermissionError>,
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
        crate::operation::get_permission::GetPermissionOutput,
        aws_smithy_http::result::SdkError<crate::operation::get_permission::GetPermissionError>,
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
    /// <p>Specifies the <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resoure Name (ARN)</a> of the permission whose contents you want to retrieve. To find the ARN for a permission, use either the <code>ListPermissions</code> operation or go to the <a href="https://console.aws.amazon.com/ram/home#Permissions:">Permissions library</a> page in the RAM console and then choose the name of the permission. The ARN is displayed on the detail page.</p>
    pub fn permission_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.permission_arn(input.into());
        self
    }
    /// <p>Specifies the <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resoure Name (ARN)</a> of the permission whose contents you want to retrieve. To find the ARN for a permission, use either the <code>ListPermissions</code> operation or go to the <a href="https://console.aws.amazon.com/ram/home#Permissions:">Permissions library</a> page in the RAM console and then choose the name of the permission. The ARN is displayed on the detail page.</p>
    pub fn set_permission_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_permission_arn(input);
        self
    }
    /// <p>Specifies identifier for the version of the RAM permission to retrieve. If you don't specify this parameter, the operation retrieves the default version.</p>
    pub fn permission_version(mut self, input: i32) -> Self {
        self.inner = self.inner.permission_version(input);
        self
    }
    /// <p>Specifies identifier for the version of the RAM permission to retrieve. If you don't specify this parameter, the operation retrieves the default version.</p>
    pub fn set_permission_version(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_permission_version(input);
        self
    }
}
