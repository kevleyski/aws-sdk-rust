// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::batch_update_attendee_capabilities_except::_batch_update_attendee_capabilities_except_output::BatchUpdateAttendeeCapabilitiesExceptOutputBuilder;

pub use crate::operation::batch_update_attendee_capabilities_except::_batch_update_attendee_capabilities_except_input::BatchUpdateAttendeeCapabilitiesExceptInputBuilder;

/// Fluent builder constructing a request to `BatchUpdateAttendeeCapabilitiesExcept`.
///
/// <p>Updates <code>AttendeeCapabilities</code> except the capabilities listed in an <code>ExcludedAttendeeIds</code> table.</p> <note>
/// <p>You use the capabilities with a set of values that control what the capabilities can do, such as <code>SendReceive</code> data. For more information about those values, see .</p>
/// </note>
/// <p>When using capabilities, be aware of these corner cases:</p>
/// <ul>
/// <li> <p>You can't set <code>content</code> capabilities to <code>SendReceive</code> or <code>Receive</code> unless you also set <code>video</code> capabilities to <code>SendReceive</code> or <code>Receive</code>. If you don't set the <code>video</code> capability to receive, the response will contain an HTTP 400 Bad Request status code. However, you can set your <code>video</code> capability to receive and you set your <code>content</code> capability to not receive.</p> </li>
/// <li> <p>When you change an <code>audio</code> capability from <code>None</code> or <code>Receive</code> to <code>Send</code> or <code>SendReceive</code> , and if the attendee left their microphone unmuted, audio will flow from the attendee to the other meeting participants.</p> </li>
/// <li> <p>When you change a <code>video</code> or <code>content</code> capability from <code>None</code> or <code>Receive</code> to <code>Send</code> or <code>SendReceive</code> , and if the attendee turned on their video or content streams, remote attendess can receive those streams, but only after media renegotiation between the client and the Amazon Chime back-end server.</p> </li>
/// </ul>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct BatchUpdateAttendeeCapabilitiesExceptFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::batch_update_attendee_capabilities_except::builders::BatchUpdateAttendeeCapabilitiesExceptInputBuilder
            }
impl BatchUpdateAttendeeCapabilitiesExceptFluentBuilder {
    /// Creates a new `BatchUpdateAttendeeCapabilitiesExcept`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::batch_update_attendee_capabilities_except::BatchUpdateAttendeeCapabilitiesExcept, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::batch_update_attendee_capabilities_except::BatchUpdateAttendeeCapabilitiesExceptError>
    >{
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
                    pub async fn send(self) -> std::result::Result<crate::operation::batch_update_attendee_capabilities_except::BatchUpdateAttendeeCapabilitiesExceptOutput, aws_smithy_http::result::SdkError<crate::operation::batch_update_attendee_capabilities_except::BatchUpdateAttendeeCapabilitiesExceptError>>
                     {
        let op = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// <p>The ID of the meeting associated with the update request.</p>
    pub fn meeting_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.meeting_id(input.into());
        self
    }
    /// <p>The ID of the meeting associated with the update request.</p>
    pub fn set_meeting_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_meeting_id(input);
        self
    }
    /// Appends an item to `ExcludedAttendeeIds`.
    ///
    /// To override the contents of this collection use [`set_excluded_attendee_ids`](Self::set_excluded_attendee_ids).
    ///
    /// <p>The <code>AttendeeIDs</code> that you want to exclude from one or more capabilities.</p>
    pub fn excluded_attendee_ids(mut self, input: crate::types::AttendeeIdItem) -> Self {
        self.inner = self.inner.excluded_attendee_ids(input);
        self
    }
    /// <p>The <code>AttendeeIDs</code> that you want to exclude from one or more capabilities.</p>
    pub fn set_excluded_attendee_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::AttendeeIdItem>>,
    ) -> Self {
        self.inner = self.inner.set_excluded_attendee_ids(input);
        self
    }
    /// <p>The capabilities (<code>audio</code>, <code>video</code>, or <code>content</code>) that you want to update.</p>
    pub fn capabilities(mut self, input: crate::types::AttendeeCapabilities) -> Self {
        self.inner = self.inner.capabilities(input);
        self
    }
    /// <p>The capabilities (<code>audio</code>, <code>video</code>, or <code>content</code>) that you want to update.</p>
    pub fn set_capabilities(
        mut self,
        input: std::option::Option<crate::types::AttendeeCapabilities>,
    ) -> Self {
        self.inner = self.inner.set_capabilities(input);
        self
    }
}
