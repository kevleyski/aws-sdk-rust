// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::set_desired_capacity::_set_desired_capacity_output::SetDesiredCapacityOutputBuilder;

pub use crate::operation::set_desired_capacity::_set_desired_capacity_input::SetDesiredCapacityInputBuilder;

/// Fluent builder constructing a request to `SetDesiredCapacity`.
///
/// <p>Sets the size of the specified Auto Scaling group.</p>
/// <p>If a scale-in activity occurs as a result of a new <code>DesiredCapacity</code> value that is lower than the current size of the group, the Auto Scaling group uses its termination policy to determine which instances to terminate. </p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-manual-scaling.html">Manual scaling</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct SetDesiredCapacityFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::set_desired_capacity::builders::SetDesiredCapacityInputBuilder,
}
impl SetDesiredCapacityFluentBuilder {
    /// Creates a new `SetDesiredCapacity`.
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
            crate::operation::set_desired_capacity::SetDesiredCapacity,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::set_desired_capacity::SetDesiredCapacityError,
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
        crate::operation::set_desired_capacity::SetDesiredCapacityOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::set_desired_capacity::SetDesiredCapacityError,
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
    /// <p>The name of the Auto Scaling group.</p>
    pub fn auto_scaling_group_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.auto_scaling_group_name(input.into());
        self
    }
    /// <p>The name of the Auto Scaling group.</p>
    pub fn set_auto_scaling_group_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_auto_scaling_group_name(input);
        self
    }
    /// <p>The desired capacity is the initial capacity of the Auto Scaling group after this operation completes and the capacity it attempts to maintain.</p>
    pub fn desired_capacity(mut self, input: i32) -> Self {
        self.inner = self.inner.desired_capacity(input);
        self
    }
    /// <p>The desired capacity is the initial capacity of the Auto Scaling group after this operation completes and the capacity it attempts to maintain.</p>
    pub fn set_desired_capacity(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_desired_capacity(input);
        self
    }
    /// <p>Indicates whether Amazon EC2 Auto Scaling waits for the cooldown period to complete before initiating a scaling activity to set your Auto Scaling group to its new capacity. By default, Amazon EC2 Auto Scaling does not honor the cooldown period during manual scaling activities.</p>
    pub fn honor_cooldown(mut self, input: bool) -> Self {
        self.inner = self.inner.honor_cooldown(input);
        self
    }
    /// <p>Indicates whether Amazon EC2 Auto Scaling waits for the cooldown period to complete before initiating a scaling activity to set your Auto Scaling group to its new capacity. By default, Amazon EC2 Auto Scaling does not honor the cooldown period during manual scaling activities.</p>
    pub fn set_honor_cooldown(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_honor_cooldown(input);
        self
    }
}
