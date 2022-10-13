// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CreateKeyspace`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_keyspace`](crate::client::Client::create_keyspace).
///
/// See [`crate::client::fluent_builders::CreateKeyspace`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateKeyspace {
    _private: (),
}
impl CreateKeyspace {
    /// Creates a new builder-style object to manufacture [`CreateKeyspaceInput`](crate::input::CreateKeyspaceInput).
    pub fn builder() -> crate::input::create_keyspace_input::Builder {
        crate::input::create_keyspace_input::Builder::default()
    }
    /// Creates a new `CreateKeyspace` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateKeyspace {
    type Output =
        std::result::Result<crate::output::CreateKeyspaceOutput, crate::error::CreateKeyspaceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_keyspace_error(response)
        } else {
            crate::operation_deser::parse_create_keyspace_response(response)
        }
    }
}

/// Operation shape for `CreateTable`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_table`](crate::client::Client::create_table).
///
/// See [`crate::client::fluent_builders::CreateTable`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateTable {
    _private: (),
}
impl CreateTable {
    /// Creates a new builder-style object to manufacture [`CreateTableInput`](crate::input::CreateTableInput).
    pub fn builder() -> crate::input::create_table_input::Builder {
        crate::input::create_table_input::Builder::default()
    }
    /// Creates a new `CreateTable` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateTable {
    type Output =
        std::result::Result<crate::output::CreateTableOutput, crate::error::CreateTableError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_table_error(response)
        } else {
            crate::operation_deser::parse_create_table_response(response)
        }
    }
}

/// Operation shape for `DeleteKeyspace`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_keyspace`](crate::client::Client::delete_keyspace).
///
/// See [`crate::client::fluent_builders::DeleteKeyspace`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteKeyspace {
    _private: (),
}
impl DeleteKeyspace {
    /// Creates a new builder-style object to manufacture [`DeleteKeyspaceInput`](crate::input::DeleteKeyspaceInput).
    pub fn builder() -> crate::input::delete_keyspace_input::Builder {
        crate::input::delete_keyspace_input::Builder::default()
    }
    /// Creates a new `DeleteKeyspace` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteKeyspace {
    type Output =
        std::result::Result<crate::output::DeleteKeyspaceOutput, crate::error::DeleteKeyspaceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_keyspace_error(response)
        } else {
            crate::operation_deser::parse_delete_keyspace_response(response)
        }
    }
}

/// Operation shape for `DeleteTable`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_table`](crate::client::Client::delete_table).
///
/// See [`crate::client::fluent_builders::DeleteTable`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteTable {
    _private: (),
}
impl DeleteTable {
    /// Creates a new builder-style object to manufacture [`DeleteTableInput`](crate::input::DeleteTableInput).
    pub fn builder() -> crate::input::delete_table_input::Builder {
        crate::input::delete_table_input::Builder::default()
    }
    /// Creates a new `DeleteTable` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteTable {
    type Output =
        std::result::Result<crate::output::DeleteTableOutput, crate::error::DeleteTableError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_table_error(response)
        } else {
            crate::operation_deser::parse_delete_table_response(response)
        }
    }
}

/// Operation shape for `GetKeyspace`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_keyspace`](crate::client::Client::get_keyspace).
///
/// See [`crate::client::fluent_builders::GetKeyspace`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetKeyspace {
    _private: (),
}
impl GetKeyspace {
    /// Creates a new builder-style object to manufacture [`GetKeyspaceInput`](crate::input::GetKeyspaceInput).
    pub fn builder() -> crate::input::get_keyspace_input::Builder {
        crate::input::get_keyspace_input::Builder::default()
    }
    /// Creates a new `GetKeyspace` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetKeyspace {
    type Output =
        std::result::Result<crate::output::GetKeyspaceOutput, crate::error::GetKeyspaceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_keyspace_error(response)
        } else {
            crate::operation_deser::parse_get_keyspace_response(response)
        }
    }
}

/// Operation shape for `GetTable`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_table`](crate::client::Client::get_table).
///
/// See [`crate::client::fluent_builders::GetTable`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetTable {
    _private: (),
}
impl GetTable {
    /// Creates a new builder-style object to manufacture [`GetTableInput`](crate::input::GetTableInput).
    pub fn builder() -> crate::input::get_table_input::Builder {
        crate::input::get_table_input::Builder::default()
    }
    /// Creates a new `GetTable` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetTable {
    type Output = std::result::Result<crate::output::GetTableOutput, crate::error::GetTableError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_table_error(response)
        } else {
            crate::operation_deser::parse_get_table_response(response)
        }
    }
}

/// Operation shape for `ListKeyspaces`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_keyspaces`](crate::client::Client::list_keyspaces).
///
/// See [`crate::client::fluent_builders::ListKeyspaces`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListKeyspaces {
    _private: (),
}
impl ListKeyspaces {
    /// Creates a new builder-style object to manufacture [`ListKeyspacesInput`](crate::input::ListKeyspacesInput).
    pub fn builder() -> crate::input::list_keyspaces_input::Builder {
        crate::input::list_keyspaces_input::Builder::default()
    }
    /// Creates a new `ListKeyspaces` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListKeyspaces {
    type Output =
        std::result::Result<crate::output::ListKeyspacesOutput, crate::error::ListKeyspacesError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_keyspaces_error(response)
        } else {
            crate::operation_deser::parse_list_keyspaces_response(response)
        }
    }
}

/// Operation shape for `ListTables`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_tables`](crate::client::Client::list_tables).
///
/// See [`crate::client::fluent_builders::ListTables`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListTables {
    _private: (),
}
impl ListTables {
    /// Creates a new builder-style object to manufacture [`ListTablesInput`](crate::input::ListTablesInput).
    pub fn builder() -> crate::input::list_tables_input::Builder {
        crate::input::list_tables_input::Builder::default()
    }
    /// Creates a new `ListTables` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListTables {
    type Output =
        std::result::Result<crate::output::ListTablesOutput, crate::error::ListTablesError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_tables_error(response)
        } else {
            crate::operation_deser::parse_list_tables_response(response)
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

/// Operation shape for `RestoreTable`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`restore_table`](crate::client::Client::restore_table).
///
/// See [`crate::client::fluent_builders::RestoreTable`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct RestoreTable {
    _private: (),
}
impl RestoreTable {
    /// Creates a new builder-style object to manufacture [`RestoreTableInput`](crate::input::RestoreTableInput).
    pub fn builder() -> crate::input::restore_table_input::Builder {
        crate::input::restore_table_input::Builder::default()
    }
    /// Creates a new `RestoreTable` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for RestoreTable {
    type Output =
        std::result::Result<crate::output::RestoreTableOutput, crate::error::RestoreTableError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_restore_table_error(response)
        } else {
            crate::operation_deser::parse_restore_table_response(response)
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

/// Operation shape for `UpdateTable`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_table`](crate::client::Client::update_table).
///
/// See [`crate::client::fluent_builders::UpdateTable`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateTable {
    _private: (),
}
impl UpdateTable {
    /// Creates a new builder-style object to manufacture [`UpdateTableInput`](crate::input::UpdateTableInput).
    pub fn builder() -> crate::input::update_table_input::Builder {
        crate::input::update_table_input::Builder::default()
    }
    /// Creates a new `UpdateTable` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateTable {
    type Output =
        std::result::Result<crate::output::UpdateTableOutput, crate::error::UpdateTableError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_table_error(response)
        } else {
            crate::operation_deser::parse_update_table_response(response)
        }
    }
}

/// Operation customization and supporting types
pub mod customize;
