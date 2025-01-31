// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_model_package::_delete_model_package_output::DeleteModelPackageOutputBuilder;

pub use crate::operation::delete_model_package::_delete_model_package_input::DeleteModelPackageInputBuilder;

/// Fluent builder constructing a request to `DeleteModelPackage`.
///
/// <p>Deletes a model package.</p>
/// <p>A model package is used to create SageMaker models or list on Amazon Web Services Marketplace. Buyers can subscribe to model packages listed on Amazon Web Services Marketplace to create models in SageMaker.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteModelPackageFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_model_package::builders::DeleteModelPackageInputBuilder,
}
impl DeleteModelPackageFluentBuilder {
    /// Creates a new `DeleteModelPackage`.
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
            crate::operation::delete_model_package::DeleteModelPackage,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_model_package::DeleteModelPackageError,
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
        crate::operation::delete_model_package::DeleteModelPackageOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_model_package::DeleteModelPackageError,
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
    /// <p>The name or Amazon Resource Name (ARN) of the model package to delete.</p>
    /// <p>When you specify a name, the name must have 1 to 63 characters. Valid characters are a-z, A-Z, 0-9, and - (hyphen).</p>
    pub fn model_package_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.model_package_name(input.into());
        self
    }
    /// <p>The name or Amazon Resource Name (ARN) of the model package to delete.</p>
    /// <p>When you specify a name, the name must have 1 to 63 characters. Valid characters are a-z, A-Z, 0-9, and - (hyphen).</p>
    pub fn set_model_package_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_model_package_name(input);
        self
    }
}
