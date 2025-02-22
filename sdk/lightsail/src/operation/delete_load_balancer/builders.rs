// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_load_balancer::_delete_load_balancer_output::DeleteLoadBalancerOutputBuilder;

pub use crate::operation::delete_load_balancer::_delete_load_balancer_input::DeleteLoadBalancerInputBuilder;

/// Fluent builder constructing a request to `DeleteLoadBalancer`.
///
/// <p>Deletes a Lightsail load balancer and all its associated SSL/TLS certificates. Once the load balancer is deleted, you will need to create a new load balancer, create a new certificate, and verify domain ownership again.</p>
/// <p>The <code>delete load balancer</code> operation supports tag-based access control via resource tags applied to the resource identified by <code>load balancer name</code>. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en_us/articles/amazon-lightsail-controlling-access-using-tags">Amazon Lightsail Developer Guide</a>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteLoadBalancerFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_load_balancer::builders::DeleteLoadBalancerInputBuilder,
}
impl DeleteLoadBalancerFluentBuilder {
    /// Creates a new `DeleteLoadBalancer`.
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
            crate::operation::delete_load_balancer::DeleteLoadBalancer,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_load_balancer::DeleteLoadBalancerError,
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
        crate::operation::delete_load_balancer::DeleteLoadBalancerOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_load_balancer::DeleteLoadBalancerError,
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
    /// <p>The name of the load balancer you want to delete.</p>
    pub fn load_balancer_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.load_balancer_name(input.into());
        self
    }
    /// <p>The name of the load balancer you want to delete.</p>
    pub fn set_load_balancer_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_load_balancer_name(input);
        self
    }
}
