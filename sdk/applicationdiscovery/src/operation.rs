// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `AssociateConfigurationItemsToApplication`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`associate_configuration_items_to_application`](crate::client::Client::associate_configuration_items_to_application).
///
/// See [`crate::client::fluent_builders::AssociateConfigurationItemsToApplication`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct AssociateConfigurationItemsToApplication {
    _private: (),
}
impl AssociateConfigurationItemsToApplication {
    /// Creates a new builder-style object to manufacture [`AssociateConfigurationItemsToApplicationInput`](crate::input::AssociateConfigurationItemsToApplicationInput).
    pub fn builder() -> crate::input::associate_configuration_items_to_application_input::Builder {
        crate::input::associate_configuration_items_to_application_input::Builder::default()
    }
    /// Creates a new `AssociateConfigurationItemsToApplication` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for AssociateConfigurationItemsToApplication {
    type Output = std::result::Result<
        crate::output::AssociateConfigurationItemsToApplicationOutput,
        crate::error::AssociateConfigurationItemsToApplicationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_associate_configuration_items_to_application_error(
                response,
            )
        } else {
            crate::operation_deser::parse_associate_configuration_items_to_application_response(
                response,
            )
        }
    }
}

/// Operation shape for `BatchDeleteImportData`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`batch_delete_import_data`](crate::client::Client::batch_delete_import_data).
///
/// See [`crate::client::fluent_builders::BatchDeleteImportData`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct BatchDeleteImportData {
    _private: (),
}
impl BatchDeleteImportData {
    /// Creates a new builder-style object to manufacture [`BatchDeleteImportDataInput`](crate::input::BatchDeleteImportDataInput).
    pub fn builder() -> crate::input::batch_delete_import_data_input::Builder {
        crate::input::batch_delete_import_data_input::Builder::default()
    }
    /// Creates a new `BatchDeleteImportData` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for BatchDeleteImportData {
    type Output = std::result::Result<
        crate::output::BatchDeleteImportDataOutput,
        crate::error::BatchDeleteImportDataError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_batch_delete_import_data_error(response)
        } else {
            crate::operation_deser::parse_batch_delete_import_data_response(response)
        }
    }
}

/// Operation shape for `CreateApplication`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_application`](crate::client::Client::create_application).
///
/// See [`crate::client::fluent_builders::CreateApplication`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateApplication {
    _private: (),
}
impl CreateApplication {
    /// Creates a new builder-style object to manufacture [`CreateApplicationInput`](crate::input::CreateApplicationInput).
    pub fn builder() -> crate::input::create_application_input::Builder {
        crate::input::create_application_input::Builder::default()
    }
    /// Creates a new `CreateApplication` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateApplication {
    type Output = std::result::Result<
        crate::output::CreateApplicationOutput,
        crate::error::CreateApplicationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_application_error(response)
        } else {
            crate::operation_deser::parse_create_application_response(response)
        }
    }
}

/// Operation shape for `CreateTags`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_tags`](crate::client::Client::create_tags).
///
/// See [`crate::client::fluent_builders::CreateTags`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateTags {
    _private: (),
}
impl CreateTags {
    /// Creates a new builder-style object to manufacture [`CreateTagsInput`](crate::input::CreateTagsInput).
    pub fn builder() -> crate::input::create_tags_input::Builder {
        crate::input::create_tags_input::Builder::default()
    }
    /// Creates a new `CreateTags` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateTags {
    type Output =
        std::result::Result<crate::output::CreateTagsOutput, crate::error::CreateTagsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_tags_error(response)
        } else {
            crate::operation_deser::parse_create_tags_response(response)
        }
    }
}

/// Operation shape for `DeleteApplications`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_applications`](crate::client::Client::delete_applications).
///
/// See [`crate::client::fluent_builders::DeleteApplications`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteApplications {
    _private: (),
}
impl DeleteApplications {
    /// Creates a new builder-style object to manufacture [`DeleteApplicationsInput`](crate::input::DeleteApplicationsInput).
    pub fn builder() -> crate::input::delete_applications_input::Builder {
        crate::input::delete_applications_input::Builder::default()
    }
    /// Creates a new `DeleteApplications` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteApplications {
    type Output = std::result::Result<
        crate::output::DeleteApplicationsOutput,
        crate::error::DeleteApplicationsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_applications_error(response)
        } else {
            crate::operation_deser::parse_delete_applications_response(response)
        }
    }
}

/// Operation shape for `DeleteTags`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_tags`](crate::client::Client::delete_tags).
///
/// See [`crate::client::fluent_builders::DeleteTags`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteTags {
    _private: (),
}
impl DeleteTags {
    /// Creates a new builder-style object to manufacture [`DeleteTagsInput`](crate::input::DeleteTagsInput).
    pub fn builder() -> crate::input::delete_tags_input::Builder {
        crate::input::delete_tags_input::Builder::default()
    }
    /// Creates a new `DeleteTags` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteTags {
    type Output =
        std::result::Result<crate::output::DeleteTagsOutput, crate::error::DeleteTagsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_tags_error(response)
        } else {
            crate::operation_deser::parse_delete_tags_response(response)
        }
    }
}

/// Operation shape for `DescribeAgents`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_agents`](crate::client::Client::describe_agents).
///
/// See [`crate::client::fluent_builders::DescribeAgents`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeAgents {
    _private: (),
}
impl DescribeAgents {
    /// Creates a new builder-style object to manufacture [`DescribeAgentsInput`](crate::input::DescribeAgentsInput).
    pub fn builder() -> crate::input::describe_agents_input::Builder {
        crate::input::describe_agents_input::Builder::default()
    }
    /// Creates a new `DescribeAgents` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeAgents {
    type Output =
        std::result::Result<crate::output::DescribeAgentsOutput, crate::error::DescribeAgentsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_agents_error(response)
        } else {
            crate::operation_deser::parse_describe_agents_response(response)
        }
    }
}

/// Operation shape for `DescribeConfigurations`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_configurations`](crate::client::Client::describe_configurations).
///
/// See [`crate::client::fluent_builders::DescribeConfigurations`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeConfigurations {
    _private: (),
}
impl DescribeConfigurations {
    /// Creates a new builder-style object to manufacture [`DescribeConfigurationsInput`](crate::input::DescribeConfigurationsInput).
    pub fn builder() -> crate::input::describe_configurations_input::Builder {
        crate::input::describe_configurations_input::Builder::default()
    }
    /// Creates a new `DescribeConfigurations` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeConfigurations {
    type Output = std::result::Result<
        crate::output::DescribeConfigurationsOutput,
        crate::error::DescribeConfigurationsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_configurations_error(response)
        } else {
            crate::operation_deser::parse_describe_configurations_response(response)
        }
    }
}

/// Operation shape for `DescribeContinuousExports`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_continuous_exports`](crate::client::Client::describe_continuous_exports).
///
/// See [`crate::client::fluent_builders::DescribeContinuousExports`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeContinuousExports {
    _private: (),
}
impl DescribeContinuousExports {
    /// Creates a new builder-style object to manufacture [`DescribeContinuousExportsInput`](crate::input::DescribeContinuousExportsInput).
    pub fn builder() -> crate::input::describe_continuous_exports_input::Builder {
        crate::input::describe_continuous_exports_input::Builder::default()
    }
    /// Creates a new `DescribeContinuousExports` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeContinuousExports {
    type Output = std::result::Result<
        crate::output::DescribeContinuousExportsOutput,
        crate::error::DescribeContinuousExportsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_continuous_exports_error(response)
        } else {
            crate::operation_deser::parse_describe_continuous_exports_response(response)
        }
    }
}

/// Operation shape for `DescribeExportConfigurations`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_export_configurations`](crate::client::Client::describe_export_configurations).
///
/// See [`crate::client::fluent_builders::DescribeExportConfigurations`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeExportConfigurations {
    _private: (),
}
impl DescribeExportConfigurations {
    /// Creates a new builder-style object to manufacture [`DescribeExportConfigurationsInput`](crate::input::DescribeExportConfigurationsInput).
    pub fn builder() -> crate::input::describe_export_configurations_input::Builder {
        crate::input::describe_export_configurations_input::Builder::default()
    }
    /// Creates a new `DescribeExportConfigurations` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeExportConfigurations {
    type Output = std::result::Result<
        crate::output::DescribeExportConfigurationsOutput,
        crate::error::DescribeExportConfigurationsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_export_configurations_error(response)
        } else {
            crate::operation_deser::parse_describe_export_configurations_response(response)
        }
    }
}

/// Operation shape for `DescribeExportTasks`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_export_tasks`](crate::client::Client::describe_export_tasks).
///
/// See [`crate::client::fluent_builders::DescribeExportTasks`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeExportTasks {
    _private: (),
}
impl DescribeExportTasks {
    /// Creates a new builder-style object to manufacture [`DescribeExportTasksInput`](crate::input::DescribeExportTasksInput).
    pub fn builder() -> crate::input::describe_export_tasks_input::Builder {
        crate::input::describe_export_tasks_input::Builder::default()
    }
    /// Creates a new `DescribeExportTasks` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeExportTasks {
    type Output = std::result::Result<
        crate::output::DescribeExportTasksOutput,
        crate::error::DescribeExportTasksError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_export_tasks_error(response)
        } else {
            crate::operation_deser::parse_describe_export_tasks_response(response)
        }
    }
}

/// Operation shape for `DescribeImportTasks`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_import_tasks`](crate::client::Client::describe_import_tasks).
///
/// See [`crate::client::fluent_builders::DescribeImportTasks`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeImportTasks {
    _private: (),
}
impl DescribeImportTasks {
    /// Creates a new builder-style object to manufacture [`DescribeImportTasksInput`](crate::input::DescribeImportTasksInput).
    pub fn builder() -> crate::input::describe_import_tasks_input::Builder {
        crate::input::describe_import_tasks_input::Builder::default()
    }
    /// Creates a new `DescribeImportTasks` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeImportTasks {
    type Output = std::result::Result<
        crate::output::DescribeImportTasksOutput,
        crate::error::DescribeImportTasksError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_import_tasks_error(response)
        } else {
            crate::operation_deser::parse_describe_import_tasks_response(response)
        }
    }
}

/// Operation shape for `DescribeTags`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_tags`](crate::client::Client::describe_tags).
///
/// See [`crate::client::fluent_builders::DescribeTags`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeTags {
    _private: (),
}
impl DescribeTags {
    /// Creates a new builder-style object to manufacture [`DescribeTagsInput`](crate::input::DescribeTagsInput).
    pub fn builder() -> crate::input::describe_tags_input::Builder {
        crate::input::describe_tags_input::Builder::default()
    }
    /// Creates a new `DescribeTags` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeTags {
    type Output =
        std::result::Result<crate::output::DescribeTagsOutput, crate::error::DescribeTagsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_tags_error(response)
        } else {
            crate::operation_deser::parse_describe_tags_response(response)
        }
    }
}

/// Operation shape for `DisassociateConfigurationItemsFromApplication`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`disassociate_configuration_items_from_application`](crate::client::Client::disassociate_configuration_items_from_application).
///
/// See [`crate::client::fluent_builders::DisassociateConfigurationItemsFromApplication`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DisassociateConfigurationItemsFromApplication {
    _private: (),
}
impl DisassociateConfigurationItemsFromApplication {
    /// Creates a new builder-style object to manufacture [`DisassociateConfigurationItemsFromApplicationInput`](crate::input::DisassociateConfigurationItemsFromApplicationInput).
    pub fn builder(
    ) -> crate::input::disassociate_configuration_items_from_application_input::Builder {
        crate::input::disassociate_configuration_items_from_application_input::Builder::default()
    }
    /// Creates a new `DisassociateConfigurationItemsFromApplication` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse
    for DisassociateConfigurationItemsFromApplication
{
    type Output = std::result::Result<
        crate::output::DisassociateConfigurationItemsFromApplicationOutput,
        crate::error::DisassociateConfigurationItemsFromApplicationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_disassociate_configuration_items_from_application_error(
                response,
            )
        } else {
            crate::operation_deser::parse_disassociate_configuration_items_from_application_response(
                response,
            )
        }
    }
}

/// Operation shape for `ExportConfigurations`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`export_configurations`](crate::client::Client::export_configurations).
///
/// See [`crate::client::fluent_builders::ExportConfigurations`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ExportConfigurations {
    _private: (),
}
impl ExportConfigurations {
    /// Creates a new builder-style object to manufacture [`ExportConfigurationsInput`](crate::input::ExportConfigurationsInput).
    pub fn builder() -> crate::input::export_configurations_input::Builder {
        crate::input::export_configurations_input::Builder::default()
    }
    /// Creates a new `ExportConfigurations` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ExportConfigurations {
    type Output = std::result::Result<
        crate::output::ExportConfigurationsOutput,
        crate::error::ExportConfigurationsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_export_configurations_error(response)
        } else {
            crate::operation_deser::parse_export_configurations_response(response)
        }
    }
}

/// Operation shape for `GetDiscoverySummary`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_discovery_summary`](crate::client::Client::get_discovery_summary).
///
/// See [`crate::client::fluent_builders::GetDiscoverySummary`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetDiscoverySummary {
    _private: (),
}
impl GetDiscoverySummary {
    /// Creates a new builder-style object to manufacture [`GetDiscoverySummaryInput`](crate::input::GetDiscoverySummaryInput).
    pub fn builder() -> crate::input::get_discovery_summary_input::Builder {
        crate::input::get_discovery_summary_input::Builder::default()
    }
    /// Creates a new `GetDiscoverySummary` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetDiscoverySummary {
    type Output = std::result::Result<
        crate::output::GetDiscoverySummaryOutput,
        crate::error::GetDiscoverySummaryError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_discovery_summary_error(response)
        } else {
            crate::operation_deser::parse_get_discovery_summary_response(response)
        }
    }
}

/// Operation shape for `ListConfigurations`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_configurations`](crate::client::Client::list_configurations).
///
/// See [`crate::client::fluent_builders::ListConfigurations`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListConfigurations {
    _private: (),
}
impl ListConfigurations {
    /// Creates a new builder-style object to manufacture [`ListConfigurationsInput`](crate::input::ListConfigurationsInput).
    pub fn builder() -> crate::input::list_configurations_input::Builder {
        crate::input::list_configurations_input::Builder::default()
    }
    /// Creates a new `ListConfigurations` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListConfigurations {
    type Output = std::result::Result<
        crate::output::ListConfigurationsOutput,
        crate::error::ListConfigurationsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_configurations_error(response)
        } else {
            crate::operation_deser::parse_list_configurations_response(response)
        }
    }
}

/// Operation shape for `ListServerNeighbors`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_server_neighbors`](crate::client::Client::list_server_neighbors).
///
/// See [`crate::client::fluent_builders::ListServerNeighbors`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListServerNeighbors {
    _private: (),
}
impl ListServerNeighbors {
    /// Creates a new builder-style object to manufacture [`ListServerNeighborsInput`](crate::input::ListServerNeighborsInput).
    pub fn builder() -> crate::input::list_server_neighbors_input::Builder {
        crate::input::list_server_neighbors_input::Builder::default()
    }
    /// Creates a new `ListServerNeighbors` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListServerNeighbors {
    type Output = std::result::Result<
        crate::output::ListServerNeighborsOutput,
        crate::error::ListServerNeighborsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_server_neighbors_error(response)
        } else {
            crate::operation_deser::parse_list_server_neighbors_response(response)
        }
    }
}

/// Operation shape for `StartContinuousExport`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`start_continuous_export`](crate::client::Client::start_continuous_export).
///
/// See [`crate::client::fluent_builders::StartContinuousExport`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StartContinuousExport {
    _private: (),
}
impl StartContinuousExport {
    /// Creates a new builder-style object to manufacture [`StartContinuousExportInput`](crate::input::StartContinuousExportInput).
    pub fn builder() -> crate::input::start_continuous_export_input::Builder {
        crate::input::start_continuous_export_input::Builder::default()
    }
    /// Creates a new `StartContinuousExport` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StartContinuousExport {
    type Output = std::result::Result<
        crate::output::StartContinuousExportOutput,
        crate::error::StartContinuousExportError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_start_continuous_export_error(response)
        } else {
            crate::operation_deser::parse_start_continuous_export_response(response)
        }
    }
}

/// Operation shape for `StartDataCollectionByAgentIds`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`start_data_collection_by_agent_ids`](crate::client::Client::start_data_collection_by_agent_ids).
///
/// See [`crate::client::fluent_builders::StartDataCollectionByAgentIds`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StartDataCollectionByAgentIds {
    _private: (),
}
impl StartDataCollectionByAgentIds {
    /// Creates a new builder-style object to manufacture [`StartDataCollectionByAgentIdsInput`](crate::input::StartDataCollectionByAgentIdsInput).
    pub fn builder() -> crate::input::start_data_collection_by_agent_ids_input::Builder {
        crate::input::start_data_collection_by_agent_ids_input::Builder::default()
    }
    /// Creates a new `StartDataCollectionByAgentIds` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StartDataCollectionByAgentIds {
    type Output = std::result::Result<
        crate::output::StartDataCollectionByAgentIdsOutput,
        crate::error::StartDataCollectionByAgentIdsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_start_data_collection_by_agent_ids_error(response)
        } else {
            crate::operation_deser::parse_start_data_collection_by_agent_ids_response(response)
        }
    }
}

/// Operation shape for `StartExportTask`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`start_export_task`](crate::client::Client::start_export_task).
///
/// See [`crate::client::fluent_builders::StartExportTask`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StartExportTask {
    _private: (),
}
impl StartExportTask {
    /// Creates a new builder-style object to manufacture [`StartExportTaskInput`](crate::input::StartExportTaskInput).
    pub fn builder() -> crate::input::start_export_task_input::Builder {
        crate::input::start_export_task_input::Builder::default()
    }
    /// Creates a new `StartExportTask` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StartExportTask {
    type Output = std::result::Result<
        crate::output::StartExportTaskOutput,
        crate::error::StartExportTaskError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_start_export_task_error(response)
        } else {
            crate::operation_deser::parse_start_export_task_response(response)
        }
    }
}

/// Operation shape for `StartImportTask`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`start_import_task`](crate::client::Client::start_import_task).
///
/// See [`crate::client::fluent_builders::StartImportTask`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StartImportTask {
    _private: (),
}
impl StartImportTask {
    /// Creates a new builder-style object to manufacture [`StartImportTaskInput`](crate::input::StartImportTaskInput).
    pub fn builder() -> crate::input::start_import_task_input::Builder {
        crate::input::start_import_task_input::Builder::default()
    }
    /// Creates a new `StartImportTask` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StartImportTask {
    type Output = std::result::Result<
        crate::output::StartImportTaskOutput,
        crate::error::StartImportTaskError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_start_import_task_error(response)
        } else {
            crate::operation_deser::parse_start_import_task_response(response)
        }
    }
}

/// Operation shape for `StopContinuousExport`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`stop_continuous_export`](crate::client::Client::stop_continuous_export).
///
/// See [`crate::client::fluent_builders::StopContinuousExport`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StopContinuousExport {
    _private: (),
}
impl StopContinuousExport {
    /// Creates a new builder-style object to manufacture [`StopContinuousExportInput`](crate::input::StopContinuousExportInput).
    pub fn builder() -> crate::input::stop_continuous_export_input::Builder {
        crate::input::stop_continuous_export_input::Builder::default()
    }
    /// Creates a new `StopContinuousExport` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StopContinuousExport {
    type Output = std::result::Result<
        crate::output::StopContinuousExportOutput,
        crate::error::StopContinuousExportError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_stop_continuous_export_error(response)
        } else {
            crate::operation_deser::parse_stop_continuous_export_response(response)
        }
    }
}

/// Operation shape for `StopDataCollectionByAgentIds`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`stop_data_collection_by_agent_ids`](crate::client::Client::stop_data_collection_by_agent_ids).
///
/// See [`crate::client::fluent_builders::StopDataCollectionByAgentIds`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StopDataCollectionByAgentIds {
    _private: (),
}
impl StopDataCollectionByAgentIds {
    /// Creates a new builder-style object to manufacture [`StopDataCollectionByAgentIdsInput`](crate::input::StopDataCollectionByAgentIdsInput).
    pub fn builder() -> crate::input::stop_data_collection_by_agent_ids_input::Builder {
        crate::input::stop_data_collection_by_agent_ids_input::Builder::default()
    }
    /// Creates a new `StopDataCollectionByAgentIds` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StopDataCollectionByAgentIds {
    type Output = std::result::Result<
        crate::output::StopDataCollectionByAgentIdsOutput,
        crate::error::StopDataCollectionByAgentIdsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_stop_data_collection_by_agent_ids_error(response)
        } else {
            crate::operation_deser::parse_stop_data_collection_by_agent_ids_response(response)
        }
    }
}

/// Operation shape for `UpdateApplication`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_application`](crate::client::Client::update_application).
///
/// See [`crate::client::fluent_builders::UpdateApplication`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateApplication {
    _private: (),
}
impl UpdateApplication {
    /// Creates a new builder-style object to manufacture [`UpdateApplicationInput`](crate::input::UpdateApplicationInput).
    pub fn builder() -> crate::input::update_application_input::Builder {
        crate::input::update_application_input::Builder::default()
    }
    /// Creates a new `UpdateApplication` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateApplication {
    type Output = std::result::Result<
        crate::output::UpdateApplicationOutput,
        crate::error::UpdateApplicationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_application_error(response)
        } else {
            crate::operation_deser::parse_update_application_response(response)
        }
    }
}

/// Operation customization and supporting types
pub mod customize;
