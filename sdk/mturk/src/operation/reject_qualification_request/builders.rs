// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::reject_qualification_request::_reject_qualification_request_output::RejectQualificationRequestOutputBuilder;

pub use crate::operation::reject_qualification_request::_reject_qualification_request_input::RejectQualificationRequestInputBuilder;

/// Fluent builder constructing a request to `RejectQualificationRequest`.
///
/// <p> The <code>RejectQualificationRequest</code> operation rejects a user's request for a Qualification. </p>
/// <p> You can provide a text message explaining why the request was rejected. The Worker who made the request can see this message.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct RejectQualificationRequestFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::reject_qualification_request::builders::RejectQualificationRequestInputBuilder
            }
impl RejectQualificationRequestFluentBuilder {
    /// Creates a new `RejectQualificationRequest`.
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
            crate::operation::reject_qualification_request::RejectQualificationRequest,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::reject_qualification_request::RejectQualificationRequestError,
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
        crate::operation::reject_qualification_request::RejectQualificationRequestOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::reject_qualification_request::RejectQualificationRequestError,
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
    /// <p> The ID of the Qualification request, as returned by the <code>ListQualificationRequests</code> operation. </p>
    pub fn qualification_request_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.qualification_request_id(input.into());
        self
    }
    /// <p> The ID of the Qualification request, as returned by the <code>ListQualificationRequests</code> operation. </p>
    pub fn set_qualification_request_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_qualification_request_id(input);
        self
    }
    /// <p>A text message explaining why the request was rejected, to be shown to the Worker who made the request.</p>
    pub fn reason(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.reason(input.into());
        self
    }
    /// <p>A text message explaining why the request was rejected, to be shown to the Worker who made the request.</p>
    pub fn set_reason(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_reason(input);
        self
    }
}
