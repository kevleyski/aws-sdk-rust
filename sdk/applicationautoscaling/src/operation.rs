// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `DeleteScalingPolicy`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_scaling_policy`](crate::client::Client::delete_scaling_policy).
///
/// See [`crate::client::fluent_builders::DeleteScalingPolicy`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteScalingPolicy {
    _private: (),
}
impl DeleteScalingPolicy {
    /// Creates a new builder-style object to manufacture [`DeleteScalingPolicyInput`](crate::input::DeleteScalingPolicyInput).
    pub fn builder() -> crate::input::delete_scaling_policy_input::Builder {
        crate::input::delete_scaling_policy_input::Builder::default()
    }
    /// Creates a new `DeleteScalingPolicy` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteScalingPolicy {
    type Output = std::result::Result<
        crate::output::DeleteScalingPolicyOutput,
        crate::error::DeleteScalingPolicyError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_scaling_policy_error(response)
        } else {
            crate::operation_deser::parse_delete_scaling_policy_response(response)
        }
    }
}

/// Operation shape for `DeleteScheduledAction`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_scheduled_action`](crate::client::Client::delete_scheduled_action).
///
/// See [`crate::client::fluent_builders::DeleteScheduledAction`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteScheduledAction {
    _private: (),
}
impl DeleteScheduledAction {
    /// Creates a new builder-style object to manufacture [`DeleteScheduledActionInput`](crate::input::DeleteScheduledActionInput).
    pub fn builder() -> crate::input::delete_scheduled_action_input::Builder {
        crate::input::delete_scheduled_action_input::Builder::default()
    }
    /// Creates a new `DeleteScheduledAction` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteScheduledAction {
    type Output = std::result::Result<
        crate::output::DeleteScheduledActionOutput,
        crate::error::DeleteScheduledActionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_scheduled_action_error(response)
        } else {
            crate::operation_deser::parse_delete_scheduled_action_response(response)
        }
    }
}

/// Operation shape for `DeregisterScalableTarget`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`deregister_scalable_target`](crate::client::Client::deregister_scalable_target).
///
/// See [`crate::client::fluent_builders::DeregisterScalableTarget`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeregisterScalableTarget {
    _private: (),
}
impl DeregisterScalableTarget {
    /// Creates a new builder-style object to manufacture [`DeregisterScalableTargetInput`](crate::input::DeregisterScalableTargetInput).
    pub fn builder() -> crate::input::deregister_scalable_target_input::Builder {
        crate::input::deregister_scalable_target_input::Builder::default()
    }
    /// Creates a new `DeregisterScalableTarget` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeregisterScalableTarget {
    type Output = std::result::Result<
        crate::output::DeregisterScalableTargetOutput,
        crate::error::DeregisterScalableTargetError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_deregister_scalable_target_error(response)
        } else {
            crate::operation_deser::parse_deregister_scalable_target_response(response)
        }
    }
}

/// Operation shape for `DescribeScalableTargets`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_scalable_targets`](crate::client::Client::describe_scalable_targets).
///
/// See [`crate::client::fluent_builders::DescribeScalableTargets`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeScalableTargets {
    _private: (),
}
impl DescribeScalableTargets {
    /// Creates a new builder-style object to manufacture [`DescribeScalableTargetsInput`](crate::input::DescribeScalableTargetsInput).
    pub fn builder() -> crate::input::describe_scalable_targets_input::Builder {
        crate::input::describe_scalable_targets_input::Builder::default()
    }
    /// Creates a new `DescribeScalableTargets` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeScalableTargets {
    type Output = std::result::Result<
        crate::output::DescribeScalableTargetsOutput,
        crate::error::DescribeScalableTargetsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_scalable_targets_error(response)
        } else {
            crate::operation_deser::parse_describe_scalable_targets_response(response)
        }
    }
}

/// Operation shape for `DescribeScalingActivities`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_scaling_activities`](crate::client::Client::describe_scaling_activities).
///
/// See [`crate::client::fluent_builders::DescribeScalingActivities`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeScalingActivities {
    _private: (),
}
impl DescribeScalingActivities {
    /// Creates a new builder-style object to manufacture [`DescribeScalingActivitiesInput`](crate::input::DescribeScalingActivitiesInput).
    pub fn builder() -> crate::input::describe_scaling_activities_input::Builder {
        crate::input::describe_scaling_activities_input::Builder::default()
    }
    /// Creates a new `DescribeScalingActivities` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeScalingActivities {
    type Output = std::result::Result<
        crate::output::DescribeScalingActivitiesOutput,
        crate::error::DescribeScalingActivitiesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_scaling_activities_error(response)
        } else {
            crate::operation_deser::parse_describe_scaling_activities_response(response)
        }
    }
}

/// Operation shape for `DescribeScalingPolicies`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_scaling_policies`](crate::client::Client::describe_scaling_policies).
///
/// See [`crate::client::fluent_builders::DescribeScalingPolicies`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeScalingPolicies {
    _private: (),
}
impl DescribeScalingPolicies {
    /// Creates a new builder-style object to manufacture [`DescribeScalingPoliciesInput`](crate::input::DescribeScalingPoliciesInput).
    pub fn builder() -> crate::input::describe_scaling_policies_input::Builder {
        crate::input::describe_scaling_policies_input::Builder::default()
    }
    /// Creates a new `DescribeScalingPolicies` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeScalingPolicies {
    type Output = std::result::Result<
        crate::output::DescribeScalingPoliciesOutput,
        crate::error::DescribeScalingPoliciesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_scaling_policies_error(response)
        } else {
            crate::operation_deser::parse_describe_scaling_policies_response(response)
        }
    }
}

/// Operation shape for `DescribeScheduledActions`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_scheduled_actions`](crate::client::Client::describe_scheduled_actions).
///
/// See [`crate::client::fluent_builders::DescribeScheduledActions`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeScheduledActions {
    _private: (),
}
impl DescribeScheduledActions {
    /// Creates a new builder-style object to manufacture [`DescribeScheduledActionsInput`](crate::input::DescribeScheduledActionsInput).
    pub fn builder() -> crate::input::describe_scheduled_actions_input::Builder {
        crate::input::describe_scheduled_actions_input::Builder::default()
    }
    /// Creates a new `DescribeScheduledActions` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeScheduledActions {
    type Output = std::result::Result<
        crate::output::DescribeScheduledActionsOutput,
        crate::error::DescribeScheduledActionsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_scheduled_actions_error(response)
        } else {
            crate::operation_deser::parse_describe_scheduled_actions_response(response)
        }
    }
}

/// Operation shape for `PutScalingPolicy`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`put_scaling_policy`](crate::client::Client::put_scaling_policy).
///
/// See [`crate::client::fluent_builders::PutScalingPolicy`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct PutScalingPolicy {
    _private: (),
}
impl PutScalingPolicy {
    /// Creates a new builder-style object to manufacture [`PutScalingPolicyInput`](crate::input::PutScalingPolicyInput).
    pub fn builder() -> crate::input::put_scaling_policy_input::Builder {
        crate::input::put_scaling_policy_input::Builder::default()
    }
    /// Creates a new `PutScalingPolicy` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutScalingPolicy {
    type Output = std::result::Result<
        crate::output::PutScalingPolicyOutput,
        crate::error::PutScalingPolicyError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_put_scaling_policy_error(response)
        } else {
            crate::operation_deser::parse_put_scaling_policy_response(response)
        }
    }
}

/// Operation shape for `PutScheduledAction`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`put_scheduled_action`](crate::client::Client::put_scheduled_action).
///
/// See [`crate::client::fluent_builders::PutScheduledAction`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct PutScheduledAction {
    _private: (),
}
impl PutScheduledAction {
    /// Creates a new builder-style object to manufacture [`PutScheduledActionInput`](crate::input::PutScheduledActionInput).
    pub fn builder() -> crate::input::put_scheduled_action_input::Builder {
        crate::input::put_scheduled_action_input::Builder::default()
    }
    /// Creates a new `PutScheduledAction` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutScheduledAction {
    type Output = std::result::Result<
        crate::output::PutScheduledActionOutput,
        crate::error::PutScheduledActionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_put_scheduled_action_error(response)
        } else {
            crate::operation_deser::parse_put_scheduled_action_response(response)
        }
    }
}

/// Operation shape for `RegisterScalableTarget`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`register_scalable_target`](crate::client::Client::register_scalable_target).
///
/// See [`crate::client::fluent_builders::RegisterScalableTarget`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct RegisterScalableTarget {
    _private: (),
}
impl RegisterScalableTarget {
    /// Creates a new builder-style object to manufacture [`RegisterScalableTargetInput`](crate::input::RegisterScalableTargetInput).
    pub fn builder() -> crate::input::register_scalable_target_input::Builder {
        crate::input::register_scalable_target_input::Builder::default()
    }
    /// Creates a new `RegisterScalableTarget` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for RegisterScalableTarget {
    type Output = std::result::Result<
        crate::output::RegisterScalableTargetOutput,
        crate::error::RegisterScalableTargetError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_register_scalable_target_error(response)
        } else {
            crate::operation_deser::parse_register_scalable_target_response(response)
        }
    }
}

/// Operation customization and supporting types
pub mod customize;
