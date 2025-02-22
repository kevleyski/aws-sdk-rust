// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::add_listener_certificates::_add_listener_certificates_output::AddListenerCertificatesOutputBuilder;

pub use crate::operation::add_listener_certificates::_add_listener_certificates_input::AddListenerCertificatesInputBuilder;

/// Fluent builder constructing a request to `AddListenerCertificates`.
///
/// <p>Adds the specified SSL server certificate to the certificate list for the specified HTTPS or TLS listener.</p>
/// <p>If the certificate in already in the certificate list, the call is successful but the certificate is not added again.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/application/create-https-listener.html">HTTPS listeners</a> in the <i>Application Load Balancers Guide</i> or <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/network/create-tls-listener.html">TLS listeners</a> in the <i>Network Load Balancers Guide</i>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct AddListenerCertificatesFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::add_listener_certificates::builders::AddListenerCertificatesInputBuilder,
}
impl AddListenerCertificatesFluentBuilder {
    /// Creates a new `AddListenerCertificates`.
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
            crate::operation::add_listener_certificates::AddListenerCertificates,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::add_listener_certificates::AddListenerCertificatesError,
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
        crate::operation::add_listener_certificates::AddListenerCertificatesOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::add_listener_certificates::AddListenerCertificatesError,
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
    /// <p>The Amazon Resource Name (ARN) of the listener.</p>
    pub fn listener_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.listener_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the listener.</p>
    pub fn set_listener_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_listener_arn(input);
        self
    }
    /// Appends an item to `Certificates`.
    ///
    /// To override the contents of this collection use [`set_certificates`](Self::set_certificates).
    ///
    /// <p>The certificate to add. You can specify one certificate per call. Set <code>CertificateArn</code> to the certificate ARN but do not set <code>IsDefault</code>.</p>
    pub fn certificates(mut self, input: crate::types::Certificate) -> Self {
        self.inner = self.inner.certificates(input);
        self
    }
    /// <p>The certificate to add. You can specify one certificate per call. Set <code>CertificateArn</code> to the certificate ARN but do not set <code>IsDefault</code>.</p>
    pub fn set_certificates(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Certificate>>,
    ) -> Self {
        self.inner = self.inner.set_certificates(input);
        self
    }
}
