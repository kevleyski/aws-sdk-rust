// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;
/// See [`CreateEndpointInput`](crate::input::CreateEndpointInput)
pub mod create_endpoint_input {
    /// A builder for [`CreateEndpointInput`](crate::input::CreateEndpointInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) outpost_id: std::option::Option<std::string::String>,
        pub(crate) subnet_id: std::option::Option<std::string::String>,
        pub(crate) security_group_id: std::option::Option<std::string::String>,
        pub(crate) access_type: std::option::Option<crate::model::EndpointAccessType>,
        pub(crate) customer_owned_ipv4_pool: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The ID of the AWS Outposts. </p>
        pub fn outpost_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.outpost_id = Some(input.into());
            self
        }
        pub fn set_outpost_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.outpost_id = input;
            self
        }
        /// <p>The ID of the subnet in the selected VPC. The endpoint subnet
        /// must belong to the Outpost that has the Amazon S3 on Outposts provisioned.</p>
        pub fn subnet_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.subnet_id = Some(input.into());
            self
        }
        pub fn set_subnet_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.subnet_id = input;
            self
        }
        /// <p>The ID of the security group to use with the endpoint.</p>
        pub fn security_group_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.security_group_id = Some(input.into());
            self
        }
        pub fn set_security_group_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.security_group_id = input;
            self
        }
        /// <p>The type of access for the on-premise network connectivity for the
        /// Outpost endpoint. To access the endpoint from an on-premises network, you must
        /// specify the access type and provide the customer owned IPv4 pool.</p>
        pub fn access_type(mut self, input: crate::model::EndpointAccessType) -> Self {
            self.access_type = Some(input);
            self
        }
        pub fn set_access_type(
            mut self,
            input: std::option::Option<crate::model::EndpointAccessType>,
        ) -> Self {
            self.access_type = input;
            self
        }
        /// <p>The ID of the customer-owned IPv4 pool for the endpoint.
        /// IP addresses will be allocated from this pool for the endpoint.</p>
        pub fn customer_owned_ipv4_pool(mut self, input: impl Into<std::string::String>) -> Self {
            self.customer_owned_ipv4_pool = Some(input.into());
            self
        }
        pub fn set_customer_owned_ipv4_pool(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.customer_owned_ipv4_pool = input;
            self
        }
        /// Consumes the builder and constructs a [`CreateEndpointInput`](crate::input::CreateEndpointInput)
        pub fn build(
            self,
        ) -> std::result::Result<
            crate::input::CreateEndpointInput,
            smithy_http::operation::BuildError,
        > {
            Ok(crate::input::CreateEndpointInput {
                outpost_id: self.outpost_id,
                subnet_id: self.subnet_id,
                security_group_id: self.security_group_id,
                access_type: self.access_type,
                customer_owned_ipv4_pool: self.customer_owned_ipv4_pool,
            })
        }
    }
}
#[doc(hidden)]
pub type CreateEndpointInputOperationOutputAlias = crate::operation::CreateEndpoint;
#[doc(hidden)]
pub type CreateEndpointInputOperationRetryAlias = aws_http::AwsErrorRetryPolicy;
impl CreateEndpointInput {
    /// Consumes the builder and constructs an Operation<[`CreateEndpoint`](crate::operation::CreateEndpoint)>
    #[allow(clippy::let_and_return)]
    pub fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        smithy_http::operation::Operation<
            crate::operation::CreateEndpoint,
            aws_http::AwsErrorRetryPolicy,
        >,
        smithy_http::operation::BuildError,
    > {
        Ok({
            let request = self.request_builder_base()?;
            let body = crate::operation_ser::serialize_operation_create_endpoint(&self).map_err(
                |err| smithy_http::operation::BuildError::SerializationError(err.into()),
            )?;
            let request = Self::assemble(request, body);
            #[allow(unused_mut)]
            let mut request =
                smithy_http::operation::Request::new(request.map(smithy_http::body::SdkBody::from));
            request.properties_mut().insert(
                aws_http::user_agent::AwsUserAgent::new_from_environment(
                    crate::API_METADATA.clone(),
                ),
            );
            #[allow(unused_mut)]
            let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
            request.properties_mut().insert(signing_config);
            request
                .properties_mut()
                .insert(aws_types::SigningService::from_static(
                    _config.signing_service(),
                ));
            aws_endpoint::set_endpoint_resolver(
                &mut request.properties_mut(),
                _config.endpoint_resolver.clone(),
            );
            if let Some(region) = &_config.region {
                request.properties_mut().insert(region.clone());
            }
            aws_auth::provider::set_provider(
                &mut request.properties_mut(),
                _config.credentials_provider.clone(),
            );
            let op = smithy_http::operation::Operation::new(
                request,
                crate::operation::CreateEndpoint::new(),
            )
            .with_metadata(smithy_http::operation::Metadata::new(
                "CreateEndpoint",
                "s3outposts",
            ));
            let op = op.with_retry_policy(aws_http::AwsErrorRetryPolicy::new());
            op
        })
    }
    fn uri_base(&self, output: &mut String) -> Result<(), smithy_http::operation::BuildError> {
        write!(output, "/S3Outposts/CreateEndpoint").expect("formatting should succeed");
        Ok(())
    }
    #[allow(clippy::unnecessary_wraps)]
    fn update_http_builder(
        &self,
        builder: http::request::Builder,
    ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError> {
        let mut uri = String::new();
        self.uri_base(&mut uri)?;
        Ok(builder.method("POST").uri(uri))
    }
    #[allow(clippy::unnecessary_wraps)]
    fn request_builder_base(
        &self,
    ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError> {
        let mut builder = self.update_http_builder(http::request::Builder::new())?;
        builder =
            smithy_http::header::set_header_if_absent(builder, "content-type", "application/json");
        Ok(builder)
    }
    fn assemble(
        mut builder: http::request::Builder,
        body: smithy_http::body::SdkBody,
    ) -> http::request::Request<smithy_http::body::SdkBody> {
        if let Some(content_length) = body.content_length() {
            builder = builder.header(http::header::CONTENT_LENGTH, content_length)
        }
        builder.body(body).expect("should be valid request")
    }
    /// Creates a new builder-style object to manufacture [`CreateEndpointInput`](crate::input::CreateEndpointInput)
    pub fn builder() -> crate::input::create_endpoint_input::Builder {
        crate::input::create_endpoint_input::Builder::default()
    }
}

/// See [`DeleteEndpointInput`](crate::input::DeleteEndpointInput)
pub mod delete_endpoint_input {
    /// A builder for [`DeleteEndpointInput`](crate::input::DeleteEndpointInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) endpoint_id: std::option::Option<std::string::String>,
        pub(crate) outpost_id: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The ID of the endpoint.</p>
        pub fn endpoint_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.endpoint_id = Some(input.into());
            self
        }
        pub fn set_endpoint_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.endpoint_id = input;
            self
        }
        /// <p>The ID of the AWS Outposts. </p>
        pub fn outpost_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.outpost_id = Some(input.into());
            self
        }
        pub fn set_outpost_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.outpost_id = input;
            self
        }
        /// Consumes the builder and constructs a [`DeleteEndpointInput`](crate::input::DeleteEndpointInput)
        pub fn build(
            self,
        ) -> std::result::Result<
            crate::input::DeleteEndpointInput,
            smithy_http::operation::BuildError,
        > {
            Ok(crate::input::DeleteEndpointInput {
                endpoint_id: self.endpoint_id,
                outpost_id: self.outpost_id,
            })
        }
    }
}
#[doc(hidden)]
pub type DeleteEndpointInputOperationOutputAlias = crate::operation::DeleteEndpoint;
#[doc(hidden)]
pub type DeleteEndpointInputOperationRetryAlias = aws_http::AwsErrorRetryPolicy;
impl DeleteEndpointInput {
    /// Consumes the builder and constructs an Operation<[`DeleteEndpoint`](crate::operation::DeleteEndpoint)>
    #[allow(clippy::let_and_return)]
    pub fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        smithy_http::operation::Operation<
            crate::operation::DeleteEndpoint,
            aws_http::AwsErrorRetryPolicy,
        >,
        smithy_http::operation::BuildError,
    > {
        Ok({
            let request = self.request_builder_base()?;
            let body = smithy_http::body::SdkBody::from("");
            let request = Self::assemble(request, body);
            #[allow(unused_mut)]
            let mut request =
                smithy_http::operation::Request::new(request.map(smithy_http::body::SdkBody::from));
            request.properties_mut().insert(
                aws_http::user_agent::AwsUserAgent::new_from_environment(
                    crate::API_METADATA.clone(),
                ),
            );
            #[allow(unused_mut)]
            let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
            request.properties_mut().insert(signing_config);
            request
                .properties_mut()
                .insert(aws_types::SigningService::from_static(
                    _config.signing_service(),
                ));
            aws_endpoint::set_endpoint_resolver(
                &mut request.properties_mut(),
                _config.endpoint_resolver.clone(),
            );
            if let Some(region) = &_config.region {
                request.properties_mut().insert(region.clone());
            }
            aws_auth::provider::set_provider(
                &mut request.properties_mut(),
                _config.credentials_provider.clone(),
            );
            let op = smithy_http::operation::Operation::new(
                request,
                crate::operation::DeleteEndpoint::new(),
            )
            .with_metadata(smithy_http::operation::Metadata::new(
                "DeleteEndpoint",
                "s3outposts",
            ));
            let op = op.with_retry_policy(aws_http::AwsErrorRetryPolicy::new());
            op
        })
    }
    fn uri_base(&self, output: &mut String) -> Result<(), smithy_http::operation::BuildError> {
        write!(output, "/S3Outposts/DeleteEndpoint").expect("formatting should succeed");
        Ok(())
    }
    fn uri_query(&self, mut output: &mut String) {
        let mut query = smithy_http::query::Writer::new(&mut output);
        if let Some(inner_1) = &self.endpoint_id {
            query.push_kv("endpointId", &smithy_http::query::fmt_string(&inner_1));
        }
        if let Some(inner_2) = &self.outpost_id {
            query.push_kv("outpostId", &smithy_http::query::fmt_string(&inner_2));
        }
    }
    #[allow(clippy::unnecessary_wraps)]
    fn update_http_builder(
        &self,
        builder: http::request::Builder,
    ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError> {
        let mut uri = String::new();
        self.uri_base(&mut uri)?;
        self.uri_query(&mut uri);
        Ok(builder.method("DELETE").uri(uri))
    }
    #[allow(clippy::unnecessary_wraps)]
    fn request_builder_base(
        &self,
    ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError> {
        let mut builder = self.update_http_builder(http::request::Builder::new())?;
        builder =
            smithy_http::header::set_header_if_absent(builder, "content-type", "application/json");
        Ok(builder)
    }
    fn assemble(
        mut builder: http::request::Builder,
        body: smithy_http::body::SdkBody,
    ) -> http::request::Request<smithy_http::body::SdkBody> {
        if let Some(content_length) = body.content_length() {
            builder = builder.header(http::header::CONTENT_LENGTH, content_length)
        }
        builder.body(body).expect("should be valid request")
    }
    /// Creates a new builder-style object to manufacture [`DeleteEndpointInput`](crate::input::DeleteEndpointInput)
    pub fn builder() -> crate::input::delete_endpoint_input::Builder {
        crate::input::delete_endpoint_input::Builder::default()
    }
}

/// See [`ListEndpointsInput`](crate::input::ListEndpointsInput)
pub mod list_endpoints_input {
    /// A builder for [`ListEndpointsInput`](crate::input::ListEndpointsInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) next_token: std::option::Option<std::string::String>,
        pub(crate) max_results: std::option::Option<i32>,
    }
    impl Builder {
        /// <p>The next endpoint requested in the list.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// <p>The max number of endpoints that can be returned on the request.</p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.max_results = Some(input);
            self
        }
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.max_results = input;
            self
        }
        /// Consumes the builder and constructs a [`ListEndpointsInput`](crate::input::ListEndpointsInput)
        pub fn build(
            self,
        ) -> std::result::Result<crate::input::ListEndpointsInput, smithy_http::operation::BuildError>
        {
            Ok(crate::input::ListEndpointsInput {
                next_token: self.next_token,
                max_results: self.max_results.unwrap_or_default(),
            })
        }
    }
}
#[doc(hidden)]
pub type ListEndpointsInputOperationOutputAlias = crate::operation::ListEndpoints;
#[doc(hidden)]
pub type ListEndpointsInputOperationRetryAlias = aws_http::AwsErrorRetryPolicy;
impl ListEndpointsInput {
    /// Consumes the builder and constructs an Operation<[`ListEndpoints`](crate::operation::ListEndpoints)>
    #[allow(clippy::let_and_return)]
    pub fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        smithy_http::operation::Operation<
            crate::operation::ListEndpoints,
            aws_http::AwsErrorRetryPolicy,
        >,
        smithy_http::operation::BuildError,
    > {
        Ok({
            let request = self.request_builder_base()?;
            let body = smithy_http::body::SdkBody::from("");
            let request = Self::assemble(request, body);
            #[allow(unused_mut)]
            let mut request =
                smithy_http::operation::Request::new(request.map(smithy_http::body::SdkBody::from));
            request.properties_mut().insert(
                aws_http::user_agent::AwsUserAgent::new_from_environment(
                    crate::API_METADATA.clone(),
                ),
            );
            #[allow(unused_mut)]
            let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
            request.properties_mut().insert(signing_config);
            request
                .properties_mut()
                .insert(aws_types::SigningService::from_static(
                    _config.signing_service(),
                ));
            aws_endpoint::set_endpoint_resolver(
                &mut request.properties_mut(),
                _config.endpoint_resolver.clone(),
            );
            if let Some(region) = &_config.region {
                request.properties_mut().insert(region.clone());
            }
            aws_auth::provider::set_provider(
                &mut request.properties_mut(),
                _config.credentials_provider.clone(),
            );
            let op = smithy_http::operation::Operation::new(
                request,
                crate::operation::ListEndpoints::new(),
            )
            .with_metadata(smithy_http::operation::Metadata::new(
                "ListEndpoints",
                "s3outposts",
            ));
            let op = op.with_retry_policy(aws_http::AwsErrorRetryPolicy::new());
            op
        })
    }
    fn uri_base(&self, output: &mut String) -> Result<(), smithy_http::operation::BuildError> {
        write!(output, "/S3Outposts/ListEndpoints").expect("formatting should succeed");
        Ok(())
    }
    fn uri_query(&self, mut output: &mut String) {
        let mut query = smithy_http::query::Writer::new(&mut output);
        if let Some(inner_3) = &self.next_token {
            query.push_kv("nextToken", &smithy_http::query::fmt_string(&inner_3));
        }
        if self.max_results != 0 {
            query.push_kv(
                "maxResults",
                &smithy_types::primitive::Encoder::from(self.max_results).encode(),
            );
        }
    }
    #[allow(clippy::unnecessary_wraps)]
    fn update_http_builder(
        &self,
        builder: http::request::Builder,
    ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError> {
        let mut uri = String::new();
        self.uri_base(&mut uri)?;
        self.uri_query(&mut uri);
        Ok(builder.method("GET").uri(uri))
    }
    #[allow(clippy::unnecessary_wraps)]
    fn request_builder_base(
        &self,
    ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError> {
        let mut builder = self.update_http_builder(http::request::Builder::new())?;
        builder =
            smithy_http::header::set_header_if_absent(builder, "content-type", "application/json");
        Ok(builder)
    }
    fn assemble(
        mut builder: http::request::Builder,
        body: smithy_http::body::SdkBody,
    ) -> http::request::Request<smithy_http::body::SdkBody> {
        if let Some(content_length) = body.content_length() {
            builder = builder.header(http::header::CONTENT_LENGTH, content_length)
        }
        builder.body(body).expect("should be valid request")
    }
    /// Creates a new builder-style object to manufacture [`ListEndpointsInput`](crate::input::ListEndpointsInput)
    pub fn builder() -> crate::input::list_endpoints_input::Builder {
        crate::input::list_endpoints_input::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListEndpointsInput {
    /// <p>The next endpoint requested in the list.</p>
    pub next_token: std::option::Option<std::string::String>,
    /// <p>The max number of endpoints that can be returned on the request.</p>
    pub max_results: i32,
}
impl std::fmt::Debug for ListEndpointsInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListEndpointsInput");
        formatter.field("next_token", &self.next_token);
        formatter.field("max_results", &self.max_results);
        formatter.finish()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DeleteEndpointInput {
    /// <p>The ID of the endpoint.</p>
    pub endpoint_id: std::option::Option<std::string::String>,
    /// <p>The ID of the AWS Outposts. </p>
    pub outpost_id: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for DeleteEndpointInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DeleteEndpointInput");
        formatter.field("endpoint_id", &self.endpoint_id);
        formatter.field("outpost_id", &self.outpost_id);
        formatter.finish()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct CreateEndpointInput {
    /// <p>The ID of the AWS Outposts. </p>
    pub outpost_id: std::option::Option<std::string::String>,
    /// <p>The ID of the subnet in the selected VPC. The endpoint subnet
    /// must belong to the Outpost that has the Amazon S3 on Outposts provisioned.</p>
    pub subnet_id: std::option::Option<std::string::String>,
    /// <p>The ID of the security group to use with the endpoint.</p>
    pub security_group_id: std::option::Option<std::string::String>,
    /// <p>The type of access for the on-premise network connectivity for the
    /// Outpost endpoint. To access the endpoint from an on-premises network, you must
    /// specify the access type and provide the customer owned IPv4 pool.</p>
    pub access_type: std::option::Option<crate::model::EndpointAccessType>,
    /// <p>The ID of the customer-owned IPv4 pool for the endpoint.
    /// IP addresses will be allocated from this pool for the endpoint.</p>
    pub customer_owned_ipv4_pool: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for CreateEndpointInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("CreateEndpointInput");
        formatter.field("outpost_id", &self.outpost_id);
        formatter.field("subnet_id", &self.subnet_id);
        formatter.field("security_group_id", &self.security_group_id);
        formatter.field("access_type", &self.access_type);
        formatter.field("customer_owned_ipv4_pool", &self.customer_owned_ipv4_pool);
        formatter.finish()
    }
}
