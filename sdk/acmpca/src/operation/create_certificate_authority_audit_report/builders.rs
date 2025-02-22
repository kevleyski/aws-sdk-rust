// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_certificate_authority_audit_report::_create_certificate_authority_audit_report_output::CreateCertificateAuthorityAuditReportOutputBuilder;

pub use crate::operation::create_certificate_authority_audit_report::_create_certificate_authority_audit_report_input::CreateCertificateAuthorityAuditReportInputBuilder;

/// Fluent builder constructing a request to `CreateCertificateAuthorityAuditReport`.
///
/// <p>Creates an audit report that lists every time that your CA private key is used. The report is saved in the Amazon S3 bucket that you specify on input. The <a href="https://docs.aws.amazon.com/privateca/latest/APIReference/API_IssueCertificate.html">IssueCertificate</a> and <a href="https://docs.aws.amazon.com/privateca/latest/APIReference/API_RevokeCertificate.html">RevokeCertificate</a> actions use the private key. </p> <note>
/// <p>Both Amazon Web Services Private CA and the IAM principal must have permission to write to the S3 bucket that you specify. If the IAM principal making the call does not have permission to write to the bucket, then an exception is thrown. For more information, see <a href="https://docs.aws.amazon.com/privateca/latest/userguide/crl-planning.html#s3-policies">Access policies for CRLs in Amazon S3</a>.</p>
/// </note>
/// <p>Amazon Web Services Private CA assets that are stored in Amazon S3 can be protected with encryption. For more information, see <a href="https://docs.aws.amazon.com/privateca/latest/userguide/PcaAuditReport.html#audit-report-encryption">Encrypting Your Audit Reports</a>.</p> <note>
/// <p>You can generate a maximum of one report every 30 minutes.</p>
/// </note>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateCertificateAuthorityAuditReportFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::create_certificate_authority_audit_report::builders::CreateCertificateAuthorityAuditReportInputBuilder
            }
impl CreateCertificateAuthorityAuditReportFluentBuilder {
    /// Creates a new `CreateCertificateAuthorityAuditReport`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::create_certificate_authority_audit_report::CreateCertificateAuthorityAuditReport, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::create_certificate_authority_audit_report::CreateCertificateAuthorityAuditReportError>
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
                    pub async fn send(self) -> std::result::Result<crate::operation::create_certificate_authority_audit_report::CreateCertificateAuthorityAuditReportOutput, aws_smithy_http::result::SdkError<crate::operation::create_certificate_authority_audit_report::CreateCertificateAuthorityAuditReportError>>
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
    /// <p>The Amazon Resource Name (ARN) of the CA to be audited. This is of the form:</p>
    /// <p> <code>arn:aws:acm-pca:<i>region</i>:<i>account</i>:certificate-authority/<i>12345678-1234-1234-1234-123456789012</i> </code>.</p>
    pub fn certificate_authority_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.certificate_authority_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the CA to be audited. This is of the form:</p>
    /// <p> <code>arn:aws:acm-pca:<i>region</i>:<i>account</i>:certificate-authority/<i>12345678-1234-1234-1234-123456789012</i> </code>.</p>
    pub fn set_certificate_authority_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_certificate_authority_arn(input);
        self
    }
    /// <p>The name of the S3 bucket that will contain the audit report.</p>
    pub fn s3_bucket_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.s3_bucket_name(input.into());
        self
    }
    /// <p>The name of the S3 bucket that will contain the audit report.</p>
    pub fn set_s3_bucket_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_s3_bucket_name(input);
        self
    }
    /// <p>The format in which to create the report. This can be either <b>JSON</b> or <b>CSV</b>.</p>
    pub fn audit_report_response_format(
        mut self,
        input: crate::types::AuditReportResponseFormat,
    ) -> Self {
        self.inner = self.inner.audit_report_response_format(input);
        self
    }
    /// <p>The format in which to create the report. This can be either <b>JSON</b> or <b>CSV</b>.</p>
    pub fn set_audit_report_response_format(
        mut self,
        input: std::option::Option<crate::types::AuditReportResponseFormat>,
    ) -> Self {
        self.inner = self.inner.set_audit_report_response_format(input);
        self
    }
}
