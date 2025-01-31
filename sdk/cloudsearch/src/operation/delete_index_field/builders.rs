// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_index_field::_delete_index_field_output::DeleteIndexFieldOutputBuilder;

pub use crate::operation::delete_index_field::_delete_index_field_input::DeleteIndexFieldInputBuilder;

/// Fluent builder constructing a request to `DeleteIndexField`.
///
/// <p>Removes an <code><code>IndexField</code></code> from the search domain. For more information, see <a href="http://docs.aws.amazon.com/cloudsearch/latest/developerguide/configuring-index-fields.html" target="_blank">Configuring Index Fields</a> in the <i>Amazon CloudSearch Developer Guide</i>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteIndexFieldFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_index_field::builders::DeleteIndexFieldInputBuilder,
}
impl DeleteIndexFieldFluentBuilder {
    /// Creates a new `DeleteIndexField`.
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
            crate::operation::delete_index_field::DeleteIndexField,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_index_field::DeleteIndexFieldError,
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
        crate::operation::delete_index_field::DeleteIndexFieldOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_index_field::DeleteIndexFieldError,
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
    /// <p>A string that represents the name of a domain. Domain names are unique across the domains owned by an account within an AWS region. Domain names start with a letter or number and can contain the following characters: a-z (lowercase), 0-9, and - (hyphen).</p>
    pub fn domain_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.domain_name(input.into());
        self
    }
    /// <p>A string that represents the name of a domain. Domain names are unique across the domains owned by an account within an AWS region. Domain names start with a letter or number and can contain the following characters: a-z (lowercase), 0-9, and - (hyphen).</p>
    pub fn set_domain_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_domain_name(input);
        self
    }
    /// <p>The name of the index field your want to remove from the domain's indexing options.</p>
    pub fn index_field_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.index_field_name(input.into());
        self
    }
    /// <p>The name of the index field your want to remove from the domain's indexing options.</p>
    pub fn set_index_field_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_index_field_name(input);
        self
    }
}
