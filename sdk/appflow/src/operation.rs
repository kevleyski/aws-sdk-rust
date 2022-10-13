// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CreateConnectorProfile`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_connector_profile`](crate::client::Client::create_connector_profile).
///
/// See [`crate::client::fluent_builders::CreateConnectorProfile`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateConnectorProfile {
    _private: (),
}
impl CreateConnectorProfile {
    /// Creates a new builder-style object to manufacture [`CreateConnectorProfileInput`](crate::input::CreateConnectorProfileInput).
    pub fn builder() -> crate::input::create_connector_profile_input::Builder {
        crate::input::create_connector_profile_input::Builder::default()
    }
    /// Creates a new `CreateConnectorProfile` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateConnectorProfile {
    type Output = std::result::Result<
        crate::output::CreateConnectorProfileOutput,
        crate::error::CreateConnectorProfileError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_connector_profile_error(response)
        } else {
            crate::operation_deser::parse_create_connector_profile_response(response)
        }
    }
}

/// Operation shape for `CreateFlow`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_flow`](crate::client::Client::create_flow).
///
/// See [`crate::client::fluent_builders::CreateFlow`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateFlow {
    _private: (),
}
impl CreateFlow {
    /// Creates a new builder-style object to manufacture [`CreateFlowInput`](crate::input::CreateFlowInput).
    pub fn builder() -> crate::input::create_flow_input::Builder {
        crate::input::create_flow_input::Builder::default()
    }
    /// Creates a new `CreateFlow` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateFlow {
    type Output =
        std::result::Result<crate::output::CreateFlowOutput, crate::error::CreateFlowError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_flow_error(response)
        } else {
            crate::operation_deser::parse_create_flow_response(response)
        }
    }
}

/// Operation shape for `DeleteConnectorProfile`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_connector_profile`](crate::client::Client::delete_connector_profile).
///
/// See [`crate::client::fluent_builders::DeleteConnectorProfile`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteConnectorProfile {
    _private: (),
}
impl DeleteConnectorProfile {
    /// Creates a new builder-style object to manufacture [`DeleteConnectorProfileInput`](crate::input::DeleteConnectorProfileInput).
    pub fn builder() -> crate::input::delete_connector_profile_input::Builder {
        crate::input::delete_connector_profile_input::Builder::default()
    }
    /// Creates a new `DeleteConnectorProfile` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteConnectorProfile {
    type Output = std::result::Result<
        crate::output::DeleteConnectorProfileOutput,
        crate::error::DeleteConnectorProfileError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_connector_profile_error(response)
        } else {
            crate::operation_deser::parse_delete_connector_profile_response(response)
        }
    }
}

/// Operation shape for `DeleteFlow`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_flow`](crate::client::Client::delete_flow).
///
/// See [`crate::client::fluent_builders::DeleteFlow`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteFlow {
    _private: (),
}
impl DeleteFlow {
    /// Creates a new builder-style object to manufacture [`DeleteFlowInput`](crate::input::DeleteFlowInput).
    pub fn builder() -> crate::input::delete_flow_input::Builder {
        crate::input::delete_flow_input::Builder::default()
    }
    /// Creates a new `DeleteFlow` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteFlow {
    type Output =
        std::result::Result<crate::output::DeleteFlowOutput, crate::error::DeleteFlowError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_flow_error(response)
        } else {
            crate::operation_deser::parse_delete_flow_response(response)
        }
    }
}

/// Operation shape for `DescribeConnector`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_connector`](crate::client::Client::describe_connector).
///
/// See [`crate::client::fluent_builders::DescribeConnector`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeConnector {
    _private: (),
}
impl DescribeConnector {
    /// Creates a new builder-style object to manufacture [`DescribeConnectorInput`](crate::input::DescribeConnectorInput).
    pub fn builder() -> crate::input::describe_connector_input::Builder {
        crate::input::describe_connector_input::Builder::default()
    }
    /// Creates a new `DescribeConnector` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeConnector {
    type Output = std::result::Result<
        crate::output::DescribeConnectorOutput,
        crate::error::DescribeConnectorError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_connector_error(response)
        } else {
            crate::operation_deser::parse_describe_connector_response(response)
        }
    }
}

/// Operation shape for `DescribeConnectorEntity`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_connector_entity`](crate::client::Client::describe_connector_entity).
///
/// See [`crate::client::fluent_builders::DescribeConnectorEntity`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeConnectorEntity {
    _private: (),
}
impl DescribeConnectorEntity {
    /// Creates a new builder-style object to manufacture [`DescribeConnectorEntityInput`](crate::input::DescribeConnectorEntityInput).
    pub fn builder() -> crate::input::describe_connector_entity_input::Builder {
        crate::input::describe_connector_entity_input::Builder::default()
    }
    /// Creates a new `DescribeConnectorEntity` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeConnectorEntity {
    type Output = std::result::Result<
        crate::output::DescribeConnectorEntityOutput,
        crate::error::DescribeConnectorEntityError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_connector_entity_error(response)
        } else {
            crate::operation_deser::parse_describe_connector_entity_response(response)
        }
    }
}

/// Operation shape for `DescribeConnectorProfiles`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_connector_profiles`](crate::client::Client::describe_connector_profiles).
///
/// See [`crate::client::fluent_builders::DescribeConnectorProfiles`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeConnectorProfiles {
    _private: (),
}
impl DescribeConnectorProfiles {
    /// Creates a new builder-style object to manufacture [`DescribeConnectorProfilesInput`](crate::input::DescribeConnectorProfilesInput).
    pub fn builder() -> crate::input::describe_connector_profiles_input::Builder {
        crate::input::describe_connector_profiles_input::Builder::default()
    }
    /// Creates a new `DescribeConnectorProfiles` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeConnectorProfiles {
    type Output = std::result::Result<
        crate::output::DescribeConnectorProfilesOutput,
        crate::error::DescribeConnectorProfilesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_connector_profiles_error(response)
        } else {
            crate::operation_deser::parse_describe_connector_profiles_response(response)
        }
    }
}

/// Operation shape for `DescribeConnectors`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_connectors`](crate::client::Client::describe_connectors).
///
/// See [`crate::client::fluent_builders::DescribeConnectors`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeConnectors {
    _private: (),
}
impl DescribeConnectors {
    /// Creates a new builder-style object to manufacture [`DescribeConnectorsInput`](crate::input::DescribeConnectorsInput).
    pub fn builder() -> crate::input::describe_connectors_input::Builder {
        crate::input::describe_connectors_input::Builder::default()
    }
    /// Creates a new `DescribeConnectors` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeConnectors {
    type Output = std::result::Result<
        crate::output::DescribeConnectorsOutput,
        crate::error::DescribeConnectorsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_connectors_error(response)
        } else {
            crate::operation_deser::parse_describe_connectors_response(response)
        }
    }
}

/// Operation shape for `DescribeFlow`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_flow`](crate::client::Client::describe_flow).
///
/// See [`crate::client::fluent_builders::DescribeFlow`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeFlow {
    _private: (),
}
impl DescribeFlow {
    /// Creates a new builder-style object to manufacture [`DescribeFlowInput`](crate::input::DescribeFlowInput).
    pub fn builder() -> crate::input::describe_flow_input::Builder {
        crate::input::describe_flow_input::Builder::default()
    }
    /// Creates a new `DescribeFlow` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeFlow {
    type Output =
        std::result::Result<crate::output::DescribeFlowOutput, crate::error::DescribeFlowError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_flow_error(response)
        } else {
            crate::operation_deser::parse_describe_flow_response(response)
        }
    }
}

/// Operation shape for `DescribeFlowExecutionRecords`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_flow_execution_records`](crate::client::Client::describe_flow_execution_records).
///
/// See [`crate::client::fluent_builders::DescribeFlowExecutionRecords`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeFlowExecutionRecords {
    _private: (),
}
impl DescribeFlowExecutionRecords {
    /// Creates a new builder-style object to manufacture [`DescribeFlowExecutionRecordsInput`](crate::input::DescribeFlowExecutionRecordsInput).
    pub fn builder() -> crate::input::describe_flow_execution_records_input::Builder {
        crate::input::describe_flow_execution_records_input::Builder::default()
    }
    /// Creates a new `DescribeFlowExecutionRecords` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeFlowExecutionRecords {
    type Output = std::result::Result<
        crate::output::DescribeFlowExecutionRecordsOutput,
        crate::error::DescribeFlowExecutionRecordsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_flow_execution_records_error(response)
        } else {
            crate::operation_deser::parse_describe_flow_execution_records_response(response)
        }
    }
}

/// Operation shape for `ListConnectorEntities`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_connector_entities`](crate::client::Client::list_connector_entities).
///
/// See [`crate::client::fluent_builders::ListConnectorEntities`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListConnectorEntities {
    _private: (),
}
impl ListConnectorEntities {
    /// Creates a new builder-style object to manufacture [`ListConnectorEntitiesInput`](crate::input::ListConnectorEntitiesInput).
    pub fn builder() -> crate::input::list_connector_entities_input::Builder {
        crate::input::list_connector_entities_input::Builder::default()
    }
    /// Creates a new `ListConnectorEntities` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListConnectorEntities {
    type Output = std::result::Result<
        crate::output::ListConnectorEntitiesOutput,
        crate::error::ListConnectorEntitiesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_connector_entities_error(response)
        } else {
            crate::operation_deser::parse_list_connector_entities_response(response)
        }
    }
}

/// Operation shape for `ListConnectors`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_connectors`](crate::client::Client::list_connectors).
///
/// See [`crate::client::fluent_builders::ListConnectors`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListConnectors {
    _private: (),
}
impl ListConnectors {
    /// Creates a new builder-style object to manufacture [`ListConnectorsInput`](crate::input::ListConnectorsInput).
    pub fn builder() -> crate::input::list_connectors_input::Builder {
        crate::input::list_connectors_input::Builder::default()
    }
    /// Creates a new `ListConnectors` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListConnectors {
    type Output =
        std::result::Result<crate::output::ListConnectorsOutput, crate::error::ListConnectorsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_connectors_error(response)
        } else {
            crate::operation_deser::parse_list_connectors_response(response)
        }
    }
}

/// Operation shape for `ListFlows`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_flows`](crate::client::Client::list_flows).
///
/// See [`crate::client::fluent_builders::ListFlows`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListFlows {
    _private: (),
}
impl ListFlows {
    /// Creates a new builder-style object to manufacture [`ListFlowsInput`](crate::input::ListFlowsInput).
    pub fn builder() -> crate::input::list_flows_input::Builder {
        crate::input::list_flows_input::Builder::default()
    }
    /// Creates a new `ListFlows` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListFlows {
    type Output = std::result::Result<crate::output::ListFlowsOutput, crate::error::ListFlowsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_flows_error(response)
        } else {
            crate::operation_deser::parse_list_flows_response(response)
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

/// Operation shape for `RegisterConnector`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`register_connector`](crate::client::Client::register_connector).
///
/// See [`crate::client::fluent_builders::RegisterConnector`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct RegisterConnector {
    _private: (),
}
impl RegisterConnector {
    /// Creates a new builder-style object to manufacture [`RegisterConnectorInput`](crate::input::RegisterConnectorInput).
    pub fn builder() -> crate::input::register_connector_input::Builder {
        crate::input::register_connector_input::Builder::default()
    }
    /// Creates a new `RegisterConnector` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for RegisterConnector {
    type Output = std::result::Result<
        crate::output::RegisterConnectorOutput,
        crate::error::RegisterConnectorError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_register_connector_error(response)
        } else {
            crate::operation_deser::parse_register_connector_response(response)
        }
    }
}

/// Operation shape for `StartFlow`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`start_flow`](crate::client::Client::start_flow).
///
/// See [`crate::client::fluent_builders::StartFlow`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StartFlow {
    _private: (),
}
impl StartFlow {
    /// Creates a new builder-style object to manufacture [`StartFlowInput`](crate::input::StartFlowInput).
    pub fn builder() -> crate::input::start_flow_input::Builder {
        crate::input::start_flow_input::Builder::default()
    }
    /// Creates a new `StartFlow` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StartFlow {
    type Output = std::result::Result<crate::output::StartFlowOutput, crate::error::StartFlowError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_start_flow_error(response)
        } else {
            crate::operation_deser::parse_start_flow_response(response)
        }
    }
}

/// Operation shape for `StopFlow`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`stop_flow`](crate::client::Client::stop_flow).
///
/// See [`crate::client::fluent_builders::StopFlow`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StopFlow {
    _private: (),
}
impl StopFlow {
    /// Creates a new builder-style object to manufacture [`StopFlowInput`](crate::input::StopFlowInput).
    pub fn builder() -> crate::input::stop_flow_input::Builder {
        crate::input::stop_flow_input::Builder::default()
    }
    /// Creates a new `StopFlow` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StopFlow {
    type Output = std::result::Result<crate::output::StopFlowOutput, crate::error::StopFlowError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_stop_flow_error(response)
        } else {
            crate::operation_deser::parse_stop_flow_response(response)
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
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_tag_resource_error(response)
        } else {
            crate::operation_deser::parse_tag_resource_response(response)
        }
    }
}

/// Operation shape for `UnregisterConnector`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`unregister_connector`](crate::client::Client::unregister_connector).
///
/// See [`crate::client::fluent_builders::UnregisterConnector`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UnregisterConnector {
    _private: (),
}
impl UnregisterConnector {
    /// Creates a new builder-style object to manufacture [`UnregisterConnectorInput`](crate::input::UnregisterConnectorInput).
    pub fn builder() -> crate::input::unregister_connector_input::Builder {
        crate::input::unregister_connector_input::Builder::default()
    }
    /// Creates a new `UnregisterConnector` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UnregisterConnector {
    type Output = std::result::Result<
        crate::output::UnregisterConnectorOutput,
        crate::error::UnregisterConnectorError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_unregister_connector_error(response)
        } else {
            crate::operation_deser::parse_unregister_connector_response(response)
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
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_untag_resource_error(response)
        } else {
            crate::operation_deser::parse_untag_resource_response(response)
        }
    }
}

/// Operation shape for `UpdateConnectorProfile`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_connector_profile`](crate::client::Client::update_connector_profile).
///
/// See [`crate::client::fluent_builders::UpdateConnectorProfile`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateConnectorProfile {
    _private: (),
}
impl UpdateConnectorProfile {
    /// Creates a new builder-style object to manufacture [`UpdateConnectorProfileInput`](crate::input::UpdateConnectorProfileInput).
    pub fn builder() -> crate::input::update_connector_profile_input::Builder {
        crate::input::update_connector_profile_input::Builder::default()
    }
    /// Creates a new `UpdateConnectorProfile` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateConnectorProfile {
    type Output = std::result::Result<
        crate::output::UpdateConnectorProfileOutput,
        crate::error::UpdateConnectorProfileError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_connector_profile_error(response)
        } else {
            crate::operation_deser::parse_update_connector_profile_response(response)
        }
    }
}

/// Operation shape for `UpdateFlow`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_flow`](crate::client::Client::update_flow).
///
/// See [`crate::client::fluent_builders::UpdateFlow`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateFlow {
    _private: (),
}
impl UpdateFlow {
    /// Creates a new builder-style object to manufacture [`UpdateFlowInput`](crate::input::UpdateFlowInput).
    pub fn builder() -> crate::input::update_flow_input::Builder {
        crate::input::update_flow_input::Builder::default()
    }
    /// Creates a new `UpdateFlow` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateFlow {
    type Output =
        std::result::Result<crate::output::UpdateFlowOutput, crate::error::UpdateFlowError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_flow_error(response)
        } else {
            crate::operation_deser::parse_update_flow_response(response)
        }
    }
}

/// Operation customization and supporting types
pub mod customize;
