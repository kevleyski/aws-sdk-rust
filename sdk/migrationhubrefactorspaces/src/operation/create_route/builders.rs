// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_route::_create_route_output::CreateRouteOutputBuilder;

pub use crate::operation::create_route::_create_route_input::CreateRouteInputBuilder;

/// Fluent builder constructing a request to `CreateRoute`.
///
/// <p>Creates an Amazon Web Services Migration Hub Refactor Spaces route. The account owner of the service resource is always the environment owner, regardless of which account creates the route. Routes target a service in the application. If an application does not have any routes, then the first route must be created as a <code>DEFAULT</code> <code>RouteType</code>.</p>
/// <p>When created, the default route defaults to an active state so state is not a required input. However, like all other state values the state of the default route can be updated after creation, but only when all other routes are also inactive. Conversely, no route can be active without the default route also being active.</p>
/// <p>When you create a route, Refactor Spaces configures the Amazon API Gateway to send traffic to the target service as follows:</p>
/// <ul>
/// <li> <p>If the service has a URL endpoint, and the endpoint resolves to a private IP address, Refactor Spaces routes traffic using the API Gateway VPC link. </p> </li>
/// <li> <p>If the service has a URL endpoint, and the endpoint resolves to a public IP address, Refactor Spaces routes traffic over the public internet.</p> </li>
/// <li> <p>If the service has an Lambda function endpoint, then Refactor Spaces configures the Lambda function's resource policy to allow the application's API Gateway to invoke the function.</p> </li>
/// </ul>
/// <p>A one-time health check is performed on the service when either the route is updated from inactive to active, or when it is created with an active state. If the health check fails, the route transitions the route state to <code>FAILED</code>, an error code of <code>SERVICE_ENDPOINT_HEALTH_CHECK_FAILURE</code> is provided, and no traffic is sent to the service.</p>
/// <p>For Lambda functions, the Lambda function state is checked. If the function is not active, the function configuration is updated so that Lambda resources are provisioned. If the Lambda state is <code>Failed</code>, then the route creation fails. For more information, see the <a href="https://docs.aws.amazon.com/lambda/latest/dg/API_GetFunctionConfiguration.html#SSS-GetFunctionConfiguration-response-State">GetFunctionConfiguration's State response parameter</a> in the <i>Lambda Developer Guide</i>.</p>
/// <p>For Lambda endpoints, a check is performed to determine that a Lambda function with the specified ARN exists. If it does not exist, the health check fails. For public URLs, a connection is opened to the public endpoint. If the URL is not reachable, the health check fails. </p>
/// <p>Refactor Spaces automatically resolves the public Domain Name System (DNS) names that are set in <code>CreateServiceRequest$UrlEndpoint</code> when you create a service. The DNS names resolve when the DNS time-to-live (TTL) expires, or every 60 seconds for TTLs less than 60 seconds. This periodic DNS resolution ensures that the route configuration remains up-to-date. </p>
/// <p>For private URLS, a target group is created on the Elastic Load Balancing and the target group health check is run. The <code>HealthCheckProtocol</code>, <code>HealthCheckPort</code>, and <code>HealthCheckPath</code> are the same protocol, port, and path specified in the URL or health URL, if used. All other settings use the default values, as described in <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/application/target-group-health-checks.html">Health checks for your target groups</a>. The health check is considered successful if at least one target within the target group transitions to a healthy state.</p>
/// <p>Services can have HTTP or HTTPS URL endpoints. For HTTPS URLs, publicly-signed certificates are supported. Private Certificate Authorities (CAs) are permitted only if the CA's domain is also publicly resolvable.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateRouteFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_route::builders::CreateRouteInputBuilder,
}
impl CreateRouteFluentBuilder {
    /// Creates a new `CreateRoute`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::create_route::CreateRoute,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::create_route::CreateRouteError>,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        Ok(crate::client::customize::CustomizableOperation { handle, operation })
    }

    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> std::result::Result<
        crate::operation::create_route::CreateRouteOutput,
        aws_smithy_http::result::SdkError<crate::operation::create_route::CreateRouteError>,
    > {
        let op = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// <p>The ID of the environment in which the route is created.</p>
    pub fn environment_identifier(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.environment_identifier(input.into());
        self
    }
    /// <p>The ID of the environment in which the route is created.</p>
    pub fn set_environment_identifier(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_environment_identifier(input);
        self
    }
    /// <p>The ID of the application within which the route is being created.</p>
    pub fn application_identifier(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.application_identifier(input.into());
        self
    }
    /// <p>The ID of the application within which the route is being created.</p>
    pub fn set_application_identifier(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_application_identifier(input);
        self
    }
    /// <p>The ID of the service in which the route is created. Traffic that matches this route is forwarded to this service.</p>
    pub fn service_identifier(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.service_identifier(input.into());
        self
    }
    /// <p>The ID of the service in which the route is created. Traffic that matches this route is forwarded to this service.</p>
    pub fn set_service_identifier(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_service_identifier(input);
        self
    }
    /// <p>The route type of the route. <code>DEFAULT</code> indicates that all traffic that does not match another route is forwarded to the default route. Applications must have a default route before any other routes can be created. <code>URI_PATH</code> indicates a route that is based on a URI path.</p>
    pub fn route_type(mut self, input: crate::types::RouteType) -> Self {
        self.inner = self.inner.route_type(input);
        self
    }
    /// <p>The route type of the route. <code>DEFAULT</code> indicates that all traffic that does not match another route is forwarded to the default route. Applications must have a default route before any other routes can be created. <code>URI_PATH</code> indicates a route that is based on a URI path.</p>
    pub fn set_route_type(mut self, input: std::option::Option<crate::types::RouteType>) -> Self {
        self.inner = self.inner.set_route_type(input);
        self
    }
    /// <p> Configuration for the default route type. </p>
    pub fn default_route(mut self, input: crate::types::DefaultRouteInput) -> Self {
        self.inner = self.inner.default_route(input);
        self
    }
    /// <p> Configuration for the default route type. </p>
    pub fn set_default_route(
        mut self,
        input: std::option::Option<crate::types::DefaultRouteInput>,
    ) -> Self {
        self.inner = self.inner.set_default_route(input);
        self
    }
    /// <p>The configuration for the URI path route type. </p>
    pub fn uri_path_route(mut self, input: crate::types::UriPathRouteInput) -> Self {
        self.inner = self.inner.uri_path_route(input);
        self
    }
    /// <p>The configuration for the URI path route type. </p>
    pub fn set_uri_path_route(
        mut self,
        input: std::option::Option<crate::types::UriPathRouteInput>,
    ) -> Self {
        self.inner = self.inner.set_uri_path_route(input);
        self
    }
    /// Adds a key-value pair to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags to assign to the route. A tag is a label that you assign to an Amazon Web Services resource. Each tag consists of a key-value pair.. </p>
    pub fn tags(
        mut self,
        k: impl Into<std::string::String>,
        v: impl Into<std::string::String>,
    ) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>The tags to assign to the route. A tag is a label that you assign to an Amazon Web Services resource. Each tag consists of a key-value pair.. </p>
    pub fn set_tags(
        mut self,
        input: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
}
