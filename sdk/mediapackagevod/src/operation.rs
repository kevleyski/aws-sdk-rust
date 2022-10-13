// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `ConfigureLogs`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`configure_logs`](crate::client::Client::configure_logs).
///
/// See [`crate::client::fluent_builders::ConfigureLogs`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ConfigureLogs {
    _private: (),
}
impl ConfigureLogs {
    /// Creates a new builder-style object to manufacture [`ConfigureLogsInput`](crate::input::ConfigureLogsInput).
    pub fn builder() -> crate::input::configure_logs_input::Builder {
        crate::input::configure_logs_input::Builder::default()
    }
    /// Creates a new `ConfigureLogs` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ConfigureLogs {
    type Output =
        std::result::Result<crate::output::ConfigureLogsOutput, crate::error::ConfigureLogsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_configure_logs_error(response)
        } else {
            crate::operation_deser::parse_configure_logs_response(response)
        }
    }
}

/// Operation shape for `CreateAsset`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_asset`](crate::client::Client::create_asset).
///
/// See [`crate::client::fluent_builders::CreateAsset`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateAsset {
    _private: (),
}
impl CreateAsset {
    /// Creates a new builder-style object to manufacture [`CreateAssetInput`](crate::input::CreateAssetInput).
    pub fn builder() -> crate::input::create_asset_input::Builder {
        crate::input::create_asset_input::Builder::default()
    }
    /// Creates a new `CreateAsset` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateAsset {
    type Output =
        std::result::Result<crate::output::CreateAssetOutput, crate::error::CreateAssetError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_asset_error(response)
        } else {
            crate::operation_deser::parse_create_asset_response(response)
        }
    }
}

/// Operation shape for `CreatePackagingConfiguration`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_packaging_configuration`](crate::client::Client::create_packaging_configuration).
///
/// See [`crate::client::fluent_builders::CreatePackagingConfiguration`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreatePackagingConfiguration {
    _private: (),
}
impl CreatePackagingConfiguration {
    /// Creates a new builder-style object to manufacture [`CreatePackagingConfigurationInput`](crate::input::CreatePackagingConfigurationInput).
    pub fn builder() -> crate::input::create_packaging_configuration_input::Builder {
        crate::input::create_packaging_configuration_input::Builder::default()
    }
    /// Creates a new `CreatePackagingConfiguration` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreatePackagingConfiguration {
    type Output = std::result::Result<
        crate::output::CreatePackagingConfigurationOutput,
        crate::error::CreatePackagingConfigurationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_packaging_configuration_error(response)
        } else {
            crate::operation_deser::parse_create_packaging_configuration_response(response)
        }
    }
}

/// Operation shape for `CreatePackagingGroup`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_packaging_group`](crate::client::Client::create_packaging_group).
///
/// See [`crate::client::fluent_builders::CreatePackagingGroup`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreatePackagingGroup {
    _private: (),
}
impl CreatePackagingGroup {
    /// Creates a new builder-style object to manufacture [`CreatePackagingGroupInput`](crate::input::CreatePackagingGroupInput).
    pub fn builder() -> crate::input::create_packaging_group_input::Builder {
        crate::input::create_packaging_group_input::Builder::default()
    }
    /// Creates a new `CreatePackagingGroup` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreatePackagingGroup {
    type Output = std::result::Result<
        crate::output::CreatePackagingGroupOutput,
        crate::error::CreatePackagingGroupError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_packaging_group_error(response)
        } else {
            crate::operation_deser::parse_create_packaging_group_response(response)
        }
    }
}

/// Operation shape for `DeleteAsset`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_asset`](crate::client::Client::delete_asset).
///
/// See [`crate::client::fluent_builders::DeleteAsset`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteAsset {
    _private: (),
}
impl DeleteAsset {
    /// Creates a new builder-style object to manufacture [`DeleteAssetInput`](crate::input::DeleteAssetInput).
    pub fn builder() -> crate::input::delete_asset_input::Builder {
        crate::input::delete_asset_input::Builder::default()
    }
    /// Creates a new `DeleteAsset` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteAsset {
    type Output =
        std::result::Result<crate::output::DeleteAssetOutput, crate::error::DeleteAssetError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 202 {
            crate::operation_deser::parse_delete_asset_error(response)
        } else {
            crate::operation_deser::parse_delete_asset_response(response)
        }
    }
}

/// Operation shape for `DeletePackagingConfiguration`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_packaging_configuration`](crate::client::Client::delete_packaging_configuration).
///
/// See [`crate::client::fluent_builders::DeletePackagingConfiguration`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeletePackagingConfiguration {
    _private: (),
}
impl DeletePackagingConfiguration {
    /// Creates a new builder-style object to manufacture [`DeletePackagingConfigurationInput`](crate::input::DeletePackagingConfigurationInput).
    pub fn builder() -> crate::input::delete_packaging_configuration_input::Builder {
        crate::input::delete_packaging_configuration_input::Builder::default()
    }
    /// Creates a new `DeletePackagingConfiguration` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeletePackagingConfiguration {
    type Output = std::result::Result<
        crate::output::DeletePackagingConfigurationOutput,
        crate::error::DeletePackagingConfigurationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 202 {
            crate::operation_deser::parse_delete_packaging_configuration_error(response)
        } else {
            crate::operation_deser::parse_delete_packaging_configuration_response(response)
        }
    }
}

/// Operation shape for `DeletePackagingGroup`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_packaging_group`](crate::client::Client::delete_packaging_group).
///
/// See [`crate::client::fluent_builders::DeletePackagingGroup`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeletePackagingGroup {
    _private: (),
}
impl DeletePackagingGroup {
    /// Creates a new builder-style object to manufacture [`DeletePackagingGroupInput`](crate::input::DeletePackagingGroupInput).
    pub fn builder() -> crate::input::delete_packaging_group_input::Builder {
        crate::input::delete_packaging_group_input::Builder::default()
    }
    /// Creates a new `DeletePackagingGroup` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeletePackagingGroup {
    type Output = std::result::Result<
        crate::output::DeletePackagingGroupOutput,
        crate::error::DeletePackagingGroupError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 202 {
            crate::operation_deser::parse_delete_packaging_group_error(response)
        } else {
            crate::operation_deser::parse_delete_packaging_group_response(response)
        }
    }
}

/// Operation shape for `DescribeAsset`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_asset`](crate::client::Client::describe_asset).
///
/// See [`crate::client::fluent_builders::DescribeAsset`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeAsset {
    _private: (),
}
impl DescribeAsset {
    /// Creates a new builder-style object to manufacture [`DescribeAssetInput`](crate::input::DescribeAssetInput).
    pub fn builder() -> crate::input::describe_asset_input::Builder {
        crate::input::describe_asset_input::Builder::default()
    }
    /// Creates a new `DescribeAsset` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeAsset {
    type Output =
        std::result::Result<crate::output::DescribeAssetOutput, crate::error::DescribeAssetError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_asset_error(response)
        } else {
            crate::operation_deser::parse_describe_asset_response(response)
        }
    }
}

/// Operation shape for `DescribePackagingConfiguration`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_packaging_configuration`](crate::client::Client::describe_packaging_configuration).
///
/// See [`crate::client::fluent_builders::DescribePackagingConfiguration`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribePackagingConfiguration {
    _private: (),
}
impl DescribePackagingConfiguration {
    /// Creates a new builder-style object to manufacture [`DescribePackagingConfigurationInput`](crate::input::DescribePackagingConfigurationInput).
    pub fn builder() -> crate::input::describe_packaging_configuration_input::Builder {
        crate::input::describe_packaging_configuration_input::Builder::default()
    }
    /// Creates a new `DescribePackagingConfiguration` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribePackagingConfiguration {
    type Output = std::result::Result<
        crate::output::DescribePackagingConfigurationOutput,
        crate::error::DescribePackagingConfigurationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_packaging_configuration_error(response)
        } else {
            crate::operation_deser::parse_describe_packaging_configuration_response(response)
        }
    }
}

/// Operation shape for `DescribePackagingGroup`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_packaging_group`](crate::client::Client::describe_packaging_group).
///
/// See [`crate::client::fluent_builders::DescribePackagingGroup`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribePackagingGroup {
    _private: (),
}
impl DescribePackagingGroup {
    /// Creates a new builder-style object to manufacture [`DescribePackagingGroupInput`](crate::input::DescribePackagingGroupInput).
    pub fn builder() -> crate::input::describe_packaging_group_input::Builder {
        crate::input::describe_packaging_group_input::Builder::default()
    }
    /// Creates a new `DescribePackagingGroup` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribePackagingGroup {
    type Output = std::result::Result<
        crate::output::DescribePackagingGroupOutput,
        crate::error::DescribePackagingGroupError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_packaging_group_error(response)
        } else {
            crate::operation_deser::parse_describe_packaging_group_response(response)
        }
    }
}

/// Operation shape for `ListAssets`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_assets`](crate::client::Client::list_assets).
///
/// See [`crate::client::fluent_builders::ListAssets`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListAssets {
    _private: (),
}
impl ListAssets {
    /// Creates a new builder-style object to manufacture [`ListAssetsInput`](crate::input::ListAssetsInput).
    pub fn builder() -> crate::input::list_assets_input::Builder {
        crate::input::list_assets_input::Builder::default()
    }
    /// Creates a new `ListAssets` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListAssets {
    type Output =
        std::result::Result<crate::output::ListAssetsOutput, crate::error::ListAssetsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_assets_error(response)
        } else {
            crate::operation_deser::parse_list_assets_response(response)
        }
    }
}

/// Operation shape for `ListPackagingConfigurations`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_packaging_configurations`](crate::client::Client::list_packaging_configurations).
///
/// See [`crate::client::fluent_builders::ListPackagingConfigurations`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListPackagingConfigurations {
    _private: (),
}
impl ListPackagingConfigurations {
    /// Creates a new builder-style object to manufacture [`ListPackagingConfigurationsInput`](crate::input::ListPackagingConfigurationsInput).
    pub fn builder() -> crate::input::list_packaging_configurations_input::Builder {
        crate::input::list_packaging_configurations_input::Builder::default()
    }
    /// Creates a new `ListPackagingConfigurations` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListPackagingConfigurations {
    type Output = std::result::Result<
        crate::output::ListPackagingConfigurationsOutput,
        crate::error::ListPackagingConfigurationsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_packaging_configurations_error(response)
        } else {
            crate::operation_deser::parse_list_packaging_configurations_response(response)
        }
    }
}

/// Operation shape for `ListPackagingGroups`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_packaging_groups`](crate::client::Client::list_packaging_groups).
///
/// See [`crate::client::fluent_builders::ListPackagingGroups`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListPackagingGroups {
    _private: (),
}
impl ListPackagingGroups {
    /// Creates a new builder-style object to manufacture [`ListPackagingGroupsInput`](crate::input::ListPackagingGroupsInput).
    pub fn builder() -> crate::input::list_packaging_groups_input::Builder {
        crate::input::list_packaging_groups_input::Builder::default()
    }
    /// Creates a new `ListPackagingGroups` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListPackagingGroups {
    type Output = std::result::Result<
        crate::output::ListPackagingGroupsOutput,
        crate::error::ListPackagingGroupsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_packaging_groups_error(response)
        } else {
            crate::operation_deser::parse_list_packaging_groups_response(response)
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
        if !response.status().is_success() && response.status().as_u16() != 204 {
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

/// Operation shape for `UpdatePackagingGroup`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_packaging_group`](crate::client::Client::update_packaging_group).
///
/// See [`crate::client::fluent_builders::UpdatePackagingGroup`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdatePackagingGroup {
    _private: (),
}
impl UpdatePackagingGroup {
    /// Creates a new builder-style object to manufacture [`UpdatePackagingGroupInput`](crate::input::UpdatePackagingGroupInput).
    pub fn builder() -> crate::input::update_packaging_group_input::Builder {
        crate::input::update_packaging_group_input::Builder::default()
    }
    /// Creates a new `UpdatePackagingGroup` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdatePackagingGroup {
    type Output = std::result::Result<
        crate::output::UpdatePackagingGroupOutput,
        crate::error::UpdatePackagingGroupError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_packaging_group_error(response)
        } else {
            crate::operation_deser::parse_update_packaging_group_response(response)
        }
    }
}

/// Operation customization and supporting types
pub mod customize;
