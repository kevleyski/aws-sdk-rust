// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>Amazon S3 on Outposts Access Points simplify managing data access at scale for shared datasets in S3 on Outposts.
/// S3 on Outposts uses endpoints to connect to Outposts buckets so that you can perform actions within your
/// virtual private cloud (VPC). For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/AccessingS3Outposts.html">
/// Accessing S3 on Outposts using VPC only access points</a>.</p>
/// <p>This action creates an endpoint and associates it with the specified Outposts.</p>
/// <note>
/// <p>It can take up to 5 minutes for this action to complete.</p>
/// </note>
/// <p></p>
/// <p>Related actions include:</p>
/// <ul>
/// <li>
/// <p>
/// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_s3outposts_DeleteEndpoint.html">DeleteEndpoint</a>
/// </p>
/// </li>
/// <li>
/// <p>
/// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_s3outposts_ListEndpoints.html">ListEndpoints</a>
/// </p>
/// </li>
/// </ul>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateEndpoint {
    _private: (),
}
impl CreateEndpoint {
    /// Creates a new builder-style object to manufacture [`CreateEndpointInput`](crate::input::CreateEndpointInput)
    pub fn builder() -> crate::input::create_endpoint_input::Builder {
        crate::input::create_endpoint_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateEndpoint {
    type Output =
        std::result::Result<crate::output::CreateEndpointOutput, crate::error::CreateEndpointError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_endpoint_error(response)
        } else {
            crate::operation_deser::parse_create_endpoint_response(response)
        }
    }
}

/// <p>Amazon S3 on Outposts Access Points simplify managing data access at scale for shared datasets in S3 on Outposts.
/// S3 on Outposts uses endpoints to connect to Outposts buckets so that you can perform actions within your
/// virtual private cloud (VPC). For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/AccessingS3Outposts.html">
/// Accessing S3 on Outposts using VPC only access points</a>.</p>
/// <p>This action deletes an endpoint.</p>
/// <note>
/// <p>It can take up to 5 minutes for this action to complete.</p>
/// </note>
/// <p></p>
/// <p>Related actions include:</p>
/// <ul>
/// <li>
/// <p>
/// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_s3outposts_CreateEndpoint.html">CreateEndpoint</a>
/// </p>
/// </li>
/// <li>
/// <p>
/// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_s3outposts_ListEndpoints.html">ListEndpoints</a>
/// </p>
/// </li>
/// </ul>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteEndpoint {
    _private: (),
}
impl DeleteEndpoint {
    /// Creates a new builder-style object to manufacture [`DeleteEndpointInput`](crate::input::DeleteEndpointInput)
    pub fn builder() -> crate::input::delete_endpoint_input::Builder {
        crate::input::delete_endpoint_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteEndpoint {
    type Output =
        std::result::Result<crate::output::DeleteEndpointOutput, crate::error::DeleteEndpointError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_endpoint_error(response)
        } else {
            crate::operation_deser::parse_delete_endpoint_response(response)
        }
    }
}

/// <p>Amazon S3 on Outposts Access Points simplify managing data access at scale for shared datasets in S3 on Outposts.
/// S3 on Outposts uses endpoints to connect to Outposts buckets so that you can perform actions within your
/// virtual private cloud (VPC). For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/AccessingS3Outposts.html">
/// Accessing S3 on Outposts using VPC only access points</a>.</p>
/// <p>This action lists endpoints associated with the Outposts.
/// </p>
/// <p></p>
/// <p>Related actions include:</p>
/// <ul>
/// <li>
/// <p>
/// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_s3outposts_CreateEndpoint.html">CreateEndpoint</a>
/// </p>
/// </li>
/// <li>
/// <p>
/// <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_s3outposts_DeleteEndpoint.html">DeleteEndpoint</a>
/// </p>
/// </li>
/// </ul>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListEndpoints {
    _private: (),
}
impl ListEndpoints {
    /// Creates a new builder-style object to manufacture [`ListEndpointsInput`](crate::input::ListEndpointsInput)
    pub fn builder() -> crate::input::list_endpoints_input::Builder {
        crate::input::list_endpoints_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListEndpoints {
    type Output =
        std::result::Result<crate::output::ListEndpointsOutput, crate::error::ListEndpointsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_endpoints_error(response)
        } else {
            crate::operation_deser::parse_list_endpoints_response(response)
        }
    }
}
