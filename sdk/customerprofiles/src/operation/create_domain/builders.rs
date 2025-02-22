// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_domain::_create_domain_output::CreateDomainOutputBuilder;

pub use crate::operation::create_domain::_create_domain_input::CreateDomainInputBuilder;

/// Fluent builder constructing a request to `CreateDomain`.
///
/// <p>Creates a domain, which is a container for all customer data, such as customer profile attributes, object types, profile keys, and encryption keys. You can create multiple domains, and each domain can have multiple third-party integrations.</p>
/// <p>Each Amazon Connect instance can be associated with only one domain. Multiple Amazon Connect instances can be associated with one domain.</p>
/// <p>Use this API or <a href="https://docs.aws.amazon.com/customerprofiles/latest/APIReference/API_UpdateDomain.html">UpdateDomain</a> to enable <a href="https://docs.aws.amazon.com/customerprofiles/latest/APIReference/API_GetMatches.html">identity resolution</a>: set <code>Matching</code> to true. </p>
/// <p>To prevent cross-service impersonation when you call this API, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/cross-service-confused-deputy-prevention.html">Cross-service confused deputy prevention</a> for sample policies that you should apply. </p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateDomainFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_domain::builders::CreateDomainInputBuilder,
}
impl CreateDomainFluentBuilder {
    /// Creates a new `CreateDomain`.
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
            crate::operation::create_domain::CreateDomain,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::create_domain::CreateDomainError>,
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
        crate::operation::create_domain::CreateDomainOutput,
        aws_smithy_http::result::SdkError<crate::operation::create_domain::CreateDomainError>,
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
    /// <p>The unique name of the domain.</p>
    pub fn domain_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.domain_name(input.into());
        self
    }
    /// <p>The unique name of the domain.</p>
    pub fn set_domain_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_domain_name(input);
        self
    }
    /// <p>The default number of days until the data within the domain expires.</p>
    pub fn default_expiration_days(mut self, input: i32) -> Self {
        self.inner = self.inner.default_expiration_days(input);
        self
    }
    /// <p>The default number of days until the data within the domain expires.</p>
    pub fn set_default_expiration_days(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_default_expiration_days(input);
        self
    }
    /// <p>The default encryption key, which is an AWS managed key, is used when no specific type of encryption key is specified. It is used to encrypt all data before it is placed in permanent or semi-permanent storage.</p>
    pub fn default_encryption_key(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.default_encryption_key(input.into());
        self
    }
    /// <p>The default encryption key, which is an AWS managed key, is used when no specific type of encryption key is specified. It is used to encrypt all data before it is placed in permanent or semi-permanent storage.</p>
    pub fn set_default_encryption_key(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_default_encryption_key(input);
        self
    }
    /// <p>The URL of the SQS dead letter queue, which is used for reporting errors associated with ingesting data from third party applications. You must set up a policy on the DeadLetterQueue for the SendMessage operation to enable Amazon Connect Customer Profiles to send messages to the DeadLetterQueue.</p>
    pub fn dead_letter_queue_url(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.dead_letter_queue_url(input.into());
        self
    }
    /// <p>The URL of the SQS dead letter queue, which is used for reporting errors associated with ingesting data from third party applications. You must set up a policy on the DeadLetterQueue for the SendMessage operation to enable Amazon Connect Customer Profiles to send messages to the DeadLetterQueue.</p>
    pub fn set_dead_letter_queue_url(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_dead_letter_queue_url(input);
        self
    }
    /// <p>The process of matching duplicate profiles. If <code>Matching</code> = <code>true</code>, Amazon Connect Customer Profiles starts a weekly batch process called Identity Resolution Job. If you do not specify a date and time for Identity Resolution Job to run, by default it runs every Saturday at 12AM UTC to detect duplicate profiles in your domains. </p>
    /// <p>After the Identity Resolution Job completes, use the <a href="https://docs.aws.amazon.com/customerprofiles/latest/APIReference/API_GetMatches.html">GetMatches</a> API to return and review the results. Or, if you have configured <code>ExportingConfig</code> in the <code>MatchingRequest</code>, you can download the results from S3.</p>
    pub fn matching(mut self, input: crate::types::MatchingRequest) -> Self {
        self.inner = self.inner.matching(input);
        self
    }
    /// <p>The process of matching duplicate profiles. If <code>Matching</code> = <code>true</code>, Amazon Connect Customer Profiles starts a weekly batch process called Identity Resolution Job. If you do not specify a date and time for Identity Resolution Job to run, by default it runs every Saturday at 12AM UTC to detect duplicate profiles in your domains. </p>
    /// <p>After the Identity Resolution Job completes, use the <a href="https://docs.aws.amazon.com/customerprofiles/latest/APIReference/API_GetMatches.html">GetMatches</a> API to return and review the results. Or, if you have configured <code>ExportingConfig</code> in the <code>MatchingRequest</code>, you can download the results from S3.</p>
    pub fn set_matching(
        mut self,
        input: std::option::Option<crate::types::MatchingRequest>,
    ) -> Self {
        self.inner = self.inner.set_matching(input);
        self
    }
    /// Adds a key-value pair to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags used to organize, track, or control access for this resource.</p>
    pub fn tags(
        mut self,
        k: impl Into<std::string::String>,
        v: impl Into<std::string::String>,
    ) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>The tags used to organize, track, or control access for this resource.</p>
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
