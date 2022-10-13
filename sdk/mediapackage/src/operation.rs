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

/// Operation shape for `CreateChannel`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_channel`](crate::client::Client::create_channel).
///
/// See [`crate::client::fluent_builders::CreateChannel`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateChannel {
    _private: (),
}
impl CreateChannel {
    /// Creates a new builder-style object to manufacture [`CreateChannelInput`](crate::input::CreateChannelInput).
    pub fn builder() -> crate::input::create_channel_input::Builder {
        crate::input::create_channel_input::Builder::default()
    }
    /// Creates a new `CreateChannel` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateChannel {
    type Output =
        std::result::Result<crate::output::CreateChannelOutput, crate::error::CreateChannelError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_channel_error(response)
        } else {
            crate::operation_deser::parse_create_channel_response(response)
        }
    }
}

/// Operation shape for `CreateHarvestJob`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_harvest_job`](crate::client::Client::create_harvest_job).
///
/// See [`crate::client::fluent_builders::CreateHarvestJob`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateHarvestJob {
    _private: (),
}
impl CreateHarvestJob {
    /// Creates a new builder-style object to manufacture [`CreateHarvestJobInput`](crate::input::CreateHarvestJobInput).
    pub fn builder() -> crate::input::create_harvest_job_input::Builder {
        crate::input::create_harvest_job_input::Builder::default()
    }
    /// Creates a new `CreateHarvestJob` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateHarvestJob {
    type Output = std::result::Result<
        crate::output::CreateHarvestJobOutput,
        crate::error::CreateHarvestJobError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_harvest_job_error(response)
        } else {
            crate::operation_deser::parse_create_harvest_job_response(response)
        }
    }
}

/// Operation shape for `CreateOriginEndpoint`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_origin_endpoint`](crate::client::Client::create_origin_endpoint).
///
/// See [`crate::client::fluent_builders::CreateOriginEndpoint`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateOriginEndpoint {
    _private: (),
}
impl CreateOriginEndpoint {
    /// Creates a new builder-style object to manufacture [`CreateOriginEndpointInput`](crate::input::CreateOriginEndpointInput).
    pub fn builder() -> crate::input::create_origin_endpoint_input::Builder {
        crate::input::create_origin_endpoint_input::Builder::default()
    }
    /// Creates a new `CreateOriginEndpoint` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateOriginEndpoint {
    type Output = std::result::Result<
        crate::output::CreateOriginEndpointOutput,
        crate::error::CreateOriginEndpointError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_origin_endpoint_error(response)
        } else {
            crate::operation_deser::parse_create_origin_endpoint_response(response)
        }
    }
}

/// Operation shape for `DeleteChannel`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_channel`](crate::client::Client::delete_channel).
///
/// See [`crate::client::fluent_builders::DeleteChannel`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteChannel {
    _private: (),
}
impl DeleteChannel {
    /// Creates a new builder-style object to manufacture [`DeleteChannelInput`](crate::input::DeleteChannelInput).
    pub fn builder() -> crate::input::delete_channel_input::Builder {
        crate::input::delete_channel_input::Builder::default()
    }
    /// Creates a new `DeleteChannel` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteChannel {
    type Output =
        std::result::Result<crate::output::DeleteChannelOutput, crate::error::DeleteChannelError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 202 {
            crate::operation_deser::parse_delete_channel_error(response)
        } else {
            crate::operation_deser::parse_delete_channel_response(response)
        }
    }
}

/// Operation shape for `DeleteOriginEndpoint`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_origin_endpoint`](crate::client::Client::delete_origin_endpoint).
///
/// See [`crate::client::fluent_builders::DeleteOriginEndpoint`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteOriginEndpoint {
    _private: (),
}
impl DeleteOriginEndpoint {
    /// Creates a new builder-style object to manufacture [`DeleteOriginEndpointInput`](crate::input::DeleteOriginEndpointInput).
    pub fn builder() -> crate::input::delete_origin_endpoint_input::Builder {
        crate::input::delete_origin_endpoint_input::Builder::default()
    }
    /// Creates a new `DeleteOriginEndpoint` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteOriginEndpoint {
    type Output = std::result::Result<
        crate::output::DeleteOriginEndpointOutput,
        crate::error::DeleteOriginEndpointError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 202 {
            crate::operation_deser::parse_delete_origin_endpoint_error(response)
        } else {
            crate::operation_deser::parse_delete_origin_endpoint_response(response)
        }
    }
}

/// Operation shape for `DescribeChannel`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_channel`](crate::client::Client::describe_channel).
///
/// See [`crate::client::fluent_builders::DescribeChannel`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeChannel {
    _private: (),
}
impl DescribeChannel {
    /// Creates a new builder-style object to manufacture [`DescribeChannelInput`](crate::input::DescribeChannelInput).
    pub fn builder() -> crate::input::describe_channel_input::Builder {
        crate::input::describe_channel_input::Builder::default()
    }
    /// Creates a new `DescribeChannel` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeChannel {
    type Output = std::result::Result<
        crate::output::DescribeChannelOutput,
        crate::error::DescribeChannelError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_channel_error(response)
        } else {
            crate::operation_deser::parse_describe_channel_response(response)
        }
    }
}

/// Operation shape for `DescribeHarvestJob`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_harvest_job`](crate::client::Client::describe_harvest_job).
///
/// See [`crate::client::fluent_builders::DescribeHarvestJob`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeHarvestJob {
    _private: (),
}
impl DescribeHarvestJob {
    /// Creates a new builder-style object to manufacture [`DescribeHarvestJobInput`](crate::input::DescribeHarvestJobInput).
    pub fn builder() -> crate::input::describe_harvest_job_input::Builder {
        crate::input::describe_harvest_job_input::Builder::default()
    }
    /// Creates a new `DescribeHarvestJob` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeHarvestJob {
    type Output = std::result::Result<
        crate::output::DescribeHarvestJobOutput,
        crate::error::DescribeHarvestJobError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_harvest_job_error(response)
        } else {
            crate::operation_deser::parse_describe_harvest_job_response(response)
        }
    }
}

/// Operation shape for `DescribeOriginEndpoint`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_origin_endpoint`](crate::client::Client::describe_origin_endpoint).
///
/// See [`crate::client::fluent_builders::DescribeOriginEndpoint`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeOriginEndpoint {
    _private: (),
}
impl DescribeOriginEndpoint {
    /// Creates a new builder-style object to manufacture [`DescribeOriginEndpointInput`](crate::input::DescribeOriginEndpointInput).
    pub fn builder() -> crate::input::describe_origin_endpoint_input::Builder {
        crate::input::describe_origin_endpoint_input::Builder::default()
    }
    /// Creates a new `DescribeOriginEndpoint` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeOriginEndpoint {
    type Output = std::result::Result<
        crate::output::DescribeOriginEndpointOutput,
        crate::error::DescribeOriginEndpointError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_origin_endpoint_error(response)
        } else {
            crate::operation_deser::parse_describe_origin_endpoint_response(response)
        }
    }
}

/// Operation shape for `ListChannels`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_channels`](crate::client::Client::list_channels).
///
/// See [`crate::client::fluent_builders::ListChannels`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListChannels {
    _private: (),
}
impl ListChannels {
    /// Creates a new builder-style object to manufacture [`ListChannelsInput`](crate::input::ListChannelsInput).
    pub fn builder() -> crate::input::list_channels_input::Builder {
        crate::input::list_channels_input::Builder::default()
    }
    /// Creates a new `ListChannels` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListChannels {
    type Output =
        std::result::Result<crate::output::ListChannelsOutput, crate::error::ListChannelsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_channels_error(response)
        } else {
            crate::operation_deser::parse_list_channels_response(response)
        }
    }
}

/// Operation shape for `ListHarvestJobs`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_harvest_jobs`](crate::client::Client::list_harvest_jobs).
///
/// See [`crate::client::fluent_builders::ListHarvestJobs`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListHarvestJobs {
    _private: (),
}
impl ListHarvestJobs {
    /// Creates a new builder-style object to manufacture [`ListHarvestJobsInput`](crate::input::ListHarvestJobsInput).
    pub fn builder() -> crate::input::list_harvest_jobs_input::Builder {
        crate::input::list_harvest_jobs_input::Builder::default()
    }
    /// Creates a new `ListHarvestJobs` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListHarvestJobs {
    type Output = std::result::Result<
        crate::output::ListHarvestJobsOutput,
        crate::error::ListHarvestJobsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_harvest_jobs_error(response)
        } else {
            crate::operation_deser::parse_list_harvest_jobs_response(response)
        }
    }
}

/// Operation shape for `ListOriginEndpoints`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_origin_endpoints`](crate::client::Client::list_origin_endpoints).
///
/// See [`crate::client::fluent_builders::ListOriginEndpoints`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListOriginEndpoints {
    _private: (),
}
impl ListOriginEndpoints {
    /// Creates a new builder-style object to manufacture [`ListOriginEndpointsInput`](crate::input::ListOriginEndpointsInput).
    pub fn builder() -> crate::input::list_origin_endpoints_input::Builder {
        crate::input::list_origin_endpoints_input::Builder::default()
    }
    /// Creates a new `ListOriginEndpoints` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListOriginEndpoints {
    type Output = std::result::Result<
        crate::output::ListOriginEndpointsOutput,
        crate::error::ListOriginEndpointsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_origin_endpoints_error(response)
        } else {
            crate::operation_deser::parse_list_origin_endpoints_response(response)
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

/// Operation shape for `RotateChannelCredentials`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`rotate_channel_credentials`](crate::client::Client::rotate_channel_credentials).
///
/// See [`crate::client::fluent_builders::RotateChannelCredentials`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct RotateChannelCredentials {
    _private: (),
}
impl RotateChannelCredentials {
    /// Creates a new builder-style object to manufacture [`RotateChannelCredentialsInput`](crate::input::RotateChannelCredentialsInput).
    pub fn builder() -> crate::input::rotate_channel_credentials_input::Builder {
        crate::input::rotate_channel_credentials_input::Builder::default()
    }
    /// Creates a new `RotateChannelCredentials` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for RotateChannelCredentials {
    type Output = std::result::Result<
        crate::output::RotateChannelCredentialsOutput,
        crate::error::RotateChannelCredentialsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_rotate_channel_credentials_error(response)
        } else {
            crate::operation_deser::parse_rotate_channel_credentials_response(response)
        }
    }
}

/// Operation shape for `RotateIngestEndpointCredentials`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`rotate_ingest_endpoint_credentials`](crate::client::Client::rotate_ingest_endpoint_credentials).
///
/// See [`crate::client::fluent_builders::RotateIngestEndpointCredentials`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct RotateIngestEndpointCredentials {
    _private: (),
}
impl RotateIngestEndpointCredentials {
    /// Creates a new builder-style object to manufacture [`RotateIngestEndpointCredentialsInput`](crate::input::RotateIngestEndpointCredentialsInput).
    pub fn builder() -> crate::input::rotate_ingest_endpoint_credentials_input::Builder {
        crate::input::rotate_ingest_endpoint_credentials_input::Builder::default()
    }
    /// Creates a new `RotateIngestEndpointCredentials` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for RotateIngestEndpointCredentials {
    type Output = std::result::Result<
        crate::output::RotateIngestEndpointCredentialsOutput,
        crate::error::RotateIngestEndpointCredentialsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_rotate_ingest_endpoint_credentials_error(response)
        } else {
            crate::operation_deser::parse_rotate_ingest_endpoint_credentials_response(response)
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

/// Operation shape for `UpdateChannel`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_channel`](crate::client::Client::update_channel).
///
/// See [`crate::client::fluent_builders::UpdateChannel`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateChannel {
    _private: (),
}
impl UpdateChannel {
    /// Creates a new builder-style object to manufacture [`UpdateChannelInput`](crate::input::UpdateChannelInput).
    pub fn builder() -> crate::input::update_channel_input::Builder {
        crate::input::update_channel_input::Builder::default()
    }
    /// Creates a new `UpdateChannel` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateChannel {
    type Output =
        std::result::Result<crate::output::UpdateChannelOutput, crate::error::UpdateChannelError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_channel_error(response)
        } else {
            crate::operation_deser::parse_update_channel_response(response)
        }
    }
}

/// Operation shape for `UpdateOriginEndpoint`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_origin_endpoint`](crate::client::Client::update_origin_endpoint).
///
/// See [`crate::client::fluent_builders::UpdateOriginEndpoint`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateOriginEndpoint {
    _private: (),
}
impl UpdateOriginEndpoint {
    /// Creates a new builder-style object to manufacture [`UpdateOriginEndpointInput`](crate::input::UpdateOriginEndpointInput).
    pub fn builder() -> crate::input::update_origin_endpoint_input::Builder {
        crate::input::update_origin_endpoint_input::Builder::default()
    }
    /// Creates a new `UpdateOriginEndpoint` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateOriginEndpoint {
    type Output = std::result::Result<
        crate::output::UpdateOriginEndpointOutput,
        crate::error::UpdateOriginEndpointError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_origin_endpoint_error(response)
        } else {
            crate::operation_deser::parse_update_origin_endpoint_response(response)
        }
    }
}

/// Operation customization and supporting types
pub mod customize;
