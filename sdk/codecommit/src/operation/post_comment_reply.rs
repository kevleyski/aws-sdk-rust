// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;

impl PostCommentReplyInput {
    /// Consumes the builder and constructs an Operation<[`PostCommentReply`](crate::operation::post_comment_reply::PostCommentReply)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        mut self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::post_comment_reply::PostCommentReply,
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
        if self.client_request_token.is_none() {
            self.client_request_token = Some(_config.make_token.make_idempotency_token());
        }
        let mut request = {
            fn uri_base(
                _input: &crate::operation::post_comment_reply::PostCommentReplyInput,
                output: &mut String,
            ) -> std::result::Result<(), aws_smithy_http::operation::error::BuildError>
            {
                write!(output, "/").expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::post_comment_reply::PostCommentReplyInput,
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
                "CodeCommit_20150413.PostCommentReply",
            );
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from(
            crate::protocol_serde::shape_post_comment_reply::ser_post_comment_reply_input(&self)?,
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
            crate::operation::post_comment_reply::PostCommentReply::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "PostCommentReply",
            "codecommit",
        ));
        let op = op.with_retry_classifier(aws_http::retry::AwsResponseRetryClassifier::new());
        Ok(op)
    }
}
/// `ParseStrictResponse` impl for `PostCommentReply`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct PostCommentReply;
impl PostCommentReply {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PostCommentReply {
    type Output = std::result::Result<
        crate::operation::post_comment_reply::PostCommentReplyOutput,
        crate::operation::post_comment_reply::PostCommentReplyError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::protocol_serde::shape_post_comment_reply::de_post_comment_reply_http_error(
                response,
            )
        } else {
            crate::protocol_serde::shape_post_comment_reply::de_post_comment_reply_http_response(
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
pub type PostCommentReplyErrorKind = PostCommentReplyError;
/// Error type for the `PostCommentReplyError` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum PostCommentReplyError {
    /// <p>A client request token is required. A client request token is an unique, client-generated idempotency token that, when provided in a request, ensures the request cannot be repeated with a changed parameter. If a request is received with the same parameters and a token is included, the request returns information about the initial request that used that token.</p>
    ClientRequestTokenRequiredException(crate::types::error::ClientRequestTokenRequiredException),
    /// <p>The comment is empty. You must provide some content for a comment. The content cannot be null.</p>
    CommentContentRequiredException(crate::types::error::CommentContentRequiredException),
    /// <p>The comment is too large. Comments are limited to 1,000 characters.</p>
    CommentContentSizeLimitExceededException(
        crate::types::error::CommentContentSizeLimitExceededException,
    ),
    /// <p>No comment exists with the provided ID. Verify that you have used the correct ID, and then try again.</p>
    CommentDoesNotExistException(crate::types::error::CommentDoesNotExistException),
    /// <p>The comment ID is missing or null. A comment ID is required.</p>
    CommentIdRequiredException(crate::types::error::CommentIdRequiredException),
    /// <p>The client request token is not valid. Either the token is not in a valid format, or the token has been used in a previous request and cannot be reused.</p>
    IdempotencyParameterMismatchException(
        crate::types::error::IdempotencyParameterMismatchException,
    ),
    /// <p>The client request token is not valid.</p>
    InvalidClientRequestTokenException(crate::types::error::InvalidClientRequestTokenException),
    /// <p>The comment ID is not in a valid format. Make sure that you have provided the full comment ID.</p>
    InvalidCommentIdException(crate::types::error::InvalidCommentIdException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(aws_smithy_types::error::Unhandled),
}
impl aws_smithy_http::result::CreateUnhandledError for PostCommentReplyError {
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
impl std::fmt::Display for PostCommentReplyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ClientRequestTokenRequiredException(_inner) => _inner.fmt(f),
            Self::CommentContentRequiredException(_inner) => _inner.fmt(f),
            Self::CommentContentSizeLimitExceededException(_inner) => _inner.fmt(f),
            Self::CommentDoesNotExistException(_inner) => _inner.fmt(f),
            Self::CommentIdRequiredException(_inner) => _inner.fmt(f),
            Self::IdempotencyParameterMismatchException(_inner) => _inner.fmt(f),
            Self::InvalidClientRequestTokenException(_inner) => _inner.fmt(f),
            Self::InvalidCommentIdException(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata for PostCommentReplyError {
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::ClientRequestTokenRequiredException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::CommentContentRequiredException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::CommentContentSizeLimitExceededException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::CommentDoesNotExistException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::CommentIdRequiredException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::IdempotencyParameterMismatchException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidClientRequestTokenException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidCommentIdException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::Unhandled(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl aws_http::request_id::RequestId
    for crate::operation::post_comment_reply::PostCommentReplyError
{
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for PostCommentReplyError {
    fn code(&self) -> std::option::Option<&str> {
        aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> std::option::Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl PostCommentReplyError {
    /// Creates the `PostCommentReplyError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self::Unhandled(
            aws_smithy_types::error::Unhandled::builder()
                .source(err)
                .build(),
        )
    }

    /// Creates the `PostCommentReplyError::Unhandled` variant from a `aws_smithy_types::error::ErrorMetadata`.
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
            Self::ClientRequestTokenRequiredException(e) => e.meta(),
            Self::CommentContentRequiredException(e) => e.meta(),
            Self::CommentContentSizeLimitExceededException(e) => e.meta(),
            Self::CommentDoesNotExistException(e) => e.meta(),
            Self::CommentIdRequiredException(e) => e.meta(),
            Self::IdempotencyParameterMismatchException(e) => e.meta(),
            Self::InvalidClientRequestTokenException(e) => e.meta(),
            Self::InvalidCommentIdException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `PostCommentReplyError::ClientRequestTokenRequiredException`.
    pub fn is_client_request_token_required_exception(&self) -> bool {
        matches!(self, Self::ClientRequestTokenRequiredException(_))
    }
    /// Returns `true` if the error kind is `PostCommentReplyError::CommentContentRequiredException`.
    pub fn is_comment_content_required_exception(&self) -> bool {
        matches!(self, Self::CommentContentRequiredException(_))
    }
    /// Returns `true` if the error kind is `PostCommentReplyError::CommentContentSizeLimitExceededException`.
    pub fn is_comment_content_size_limit_exceeded_exception(&self) -> bool {
        matches!(self, Self::CommentContentSizeLimitExceededException(_))
    }
    /// Returns `true` if the error kind is `PostCommentReplyError::CommentDoesNotExistException`.
    pub fn is_comment_does_not_exist_exception(&self) -> bool {
        matches!(self, Self::CommentDoesNotExistException(_))
    }
    /// Returns `true` if the error kind is `PostCommentReplyError::CommentIdRequiredException`.
    pub fn is_comment_id_required_exception(&self) -> bool {
        matches!(self, Self::CommentIdRequiredException(_))
    }
    /// Returns `true` if the error kind is `PostCommentReplyError::IdempotencyParameterMismatchException`.
    pub fn is_idempotency_parameter_mismatch_exception(&self) -> bool {
        matches!(self, Self::IdempotencyParameterMismatchException(_))
    }
    /// Returns `true` if the error kind is `PostCommentReplyError::InvalidClientRequestTokenException`.
    pub fn is_invalid_client_request_token_exception(&self) -> bool {
        matches!(self, Self::InvalidClientRequestTokenException(_))
    }
    /// Returns `true` if the error kind is `PostCommentReplyError::InvalidCommentIdException`.
    pub fn is_invalid_comment_id_exception(&self) -> bool {
        matches!(self, Self::InvalidCommentIdException(_))
    }
}
impl std::error::Error for PostCommentReplyError {
    fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ClientRequestTokenRequiredException(_inner) => Some(_inner),
            Self::CommentContentRequiredException(_inner) => Some(_inner),
            Self::CommentContentSizeLimitExceededException(_inner) => Some(_inner),
            Self::CommentDoesNotExistException(_inner) => Some(_inner),
            Self::CommentIdRequiredException(_inner) => Some(_inner),
            Self::IdempotencyParameterMismatchException(_inner) => Some(_inner),
            Self::InvalidClientRequestTokenException(_inner) => Some(_inner),
            Self::InvalidCommentIdException(_inner) => Some(_inner),
            Self::Unhandled(_inner) => Some(_inner),
        }
    }
}

pub use crate::operation::post_comment_reply::_post_comment_reply_output::PostCommentReplyOutput;

pub use crate::operation::post_comment_reply::_post_comment_reply_input::PostCommentReplyInput;

mod _post_comment_reply_input;

mod _post_comment_reply_output;

/// Builders
pub mod builders;
