// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;

impl RegisterComputeInput {
    /// Consumes the builder and constructs an Operation<[`RegisterCompute`](crate::operation::register_compute::RegisterCompute)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::register_compute::RegisterCompute,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::operation::error::BuildError,
    > {
        let params_result = crate::endpoint::Params::builder()
            .set_region(_config.region.as_ref().map(|r| r.as_ref().to_owned()))
            .set_use_dual_stack(_config.use_dual_stack)
            .set_use_fips(_config.use_fips)
            .set_endpoint(_config.endpoint_url.clone())
            .build()
            .map_err(|err| {
                aws_smithy_http::endpoint::ResolveEndpointError::from_source(
                    "could not construct endpoint parameters",
                    err,
                )
            });
        let (endpoint_result, params) = match params_result {
            Ok(params) => (
                _config.endpoint_resolver.resolve_endpoint(&params),
                Some(params),
            ),
            Err(e) => (Err(e), None),
        };
        let mut request = {
            fn uri_base(
                _input: &crate::operation::register_compute::RegisterComputeInput,
                output: &mut String,
            ) -> std::result::Result<(), aws_smithy_http::operation::error::BuildError>
            {
                write!(output, "/").expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::register_compute::RegisterComputeInput,
                builder: http::request::Builder,
            ) -> std::result::Result<
                http::request::Builder,
                aws_smithy_http::operation::error::BuildError,
            > {
                let mut uri = String::new();
                uri_base(input, &mut uri)?;
                Ok(builder.method("POST").uri(uri))
            }
            let mut builder = update_http_builder(&self, http::request::Builder::new())?;
            builder = aws_smithy_http::header::set_request_header_if_absent(
                builder,
                http::header::CONTENT_TYPE,
                "application/x-amz-json-1.1",
            );
            builder = aws_smithy_http::header::set_request_header_if_absent(
                builder,
                http::header::HeaderName::from_static("x-amz-target"),
                "GameLift.RegisterCompute",
            );
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from(
            crate::protocol_serde::shape_register_compute::ser_register_compute_input(&self)?,
        );
        if let Some(content_length) = body.content_length() {
            request = aws_smithy_http::header::set_request_header_if_absent(
                request,
                http::header::CONTENT_LENGTH,
                content_length,
            );
        }
        let request = request.body(body).expect("should be valid request");
        let mut request = aws_smithy_http::operation::Request::from_parts(request, properties);
        request.properties_mut().insert(endpoint_result);
        if let Some(params) = params {
            request.properties_mut().insert(params);
        }
        request
            .properties_mut()
            .insert(aws_smithy_http::http_versions::DEFAULT_HTTP_VERSION_LIST.clone());
        let mut user_agent = aws_http::user_agent::AwsUserAgent::new_from_environment(
            aws_types::os_shim_internal::Env::real(),
            crate::meta::API_METADATA.clone(),
        );
        if let Some(app_name) = _config.app_name() {
            user_agent = user_agent.with_app_name(app_name.clone());
        }
        request.properties_mut().insert(user_agent);
        let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
        request.properties_mut().insert(signing_config);
        request
            .properties_mut()
            .insert(aws_types::SigningService::from_static(
                _config.signing_service(),
            ));
        if let Some(region) = &_config.region {
            request
                .properties_mut()
                .insert(aws_types::region::SigningRegion::from(region.clone()));
        }
        if let Some(region) = &_config.region {
            request.properties_mut().insert(region.clone());
        }
        aws_http::auth::set_credentials_cache(
            &mut request.properties_mut(),
            _config.credentials_cache.clone(),
        );
        let op = aws_smithy_http::operation::Operation::new(
            request,
            crate::operation::register_compute::RegisterCompute::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "RegisterCompute",
            "gamelift",
        ));
        let op = op.with_retry_classifier(aws_http::retry::AwsResponseRetryClassifier::new());
        Ok(op)
    }
}
/// `ParseStrictResponse` impl for `RegisterCompute`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct RegisterCompute;
impl RegisterCompute {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
}
impl aws_smithy_http::response::ParseStrictResponse for RegisterCompute {
    type Output = std::result::Result<
        crate::operation::register_compute::RegisterComputeOutput,
        crate::operation::register_compute::RegisterComputeError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::protocol_serde::shape_register_compute::de_register_compute_http_error(response)
        } else {
            crate::protocol_serde::shape_register_compute::de_register_compute_http_response(
                response,
            )
        }
    }
}

/// Do not use this.
///
/// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
#[deprecated(
    note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now)."
)]
pub type RegisterComputeErrorKind = RegisterComputeError;
/// Error type for the `RegisterComputeError` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum RegisterComputeError {
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request.</p>
    /// <p></p>
    ConflictException(crate::types::error::ConflictException),
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalServiceException(crate::types::error::InternalServiceException),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequestException(crate::types::error::InvalidRequestException),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    UnauthorizedException(crate::types::error::UnauthorizedException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(aws_smithy_types::error::Unhandled),
}
impl aws_smithy_http::result::CreateUnhandledError for RegisterComputeError {
    fn create_unhandled_error(
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
        meta: std::option::Option<aws_smithy_types::error::ErrorMetadata>,
    ) -> Self {
        Self::Unhandled({
            let mut builder = aws_smithy_types::error::Unhandled::builder().source(source);
            builder.set_meta(meta);
            builder.build()
        })
    }
}
impl std::fmt::Display for RegisterComputeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConflictException(_inner) => _inner.fmt(f),
            Self::InternalServiceException(_inner) => _inner.fmt(f),
            Self::InvalidRequestException(_inner) => _inner.fmt(f),
            Self::UnauthorizedException(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata for RegisterComputeError {
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::ConflictException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InternalServiceException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidRequestException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::UnauthorizedException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::Unhandled(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl aws_http::request_id::RequestId for crate::operation::register_compute::RegisterComputeError {
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for RegisterComputeError {
    fn code(&self) -> std::option::Option<&str> {
        aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> std::option::Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl RegisterComputeError {
    /// Creates the `RegisterComputeError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self::Unhandled(
            aws_smithy_types::error::Unhandled::builder()
                .source(err)
                .build(),
        )
    }

    /// Creates the `RegisterComputeError::Unhandled` variant from a `aws_smithy_types::error::ErrorMetadata`.
    pub fn generic(err: aws_smithy_types::error::ErrorMetadata) -> Self {
        Self::Unhandled(
            aws_smithy_types::error::Unhandled::builder()
                .source(err.clone())
                .meta(err)
                .build(),
        )
    }
    ///
    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    ///
    pub fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        use aws_smithy_types::error::metadata::ProvideErrorMetadata;
        match self {
            Self::ConflictException(e) => e.meta(),
            Self::InternalServiceException(e) => e.meta(),
            Self::InvalidRequestException(e) => e.meta(),
            Self::UnauthorizedException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `RegisterComputeError::ConflictException`.
    pub fn is_conflict_exception(&self) -> bool {
        matches!(self, Self::ConflictException(_))
    }
    /// Returns `true` if the error kind is `RegisterComputeError::InternalServiceException`.
    pub fn is_internal_service_exception(&self) -> bool {
        matches!(self, Self::InternalServiceException(_))
    }
    /// Returns `true` if the error kind is `RegisterComputeError::InvalidRequestException`.
    pub fn is_invalid_request_exception(&self) -> bool {
        matches!(self, Self::InvalidRequestException(_))
    }
    /// Returns `true` if the error kind is `RegisterComputeError::UnauthorizedException`.
    pub fn is_unauthorized_exception(&self) -> bool {
        matches!(self, Self::UnauthorizedException(_))
    }
}
impl std::error::Error for RegisterComputeError {
    fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ConflictException(_inner) => Some(_inner),
            Self::InternalServiceException(_inner) => Some(_inner),
            Self::InvalidRequestException(_inner) => Some(_inner),
            Self::UnauthorizedException(_inner) => Some(_inner),
            Self::Unhandled(_inner) => Some(_inner),
        }
    }
}

pub use crate::operation::register_compute::_register_compute_output::RegisterComputeOutput;

pub use crate::operation::register_compute::_register_compute_input::RegisterComputeInput;

mod _register_compute_input;

mod _register_compute_output;

/// Builders
pub mod builders;
