// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CreateFHIRDatastore`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_fhir_datastore`](crate::client::Client::create_fhir_datastore).
///
/// See [`crate::client::fluent_builders::CreateFHIRDatastore`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateFHIRDatastore {
    _private: (),
}
impl CreateFHIRDatastore {
    /// Creates a new builder-style object to manufacture [`CreateFhirDatastoreInput`](crate::input::CreateFhirDatastoreInput).
    pub fn builder() -> crate::input::create_fhir_datastore_input::Builder {
        crate::input::create_fhir_datastore_input::Builder::default()
    }
    /// Creates a new `CreateFHIRDatastore` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateFHIRDatastore {
    type Output = std::result::Result<
        crate::output::CreateFhirDatastoreOutput,
        crate::error::CreateFHIRDatastoreError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_fhir_datastore_error(response)
        } else {
            crate::operation_deser::parse_create_fhir_datastore_response(response)
        }
    }
}

/// Operation shape for `DeleteFHIRDatastore`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_fhir_datastore`](crate::client::Client::delete_fhir_datastore).
///
/// See [`crate::client::fluent_builders::DeleteFHIRDatastore`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteFHIRDatastore {
    _private: (),
}
impl DeleteFHIRDatastore {
    /// Creates a new builder-style object to manufacture [`DeleteFhirDatastoreInput`](crate::input::DeleteFhirDatastoreInput).
    pub fn builder() -> crate::input::delete_fhir_datastore_input::Builder {
        crate::input::delete_fhir_datastore_input::Builder::default()
    }
    /// Creates a new `DeleteFHIRDatastore` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteFHIRDatastore {
    type Output = std::result::Result<
        crate::output::DeleteFhirDatastoreOutput,
        crate::error::DeleteFHIRDatastoreError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_fhir_datastore_error(response)
        } else {
            crate::operation_deser::parse_delete_fhir_datastore_response(response)
        }
    }
}

/// Operation shape for `DescribeFHIRDatastore`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_fhir_datastore`](crate::client::Client::describe_fhir_datastore).
///
/// See [`crate::client::fluent_builders::DescribeFHIRDatastore`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeFHIRDatastore {
    _private: (),
}
impl DescribeFHIRDatastore {
    /// Creates a new builder-style object to manufacture [`DescribeFhirDatastoreInput`](crate::input::DescribeFhirDatastoreInput).
    pub fn builder() -> crate::input::describe_fhir_datastore_input::Builder {
        crate::input::describe_fhir_datastore_input::Builder::default()
    }
    /// Creates a new `DescribeFHIRDatastore` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeFHIRDatastore {
    type Output = std::result::Result<
        crate::output::DescribeFhirDatastoreOutput,
        crate::error::DescribeFHIRDatastoreError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_fhir_datastore_error(response)
        } else {
            crate::operation_deser::parse_describe_fhir_datastore_response(response)
        }
    }
}

/// Operation shape for `DescribeFHIRExportJob`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_fhir_export_job`](crate::client::Client::describe_fhir_export_job).
///
/// See [`crate::client::fluent_builders::DescribeFHIRExportJob`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeFHIRExportJob {
    _private: (),
}
impl DescribeFHIRExportJob {
    /// Creates a new builder-style object to manufacture [`DescribeFhirExportJobInput`](crate::input::DescribeFhirExportJobInput).
    pub fn builder() -> crate::input::describe_fhir_export_job_input::Builder {
        crate::input::describe_fhir_export_job_input::Builder::default()
    }
    /// Creates a new `DescribeFHIRExportJob` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeFHIRExportJob {
    type Output = std::result::Result<
        crate::output::DescribeFhirExportJobOutput,
        crate::error::DescribeFHIRExportJobError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_fhir_export_job_error(response)
        } else {
            crate::operation_deser::parse_describe_fhir_export_job_response(response)
        }
    }
}

/// Operation shape for `DescribeFHIRImportJob`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_fhir_import_job`](crate::client::Client::describe_fhir_import_job).
///
/// See [`crate::client::fluent_builders::DescribeFHIRImportJob`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeFHIRImportJob {
    _private: (),
}
impl DescribeFHIRImportJob {
    /// Creates a new builder-style object to manufacture [`DescribeFhirImportJobInput`](crate::input::DescribeFhirImportJobInput).
    pub fn builder() -> crate::input::describe_fhir_import_job_input::Builder {
        crate::input::describe_fhir_import_job_input::Builder::default()
    }
    /// Creates a new `DescribeFHIRImportJob` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeFHIRImportJob {
    type Output = std::result::Result<
        crate::output::DescribeFhirImportJobOutput,
        crate::error::DescribeFHIRImportJobError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_fhir_import_job_error(response)
        } else {
            crate::operation_deser::parse_describe_fhir_import_job_response(response)
        }
    }
}

/// Operation shape for `ListFHIRDatastores`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_fhir_datastores`](crate::client::Client::list_fhir_datastores).
///
/// See [`crate::client::fluent_builders::ListFHIRDatastores`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListFHIRDatastores {
    _private: (),
}
impl ListFHIRDatastores {
    /// Creates a new builder-style object to manufacture [`ListFhirDatastoresInput`](crate::input::ListFhirDatastoresInput).
    pub fn builder() -> crate::input::list_fhir_datastores_input::Builder {
        crate::input::list_fhir_datastores_input::Builder::default()
    }
    /// Creates a new `ListFHIRDatastores` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListFHIRDatastores {
    type Output = std::result::Result<
        crate::output::ListFhirDatastoresOutput,
        crate::error::ListFHIRDatastoresError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_fhir_datastores_error(response)
        } else {
            crate::operation_deser::parse_list_fhir_datastores_response(response)
        }
    }
}

/// Operation shape for `ListFHIRExportJobs`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_fhir_export_jobs`](crate::client::Client::list_fhir_export_jobs).
///
/// See [`crate::client::fluent_builders::ListFHIRExportJobs`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListFHIRExportJobs {
    _private: (),
}
impl ListFHIRExportJobs {
    /// Creates a new builder-style object to manufacture [`ListFhirExportJobsInput`](crate::input::ListFhirExportJobsInput).
    pub fn builder() -> crate::input::list_fhir_export_jobs_input::Builder {
        crate::input::list_fhir_export_jobs_input::Builder::default()
    }
    /// Creates a new `ListFHIRExportJobs` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListFHIRExportJobs {
    type Output = std::result::Result<
        crate::output::ListFhirExportJobsOutput,
        crate::error::ListFHIRExportJobsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_fhir_export_jobs_error(response)
        } else {
            crate::operation_deser::parse_list_fhir_export_jobs_response(response)
        }
    }
}

/// Operation shape for `ListFHIRImportJobs`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_fhir_import_jobs`](crate::client::Client::list_fhir_import_jobs).
///
/// See [`crate::client::fluent_builders::ListFHIRImportJobs`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListFHIRImportJobs {
    _private: (),
}
impl ListFHIRImportJobs {
    /// Creates a new builder-style object to manufacture [`ListFhirImportJobsInput`](crate::input::ListFhirImportJobsInput).
    pub fn builder() -> crate::input::list_fhir_import_jobs_input::Builder {
        crate::input::list_fhir_import_jobs_input::Builder::default()
    }
    /// Creates a new `ListFHIRImportJobs` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListFHIRImportJobs {
    type Output = std::result::Result<
        crate::output::ListFhirImportJobsOutput,
        crate::error::ListFHIRImportJobsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_fhir_import_jobs_error(response)
        } else {
            crate::operation_deser::parse_list_fhir_import_jobs_response(response)
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

/// Operation shape for `StartFHIRExportJob`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`start_fhir_export_job`](crate::client::Client::start_fhir_export_job).
///
/// See [`crate::client::fluent_builders::StartFHIRExportJob`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StartFHIRExportJob {
    _private: (),
}
impl StartFHIRExportJob {
    /// Creates a new builder-style object to manufacture [`StartFhirExportJobInput`](crate::input::StartFhirExportJobInput).
    pub fn builder() -> crate::input::start_fhir_export_job_input::Builder {
        crate::input::start_fhir_export_job_input::Builder::default()
    }
    /// Creates a new `StartFHIRExportJob` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StartFHIRExportJob {
    type Output = std::result::Result<
        crate::output::StartFhirExportJobOutput,
        crate::error::StartFHIRExportJobError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_start_fhir_export_job_error(response)
        } else {
            crate::operation_deser::parse_start_fhir_export_job_response(response)
        }
    }
}

/// Operation shape for `StartFHIRImportJob`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`start_fhir_import_job`](crate::client::Client::start_fhir_import_job).
///
/// See [`crate::client::fluent_builders::StartFHIRImportJob`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StartFHIRImportJob {
    _private: (),
}
impl StartFHIRImportJob {
    /// Creates a new builder-style object to manufacture [`StartFhirImportJobInput`](crate::input::StartFhirImportJobInput).
    pub fn builder() -> crate::input::start_fhir_import_job_input::Builder {
        crate::input::start_fhir_import_job_input::Builder::default()
    }
    /// Creates a new `StartFHIRImportJob` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StartFHIRImportJob {
    type Output = std::result::Result<
        crate::output::StartFhirImportJobOutput,
        crate::error::StartFHIRImportJobError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_start_fhir_import_job_error(response)
        } else {
            crate::operation_deser::parse_start_fhir_import_job_response(response)
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

/// Operation customization and supporting types
pub mod customize;
