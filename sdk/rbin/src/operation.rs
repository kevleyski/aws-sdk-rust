// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CreateRule`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_rule`](crate::client::Client::create_rule).
///
/// See [`crate::client::fluent_builders::CreateRule`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateRule {
    _private: (),
}
impl CreateRule {
    /// Creates a new builder-style object to manufacture [`CreateRuleInput`](crate::input::CreateRuleInput).
    pub fn builder() -> crate::input::create_rule_input::Builder {
        crate::input::create_rule_input::Builder::default()
    }
    /// Creates a new `CreateRule` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateRule {
    type Output =
        std::result::Result<crate::output::CreateRuleOutput, crate::error::CreateRuleError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 201 {
            crate::operation_deser::parse_create_rule_error(response)
        } else {
            crate::operation_deser::parse_create_rule_response(response)
        }
    }
}

/// Operation shape for `DeleteRule`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_rule`](crate::client::Client::delete_rule).
///
/// See [`crate::client::fluent_builders::DeleteRule`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteRule {
    _private: (),
}
impl DeleteRule {
    /// Creates a new builder-style object to manufacture [`DeleteRuleInput`](crate::input::DeleteRuleInput).
    pub fn builder() -> crate::input::delete_rule_input::Builder {
        crate::input::delete_rule_input::Builder::default()
    }
    /// Creates a new `DeleteRule` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteRule {
    type Output =
        std::result::Result<crate::output::DeleteRuleOutput, crate::error::DeleteRuleError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 204 {
            crate::operation_deser::parse_delete_rule_error(response)
        } else {
            crate::operation_deser::parse_delete_rule_response(response)
        }
    }
}

/// Operation shape for `GetRule`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_rule`](crate::client::Client::get_rule).
///
/// See [`crate::client::fluent_builders::GetRule`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetRule {
    _private: (),
}
impl GetRule {
    /// Creates a new builder-style object to manufacture [`GetRuleInput`](crate::input::GetRuleInput).
    pub fn builder() -> crate::input::get_rule_input::Builder {
        crate::input::get_rule_input::Builder::default()
    }
    /// Creates a new `GetRule` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetRule {
    type Output = std::result::Result<crate::output::GetRuleOutput, crate::error::GetRuleError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_rule_error(response)
        } else {
            crate::operation_deser::parse_get_rule_response(response)
        }
    }
}

/// Operation shape for `ListRules`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_rules`](crate::client::Client::list_rules).
///
/// See [`crate::client::fluent_builders::ListRules`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListRules {
    _private: (),
}
impl ListRules {
    /// Creates a new builder-style object to manufacture [`ListRulesInput`](crate::input::ListRulesInput).
    pub fn builder() -> crate::input::list_rules_input::Builder {
        crate::input::list_rules_input::Builder::default()
    }
    /// Creates a new `ListRules` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListRules {
    type Output = std::result::Result<crate::output::ListRulesOutput, crate::error::ListRulesError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_rules_error(response)
        } else {
            crate::operation_deser::parse_list_rules_response(response)
        }
    }
}

/// Operation shape for `ListTagsForResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_tags_for_resource`](crate::client::Client::list_tags_for_resource).
///
/// See [`crate::client::fluent_builders::ListTagsForResource`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListTagsForResource {
    _private: (),
}
impl ListTagsForResource {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceInput`](crate::input::ListTagsForResourceInput).
    pub fn builder() -> crate::input::list_tags_for_resource_input::Builder {
        crate::input::list_tags_for_resource_input::Builder::default()
    }
    /// Creates a new `ListTagsForResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListTagsForResource {
    type Output = std::result::Result<
        crate::output::ListTagsForResourceOutput,
        crate::error::ListTagsForResourceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_tags_for_resource_error(response)
        } else {
            crate::operation_deser::parse_list_tags_for_resource_response(response)
        }
    }
}

/// Operation shape for `TagResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`tag_resource`](crate::client::Client::tag_resource).
///
/// See [`crate::client::fluent_builders::TagResource`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct TagResource {
    _private: (),
}
impl TagResource {
    /// Creates a new builder-style object to manufacture [`TagResourceInput`](crate::input::TagResourceInput).
    pub fn builder() -> crate::input::tag_resource_input::Builder {
        crate::input::tag_resource_input::Builder::default()
    }
    /// Creates a new `TagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for TagResource {
    type Output =
        std::result::Result<crate::output::TagResourceOutput, crate::error::TagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 201 {
            crate::operation_deser::parse_tag_resource_error(response)
        } else {
            crate::operation_deser::parse_tag_resource_response(response)
        }
    }
}

/// Operation shape for `UntagResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`untag_resource`](crate::client::Client::untag_resource).
///
/// See [`crate::client::fluent_builders::UntagResource`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UntagResource {
    _private: (),
}
impl UntagResource {
    /// Creates a new builder-style object to manufacture [`UntagResourceInput`](crate::input::UntagResourceInput).
    pub fn builder() -> crate::input::untag_resource_input::Builder {
        crate::input::untag_resource_input::Builder::default()
    }
    /// Creates a new `UntagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UntagResource {
    type Output =
        std::result::Result<crate::output::UntagResourceOutput, crate::error::UntagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 204 {
            crate::operation_deser::parse_untag_resource_error(response)
        } else {
            crate::operation_deser::parse_untag_resource_response(response)
        }
    }
}

/// Operation shape for `UpdateRule`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_rule`](crate::client::Client::update_rule).
///
/// See [`crate::client::fluent_builders::UpdateRule`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateRule {
    _private: (),
}
impl UpdateRule {
    /// Creates a new builder-style object to manufacture [`UpdateRuleInput`](crate::input::UpdateRuleInput).
    pub fn builder() -> crate::input::update_rule_input::Builder {
        crate::input::update_rule_input::Builder::default()
    }
    /// Creates a new `UpdateRule` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateRule {
    type Output =
        std::result::Result<crate::output::UpdateRuleOutput, crate::error::UpdateRuleError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_rule_error(response)
        } else {
            crate::operation_deser::parse_update_rule_response(response)
        }
    }
}

/// Operation customization and supporting types
pub mod customize;
