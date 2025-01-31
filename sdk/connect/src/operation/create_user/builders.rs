// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_user::_create_user_output::CreateUserOutputBuilder;

pub use crate::operation::create_user::_create_user_input::CreateUserInputBuilder;

/// Fluent builder constructing a request to `CreateUser`.
///
/// <p>Creates a user account for the specified Amazon Connect instance.</p>
/// <p>For information about how to create user accounts using the Amazon Connect console, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/user-management.html">Add Users</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateUserFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_user::builders::CreateUserInputBuilder,
}
impl CreateUserFluentBuilder {
    /// Creates a new `CreateUser`.
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
            crate::operation::create_user::CreateUser,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::create_user::CreateUserError>,
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
        crate::operation::create_user::CreateUserOutput,
        aws_smithy_http::result::SdkError<crate::operation::create_user::CreateUserError>,
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
    /// <p>The user name for the account. For instances not using SAML for identity management, the user name can include up to 20 characters. If you are using SAML for identity management, the user name can include up to 64 characters from [a-zA-Z0-9_-.\@]+.</p>
    pub fn username(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.username(input.into());
        self
    }
    /// <p>The user name for the account. For instances not using SAML for identity management, the user name can include up to 20 characters. If you are using SAML for identity management, the user name can include up to 64 characters from [a-zA-Z0-9_-.\@]+.</p>
    pub fn set_username(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_username(input);
        self
    }
    /// <p>The password for the user account. A password is required if you are using Amazon Connect for identity management. Otherwise, it is an error to include a password.</p>
    pub fn password(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.password(input.into());
        self
    }
    /// <p>The password for the user account. A password is required if you are using Amazon Connect for identity management. Otherwise, it is an error to include a password.</p>
    pub fn set_password(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_password(input);
        self
    }
    /// <p>The information about the identity of the user.</p>
    pub fn identity_info(mut self, input: crate::types::UserIdentityInfo) -> Self {
        self.inner = self.inner.identity_info(input);
        self
    }
    /// <p>The information about the identity of the user.</p>
    pub fn set_identity_info(
        mut self,
        input: std::option::Option<crate::types::UserIdentityInfo>,
    ) -> Self {
        self.inner = self.inner.set_identity_info(input);
        self
    }
    /// <p>The phone settings for the user.</p>
    pub fn phone_config(mut self, input: crate::types::UserPhoneConfig) -> Self {
        self.inner = self.inner.phone_config(input);
        self
    }
    /// <p>The phone settings for the user.</p>
    pub fn set_phone_config(
        mut self,
        input: std::option::Option<crate::types::UserPhoneConfig>,
    ) -> Self {
        self.inner = self.inner.set_phone_config(input);
        self
    }
    /// <p>The identifier of the user account in the directory used for identity management. If Amazon Connect cannot access the directory, you can specify this identifier to authenticate users. If you include the identifier, we assume that Amazon Connect cannot access the directory. Otherwise, the identity information is used to authenticate users from your directory.</p>
    /// <p>This parameter is required if you are using an existing directory for identity management in Amazon Connect when Amazon Connect cannot access your directory to authenticate users. If you are using SAML for identity management and include this parameter, an error is returned.</p>
    pub fn directory_user_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.directory_user_id(input.into());
        self
    }
    /// <p>The identifier of the user account in the directory used for identity management. If Amazon Connect cannot access the directory, you can specify this identifier to authenticate users. If you include the identifier, we assume that Amazon Connect cannot access the directory. Otherwise, the identity information is used to authenticate users from your directory.</p>
    /// <p>This parameter is required if you are using an existing directory for identity management in Amazon Connect when Amazon Connect cannot access your directory to authenticate users. If you are using SAML for identity management and include this parameter, an error is returned.</p>
    pub fn set_directory_user_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_directory_user_id(input);
        self
    }
    /// Appends an item to `SecurityProfileIds`.
    ///
    /// To override the contents of this collection use [`set_security_profile_ids`](Self::set_security_profile_ids).
    ///
    /// <p>The identifier of the security profile for the user.</p>
    pub fn security_profile_ids(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.security_profile_ids(input.into());
        self
    }
    /// <p>The identifier of the security profile for the user.</p>
    pub fn set_security_profile_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_security_profile_ids(input);
        self
    }
    /// <p>The identifier of the routing profile for the user.</p>
    pub fn routing_profile_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.routing_profile_id(input.into());
        self
    }
    /// <p>The identifier of the routing profile for the user.</p>
    pub fn set_routing_profile_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_routing_profile_id(input);
        self
    }
    /// <p>The identifier of the hierarchy group for the user.</p>
    pub fn hierarchy_group_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.hierarchy_group_id(input.into());
        self
    }
    /// <p>The identifier of the hierarchy group for the user.</p>
    pub fn set_hierarchy_group_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_hierarchy_group_id(input);
        self
    }
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn instance_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.instance_id(input.into());
        self
    }
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn set_instance_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_instance_id(input);
        self
    }
    /// Adds a key-value pair to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags used to organize, track, or control access for this resource. For example, { "tags": {"key1":"value1", "key2":"value2"} }.</p>
    pub fn tags(
        mut self,
        k: impl Into<std::string::String>,
        v: impl Into<std::string::String>,
    ) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>The tags used to organize, track, or control access for this resource. For example, { "tags": {"key1":"value1", "key2":"value2"} }.</p>
    pub fn set_tags(
        mut self,
        input: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
}
