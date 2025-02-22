// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_folder::_update_folder_output::UpdateFolderOutputBuilder;

pub use crate::operation::update_folder::_update_folder_input::UpdateFolderInputBuilder;

/// Fluent builder constructing a request to `UpdateFolder`.
///
/// <p>Updates the specified attributes of the specified folder. The user must have access to both the folder and its parent folder, if applicable.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateFolderFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_folder::builders::UpdateFolderInputBuilder,
}
impl UpdateFolderFluentBuilder {
    /// Creates a new `UpdateFolder`.
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
            crate::operation::update_folder::UpdateFolder,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::update_folder::UpdateFolderError>,
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
        crate::operation::update_folder::UpdateFolderOutput,
        aws_smithy_http::result::SdkError<crate::operation::update_folder::UpdateFolderError>,
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
    /// <p>Amazon WorkDocs authentication token. Not required when using Amazon Web Services administrator credentials to access the API.</p>
    pub fn authentication_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.authentication_token(input.into());
        self
    }
    /// <p>Amazon WorkDocs authentication token. Not required when using Amazon Web Services administrator credentials to access the API.</p>
    pub fn set_authentication_token(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_authentication_token(input);
        self
    }
    /// <p>The ID of the folder.</p>
    pub fn folder_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.folder_id(input.into());
        self
    }
    /// <p>The ID of the folder.</p>
    pub fn set_folder_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_folder_id(input);
        self
    }
    /// <p>The name of the folder.</p>
    pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the folder.</p>
    pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The ID of the parent folder.</p>
    pub fn parent_folder_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.parent_folder_id(input.into());
        self
    }
    /// <p>The ID of the parent folder.</p>
    pub fn set_parent_folder_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_parent_folder_id(input);
        self
    }
    /// <p>The resource state of the folder. Only ACTIVE and RECYCLED are accepted values from the API.</p>
    pub fn resource_state(mut self, input: crate::types::ResourceStateType) -> Self {
        self.inner = self.inner.resource_state(input);
        self
    }
    /// <p>The resource state of the folder. Only ACTIVE and RECYCLED are accepted values from the API.</p>
    pub fn set_resource_state(
        mut self,
        input: std::option::Option<crate::types::ResourceStateType>,
    ) -> Self {
        self.inner = self.inner.set_resource_state(input);
        self
    }
}
