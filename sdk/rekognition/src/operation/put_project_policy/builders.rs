// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_project_policy::_put_project_policy_output::PutProjectPolicyOutputBuilder;

pub use crate::operation::put_project_policy::_put_project_policy_input::PutProjectPolicyInputBuilder;

/// Fluent builder constructing a request to `PutProjectPolicy`.
///
/// <p>Attaches a project policy to a Amazon Rekognition Custom Labels project in a trusting AWS account. A project policy specifies that a trusted AWS account can copy a model version from a trusting AWS account to a project in the trusted AWS account. To copy a model version you use the <code>CopyProjectVersion</code> operation.</p>
/// <p>For more information about the format of a project policy document, see Attaching a project policy (SDK) in the <i>Amazon Rekognition Custom Labels Developer Guide</i>. </p>
/// <p>The response from <code>PutProjectPolicy</code> is a revision ID for the project policy. You can attach multiple project policies to a project. You can also update an existing project policy by specifying the policy revision ID of the existing policy.</p>
/// <p>To remove a project policy from a project, call <code>DeleteProjectPolicy</code>. To get a list of project policies attached to a project, call <code>ListProjectPolicies</code>. </p>
/// <p>You copy a model version by calling <code>CopyProjectVersion</code>.</p>
/// <p>This operation requires permissions to perform the <code>rekognition:PutProjectPolicy</code> action.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct PutProjectPolicyFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::put_project_policy::builders::PutProjectPolicyInputBuilder,
}
impl PutProjectPolicyFluentBuilder {
    /// Creates a new `PutProjectPolicy`.
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
            crate::operation::put_project_policy::PutProjectPolicy,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::put_project_policy::PutProjectPolicyError,
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
        crate::operation::put_project_policy::PutProjectPolicyOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::put_project_policy::PutProjectPolicyError,
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
    /// <p>The Amazon Resource Name (ARN) of the project that the project policy is attached to.</p>
    pub fn project_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.project_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the project that the project policy is attached to.</p>
    pub fn set_project_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_project_arn(input);
        self
    }
    /// <p>A name for the policy.</p>
    pub fn policy_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.policy_name(input.into());
        self
    }
    /// <p>A name for the policy.</p>
    pub fn set_policy_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_policy_name(input);
        self
    }
    /// <p>The revision ID for the Project Policy. Each time you modify a policy, Amazon Rekognition Custom Labels generates and assigns a new <code>PolicyRevisionId</code> and then deletes the previous version of the policy.</p>
    pub fn policy_revision_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.policy_revision_id(input.into());
        self
    }
    /// <p>The revision ID for the Project Policy. Each time you modify a policy, Amazon Rekognition Custom Labels generates and assigns a new <code>PolicyRevisionId</code> and then deletes the previous version of the policy.</p>
    pub fn set_policy_revision_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_policy_revision_id(input);
        self
    }
    /// <p>A resource policy to add to the model. The policy is a JSON structure that contains one or more statements that define the policy. The policy must follow the IAM syntax. For more information about the contents of a JSON policy document, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies.html">IAM JSON policy reference</a>. </p>
    pub fn policy_document(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.policy_document(input.into());
        self
    }
    /// <p>A resource policy to add to the model. The policy is a JSON structure that contains one or more statements that define the policy. The policy must follow the IAM syntax. For more information about the contents of a JSON policy document, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies.html">IAM JSON policy reference</a>. </p>
    pub fn set_policy_document(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_policy_document(input);
        self
    }
}
