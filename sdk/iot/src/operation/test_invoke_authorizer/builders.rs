// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::test_invoke_authorizer::_test_invoke_authorizer_output::TestInvokeAuthorizerOutputBuilder;

pub use crate::operation::test_invoke_authorizer::_test_invoke_authorizer_input::TestInvokeAuthorizerInputBuilder;

/// Fluent builder constructing a request to `TestInvokeAuthorizer`.
///
/// <p>Tests a custom authorization behavior by invoking a specified custom authorizer. Use this to test and debug the custom authorization behavior of devices that connect to the IoT device gateway.</p>
/// <p>Requires permission to access the <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_awsiot.html#awsiot-actions-as-permissions">TestInvokeAuthorizer</a> action.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct TestInvokeAuthorizerFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::test_invoke_authorizer::builders::TestInvokeAuthorizerInputBuilder,
}
impl TestInvokeAuthorizerFluentBuilder {
    /// Creates a new `TestInvokeAuthorizer`.
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
            crate::operation::test_invoke_authorizer::TestInvokeAuthorizer,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::test_invoke_authorizer::TestInvokeAuthorizerError,
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
        crate::operation::test_invoke_authorizer::TestInvokeAuthorizerOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::test_invoke_authorizer::TestInvokeAuthorizerError,
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
    /// <p>The custom authorizer name.</p>
    pub fn authorizer_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.authorizer_name(input.into());
        self
    }
    /// <p>The custom authorizer name.</p>
    pub fn set_authorizer_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_authorizer_name(input);
        self
    }
    /// <p>The token returned by your custom authentication service.</p>
    pub fn token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.token(input.into());
        self
    }
    /// <p>The token returned by your custom authentication service.</p>
    pub fn set_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_token(input);
        self
    }
    /// <p>The signature made with the token and your custom authentication service's private key. This value must be Base-64-encoded.</p>
    pub fn token_signature(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.token_signature(input.into());
        self
    }
    /// <p>The signature made with the token and your custom authentication service's private key. This value must be Base-64-encoded.</p>
    pub fn set_token_signature(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_token_signature(input);
        self
    }
    /// <p>Specifies a test HTTP authorization request.</p>
    pub fn http_context(mut self, input: crate::types::HttpContext) -> Self {
        self.inner = self.inner.http_context(input);
        self
    }
    /// <p>Specifies a test HTTP authorization request.</p>
    pub fn set_http_context(
        mut self,
        input: std::option::Option<crate::types::HttpContext>,
    ) -> Self {
        self.inner = self.inner.set_http_context(input);
        self
    }
    /// <p>Specifies a test MQTT authorization request.</p>
    pub fn mqtt_context(mut self, input: crate::types::MqttContext) -> Self {
        self.inner = self.inner.mqtt_context(input);
        self
    }
    /// <p>Specifies a test MQTT authorization request.</p>
    pub fn set_mqtt_context(
        mut self,
        input: std::option::Option<crate::types::MqttContext>,
    ) -> Self {
        self.inner = self.inner.set_mqtt_context(input);
        self
    }
    /// <p>Specifies a test TLS authorization request.</p>
    pub fn tls_context(mut self, input: crate::types::TlsContext) -> Self {
        self.inner = self.inner.tls_context(input);
        self
    }
    /// <p>Specifies a test TLS authorization request.</p>
    pub fn set_tls_context(mut self, input: std::option::Option<crate::types::TlsContext>) -> Self {
        self.inner = self.inner.set_tls_context(input);
        self
    }
}
