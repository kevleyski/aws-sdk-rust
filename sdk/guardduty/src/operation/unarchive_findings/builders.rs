// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::unarchive_findings::_unarchive_findings_output::UnarchiveFindingsOutputBuilder;

pub use crate::operation::unarchive_findings::_unarchive_findings_input::UnarchiveFindingsInputBuilder;

/// Fluent builder constructing a request to `UnarchiveFindings`.
///
/// <p>Unarchives GuardDuty findings specified by the <code>findingIds</code>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UnarchiveFindingsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::unarchive_findings::builders::UnarchiveFindingsInputBuilder,
}
impl UnarchiveFindingsFluentBuilder {
    /// Creates a new `UnarchiveFindings`.
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
            crate::operation::unarchive_findings::UnarchiveFindings,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::unarchive_findings::UnarchiveFindingsError,
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
        crate::operation::unarchive_findings::UnarchiveFindingsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::unarchive_findings::UnarchiveFindingsError,
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
    /// <p>The ID of the detector associated with the findings to unarchive.</p>
    pub fn detector_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.detector_id(input.into());
        self
    }
    /// <p>The ID of the detector associated with the findings to unarchive.</p>
    pub fn set_detector_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_detector_id(input);
        self
    }
    /// Appends an item to `FindingIds`.
    ///
    /// To override the contents of this collection use [`set_finding_ids`](Self::set_finding_ids).
    ///
    /// <p>The IDs of the findings to unarchive.</p>
    pub fn finding_ids(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.finding_ids(input.into());
        self
    }
    /// <p>The IDs of the findings to unarchive.</p>
    pub fn set_finding_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_finding_ids(input);
        self
    }
}
