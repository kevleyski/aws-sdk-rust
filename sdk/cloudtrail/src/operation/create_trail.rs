// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;

impl CreateTrailInput {
    /// Consumes the builder and constructs an Operation<[`CreateTrail`](crate::operation::create_trail::CreateTrail)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::create_trail::CreateTrail,
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
                _input: &crate::operation::create_trail::CreateTrailInput,
                output: &mut String,
            ) -> std::result::Result<(), aws_smithy_http::operation::error::BuildError>
            {
                write!(output, "/").expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::create_trail::CreateTrailInput,
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
                "CloudTrail_20131101.CreateTrail",
            );
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from(
            crate::protocol_serde::shape_create_trail::ser_create_trail_input(&self)?,
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
            crate::operation::create_trail::CreateTrail::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "CreateTrail",
            "cloudtrail",
        ));
        let op = op.with_retry_classifier(aws_http::retry::AwsResponseRetryClassifier::new());
        Ok(op)
    }
}
/// `ParseStrictResponse` impl for `CreateTrail`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct CreateTrail;
impl CreateTrail {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateTrail {
    type Output = std::result::Result<
        crate::operation::create_trail::CreateTrailOutput,
        crate::operation::create_trail::CreateTrailError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::protocol_serde::shape_create_trail::de_create_trail_http_error(response)
        } else {
            crate::protocol_serde::shape_create_trail::de_create_trail_http_response(response)
        }
    }
}

/// Do not use this.
///
/// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
#[deprecated(
    note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now)."
)]
pub type CreateTrailErrorKind = CreateTrailError;
/// Error type for the `CreateTrailError` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum CreateTrailError {
    /// <p>This exception is thrown when trusted access has not been enabled between CloudTrail and Organizations. For more information, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_integrate_services.html">Enabling Trusted Access with Other Amazon Web Services Services</a> and <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/creating-an-organizational-trail-prepare.html">Prepare For Creating a Trail For Your Organization</a>. </p>
    CloudTrailAccessNotEnabledException(crate::types::error::CloudTrailAccessNotEnabledException),
    /// <p>This exception is thrown when a call results in the <code>InvalidClientTokenId</code> error code. This can occur when you are creating or updating a trail to send notifications to an Amazon SNS topic that is in a suspended Amazon Web Services account.</p>
    CloudTrailInvalidClientTokenIdException(
        crate::types::error::CloudTrailInvalidClientTokenIdException,
    ),
    /// <p>Cannot set a CloudWatch Logs delivery for this region.</p>
    CloudWatchLogsDeliveryUnavailableException(
        crate::types::error::CloudWatchLogsDeliveryUnavailableException,
    ),
    /// <p>This exception is thrown when the specified resource is not ready for an operation. This can occur when you try to run an operation on a resource before CloudTrail has time to fully load the resource, or because another operation is modifying the resource. If this exception occurs, wait a few minutes, and then try the operation again.</p>
    ConflictException(crate::types::error::ConflictException),
    /// <p>This exception is thrown when the IAM user or role that is used to create the organization resource lacks one or more required permissions for creating an organization resource in a required service.</p>
    InsufficientDependencyServiceAccessPermissionException(
        crate::types::error::InsufficientDependencyServiceAccessPermissionException,
    ),
    /// <p>This exception is thrown when the policy on the S3 bucket or KMS key does not have sufficient permissions for the operation.</p>
    InsufficientEncryptionPolicyException(
        crate::types::error::InsufficientEncryptionPolicyException,
    ),
    /// <p>This exception is thrown when the policy on the S3 bucket is not sufficient.</p>
    InsufficientS3BucketPolicyException(crate::types::error::InsufficientS3BucketPolicyException),
    /// <p>This exception is thrown when the policy on the Amazon SNS topic is not sufficient.</p>
    InsufficientSnsTopicPolicyException(crate::types::error::InsufficientSnsTopicPolicyException),
    /// <p>This exception is thrown when the provided CloudWatch Logs log group is not valid.</p>
    InvalidCloudWatchLogsLogGroupArnException(
        crate::types::error::InvalidCloudWatchLogsLogGroupArnException,
    ),
    /// <p>This exception is thrown when the provided role is not valid.</p>
    InvalidCloudWatchLogsRoleArnException(
        crate::types::error::InvalidCloudWatchLogsRoleArnException,
    ),
    /// <p>This exception is thrown when the KMS key ARN is not valid.</p>
    InvalidKmsKeyIdException(crate::types::error::InvalidKmsKeyIdException),
    /// <p>This exception is thrown when the combination of parameters provided is not valid.</p>
    InvalidParameterCombinationException(crate::types::error::InvalidParameterCombinationException),
    /// <p>This exception is thrown when the provided S3 bucket name is not valid.</p>
    InvalidS3BucketNameException(crate::types::error::InvalidS3BucketNameException),
    /// <p>This exception is thrown when the provided S3 prefix is not valid.</p>
    InvalidS3PrefixException(crate::types::error::InvalidS3PrefixException),
    /// <p>This exception is thrown when the provided SNS topic name is not valid.</p>
    InvalidSnsTopicNameException(crate::types::error::InvalidSnsTopicNameException),
    /// <p>This exception is thrown when the specified tag key or values are not valid. It can also occur if there are duplicate tags or too many tags on the resource.</p>
    InvalidTagParameterException(crate::types::error::InvalidTagParameterException),
    /// <p>This exception is thrown when the provided trail name is not valid. Trail names must meet the following requirements:</p>
    /// <ul>
    /// <li> <p>Contain only ASCII letters (a-z, A-Z), numbers (0-9), periods (.), underscores (_), or dashes (-)</p> </li>
    /// <li> <p>Start with a letter or number, and end with a letter or number</p> </li>
    /// <li> <p>Be between 3 and 128 characters</p> </li>
    /// <li> <p>Have no adjacent periods, underscores or dashes. Names like <code>my-_namespace</code> and <code>my--namespace</code> are not valid.</p> </li>
    /// <li> <p>Not be in IP address format (for example, 192.168.5.4)</p> </li>
    /// </ul>
    InvalidTrailNameException(crate::types::error::InvalidTrailNameException),
    /// <p>This exception is thrown when there is an issue with the specified KMS key and the trail or event data store can't be updated.</p>
    KmsException(crate::types::error::KmsException),
    /// <p>This exception is no longer in use.</p>
    #[deprecated]
    KmsKeyDisabledException(crate::types::error::KmsKeyDisabledException),
    /// <p>This exception is thrown when the KMS key does not exist, when the S3 bucket and the KMS key are not in the same region, or when the KMS key associated with the Amazon SNS topic either does not exist or is not in the same region.</p>
    KmsKeyNotFoundException(crate::types::error::KmsKeyNotFoundException),
    /// <p>This exception is thrown when the maximum number of trails is reached.</p>
    MaximumNumberOfTrailsExceededException(
        crate::types::error::MaximumNumberOfTrailsExceededException,
    ),
    /// <p> This exception is thrown when the management account does not have a service-linked role. </p>
    NoManagementAccountSlrExistsException(
        crate::types::error::NoManagementAccountSlrExistsException,
    ),
    /// <p>This exception is thrown when the Amazon Web Services account making the request to create or update an organization trail or event data store is not the management account for an organization in Organizations. For more information, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/creating-an-organizational-trail-prepare.html">Prepare For Creating a Trail For Your Organization</a> or <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/query-event-data-store.html">Create an event data store</a>.</p>
    NotOrganizationMasterAccountException(
        crate::types::error::NotOrganizationMasterAccountException,
    ),
    /// <p>This exception is thrown when the requested operation is not permitted.</p>
    OperationNotPermittedException(crate::types::error::OperationNotPermittedException),
    /// <p>This exception is thrown when Organizations is not configured to support all features. All features must be enabled in Organizations to support creating an organization trail or event data store.</p>
    OrganizationNotInAllFeaturesModeException(
        crate::types::error::OrganizationNotInAllFeaturesModeException,
    ),
    /// <p>This exception is thrown when the request is made from an Amazon Web Services account that is not a member of an organization. To make this request, sign in using the credentials of an account that belongs to an organization.</p>
    OrganizationsNotInUseException(crate::types::error::OrganizationsNotInUseException),
    /// <p>This exception is thrown when the specified S3 bucket does not exist.</p>
    S3BucketDoesNotExistException(crate::types::error::S3BucketDoesNotExistException),
    /// <p>The number of tags per trail, event data store, or channel has exceeded the permitted amount. Currently, the limit is 50.</p>
    TagsLimitExceededException(crate::types::error::TagsLimitExceededException),
    /// <p>This exception is thrown when the specified trail already exists.</p>
    TrailAlreadyExistsException(crate::types::error::TrailAlreadyExistsException),
    /// <p>This exception is no longer in use.</p>
    TrailNotProvidedException(crate::types::error::TrailNotProvidedException),
    /// <p>This exception is thrown when the requested operation is not supported.</p>
    UnsupportedOperationException(crate::types::error::UnsupportedOperationException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(aws_smithy_types::error::Unhandled),
}
impl aws_smithy_http::result::CreateUnhandledError for CreateTrailError {
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
impl std::fmt::Display for CreateTrailError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CloudTrailAccessNotEnabledException(_inner) => _inner.fmt(f),
            Self::CloudTrailInvalidClientTokenIdException(_inner) => _inner.fmt(f),
            Self::CloudWatchLogsDeliveryUnavailableException(_inner) => _inner.fmt(f),
            Self::ConflictException(_inner) => _inner.fmt(f),
            Self::InsufficientDependencyServiceAccessPermissionException(_inner) => _inner.fmt(f),
            Self::InsufficientEncryptionPolicyException(_inner) => _inner.fmt(f),
            Self::InsufficientS3BucketPolicyException(_inner) => _inner.fmt(f),
            Self::InsufficientSnsTopicPolicyException(_inner) => _inner.fmt(f),
            Self::InvalidCloudWatchLogsLogGroupArnException(_inner) => _inner.fmt(f),
            Self::InvalidCloudWatchLogsRoleArnException(_inner) => _inner.fmt(f),
            Self::InvalidKmsKeyIdException(_inner) => _inner.fmt(f),
            Self::InvalidParameterCombinationException(_inner) => _inner.fmt(f),
            Self::InvalidS3BucketNameException(_inner) => _inner.fmt(f),
            Self::InvalidS3PrefixException(_inner) => _inner.fmt(f),
            Self::InvalidSnsTopicNameException(_inner) => _inner.fmt(f),
            Self::InvalidTagParameterException(_inner) => _inner.fmt(f),
            Self::InvalidTrailNameException(_inner) => _inner.fmt(f),
            Self::KmsException(_inner) => _inner.fmt(f),
            Self::KmsKeyDisabledException(_inner) => _inner.fmt(f),
            Self::KmsKeyNotFoundException(_inner) => _inner.fmt(f),
            Self::MaximumNumberOfTrailsExceededException(_inner) => _inner.fmt(f),
            Self::NoManagementAccountSlrExistsException(_inner) => _inner.fmt(f),
            Self::NotOrganizationMasterAccountException(_inner) => _inner.fmt(f),
            Self::OperationNotPermittedException(_inner) => _inner.fmt(f),
            Self::OrganizationNotInAllFeaturesModeException(_inner) => _inner.fmt(f),
            Self::OrganizationsNotInUseException(_inner) => _inner.fmt(f),
            Self::S3BucketDoesNotExistException(_inner) => _inner.fmt(f),
            Self::TagsLimitExceededException(_inner) => _inner.fmt(f),
            Self::TrailAlreadyExistsException(_inner) => _inner.fmt(f),
            Self::TrailNotProvidedException(_inner) => _inner.fmt(f),
            Self::UnsupportedOperationException(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata for CreateTrailError {
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::CloudTrailAccessNotEnabledException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::CloudTrailInvalidClientTokenIdException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::CloudWatchLogsDeliveryUnavailableException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ConflictException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InsufficientDependencyServiceAccessPermissionException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InsufficientEncryptionPolicyException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InsufficientS3BucketPolicyException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InsufficientSnsTopicPolicyException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidCloudWatchLogsLogGroupArnException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidCloudWatchLogsRoleArnException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidKmsKeyIdException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidParameterCombinationException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidS3BucketNameException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidS3PrefixException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidSnsTopicNameException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidTagParameterException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidTrailNameException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::KmsException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::KmsKeyDisabledException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::KmsKeyNotFoundException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::MaximumNumberOfTrailsExceededException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::NoManagementAccountSlrExistsException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::NotOrganizationMasterAccountException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::OperationNotPermittedException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::OrganizationNotInAllFeaturesModeException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::OrganizationsNotInUseException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::S3BucketDoesNotExistException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::TagsLimitExceededException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::TrailAlreadyExistsException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::TrailNotProvidedException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::UnsupportedOperationException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::Unhandled(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl aws_http::request_id::RequestId for crate::operation::create_trail::CreateTrailError {
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for CreateTrailError {
    fn code(&self) -> std::option::Option<&str> {
        aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> std::option::Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl CreateTrailError {
    /// Creates the `CreateTrailError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self::Unhandled(
            aws_smithy_types::error::Unhandled::builder()
                .source(err)
                .build(),
        )
    }

    /// Creates the `CreateTrailError::Unhandled` variant from a `aws_smithy_types::error::ErrorMetadata`.
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
            Self::CloudTrailAccessNotEnabledException(e) => e.meta(),
            Self::CloudTrailInvalidClientTokenIdException(e) => e.meta(),
            Self::CloudWatchLogsDeliveryUnavailableException(e) => e.meta(),
            Self::ConflictException(e) => e.meta(),
            Self::InsufficientDependencyServiceAccessPermissionException(e) => e.meta(),
            Self::InsufficientEncryptionPolicyException(e) => e.meta(),
            Self::InsufficientS3BucketPolicyException(e) => e.meta(),
            Self::InsufficientSnsTopicPolicyException(e) => e.meta(),
            Self::InvalidCloudWatchLogsLogGroupArnException(e) => e.meta(),
            Self::InvalidCloudWatchLogsRoleArnException(e) => e.meta(),
            Self::InvalidKmsKeyIdException(e) => e.meta(),
            Self::InvalidParameterCombinationException(e) => e.meta(),
            Self::InvalidS3BucketNameException(e) => e.meta(),
            Self::InvalidS3PrefixException(e) => e.meta(),
            Self::InvalidSnsTopicNameException(e) => e.meta(),
            Self::InvalidTagParameterException(e) => e.meta(),
            Self::InvalidTrailNameException(e) => e.meta(),
            Self::KmsException(e) => e.meta(),
            Self::KmsKeyDisabledException(e) => e.meta(),
            Self::KmsKeyNotFoundException(e) => e.meta(),
            Self::MaximumNumberOfTrailsExceededException(e) => e.meta(),
            Self::NoManagementAccountSlrExistsException(e) => e.meta(),
            Self::NotOrganizationMasterAccountException(e) => e.meta(),
            Self::OperationNotPermittedException(e) => e.meta(),
            Self::OrganizationNotInAllFeaturesModeException(e) => e.meta(),
            Self::OrganizationsNotInUseException(e) => e.meta(),
            Self::S3BucketDoesNotExistException(e) => e.meta(),
            Self::TagsLimitExceededException(e) => e.meta(),
            Self::TrailAlreadyExistsException(e) => e.meta(),
            Self::TrailNotProvidedException(e) => e.meta(),
            Self::UnsupportedOperationException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `CreateTrailError::CloudTrailAccessNotEnabledException`.
    pub fn is_cloud_trail_access_not_enabled_exception(&self) -> bool {
        matches!(self, Self::CloudTrailAccessNotEnabledException(_))
    }
    /// Returns `true` if the error kind is `CreateTrailError::CloudTrailInvalidClientTokenIdException`.
    pub fn is_cloud_trail_invalid_client_token_id_exception(&self) -> bool {
        matches!(self, Self::CloudTrailInvalidClientTokenIdException(_))
    }
    /// Returns `true` if the error kind is `CreateTrailError::CloudWatchLogsDeliveryUnavailableException`.
    pub fn is_cloud_watch_logs_delivery_unavailable_exception(&self) -> bool {
        matches!(self, Self::CloudWatchLogsDeliveryUnavailableException(_))
    }
    /// Returns `true` if the error kind is `CreateTrailError::ConflictException`.
    pub fn is_conflict_exception(&self) -> bool {
        matches!(self, Self::ConflictException(_))
    }
    /// Returns `true` if the error kind is `CreateTrailError::InsufficientDependencyServiceAccessPermissionException`.
    pub fn is_insufficient_dependency_service_access_permission_exception(&self) -> bool {
        matches!(
            self,
            Self::InsufficientDependencyServiceAccessPermissionException(_)
        )
    }
    /// Returns `true` if the error kind is `CreateTrailError::InsufficientEncryptionPolicyException`.
    pub fn is_insufficient_encryption_policy_exception(&self) -> bool {
        matches!(self, Self::InsufficientEncryptionPolicyException(_))
    }
    /// Returns `true` if the error kind is `CreateTrailError::InsufficientS3BucketPolicyException`.
    pub fn is_insufficient_s3_bucket_policy_exception(&self) -> bool {
        matches!(self, Self::InsufficientS3BucketPolicyException(_))
    }
    /// Returns `true` if the error kind is `CreateTrailError::InsufficientSnsTopicPolicyException`.
    pub fn is_insufficient_sns_topic_policy_exception(&self) -> bool {
        matches!(self, Self::InsufficientSnsTopicPolicyException(_))
    }
    /// Returns `true` if the error kind is `CreateTrailError::InvalidCloudWatchLogsLogGroupArnException`.
    pub fn is_invalid_cloud_watch_logs_log_group_arn_exception(&self) -> bool {
        matches!(self, Self::InvalidCloudWatchLogsLogGroupArnException(_))
    }
    /// Returns `true` if the error kind is `CreateTrailError::InvalidCloudWatchLogsRoleArnException`.
    pub fn is_invalid_cloud_watch_logs_role_arn_exception(&self) -> bool {
        matches!(self, Self::InvalidCloudWatchLogsRoleArnException(_))
    }
    /// Returns `true` if the error kind is `CreateTrailError::InvalidKmsKeyIdException`.
    pub fn is_invalid_kms_key_id_exception(&self) -> bool {
        matches!(self, Self::InvalidKmsKeyIdException(_))
    }
    /// Returns `true` if the error kind is `CreateTrailError::InvalidParameterCombinationException`.
    pub fn is_invalid_parameter_combination_exception(&self) -> bool {
        matches!(self, Self::InvalidParameterCombinationException(_))
    }
    /// Returns `true` if the error kind is `CreateTrailError::InvalidS3BucketNameException`.
    pub fn is_invalid_s3_bucket_name_exception(&self) -> bool {
        matches!(self, Self::InvalidS3BucketNameException(_))
    }
    /// Returns `true` if the error kind is `CreateTrailError::InvalidS3PrefixException`.
    pub fn is_invalid_s3_prefix_exception(&self) -> bool {
        matches!(self, Self::InvalidS3PrefixException(_))
    }
    /// Returns `true` if the error kind is `CreateTrailError::InvalidSnsTopicNameException`.
    pub fn is_invalid_sns_topic_name_exception(&self) -> bool {
        matches!(self, Self::InvalidSnsTopicNameException(_))
    }
    /// Returns `true` if the error kind is `CreateTrailError::InvalidTagParameterException`.
    pub fn is_invalid_tag_parameter_exception(&self) -> bool {
        matches!(self, Self::InvalidTagParameterException(_))
    }
    /// Returns `true` if the error kind is `CreateTrailError::InvalidTrailNameException`.
    pub fn is_invalid_trail_name_exception(&self) -> bool {
        matches!(self, Self::InvalidTrailNameException(_))
    }
    /// Returns `true` if the error kind is `CreateTrailError::KmsException`.
    pub fn is_kms_exception(&self) -> bool {
        matches!(self, Self::KmsException(_))
    }
    /// Returns `true` if the error kind is `CreateTrailError::KmsKeyDisabledException`.
    pub fn is_kms_key_disabled_exception(&self) -> bool {
        matches!(self, Self::KmsKeyDisabledException(_))
    }
    /// Returns `true` if the error kind is `CreateTrailError::KmsKeyNotFoundException`.
    pub fn is_kms_key_not_found_exception(&self) -> bool {
        matches!(self, Self::KmsKeyNotFoundException(_))
    }
    /// Returns `true` if the error kind is `CreateTrailError::MaximumNumberOfTrailsExceededException`.
    pub fn is_maximum_number_of_trails_exceeded_exception(&self) -> bool {
        matches!(self, Self::MaximumNumberOfTrailsExceededException(_))
    }
    /// Returns `true` if the error kind is `CreateTrailError::NoManagementAccountSlrExistsException`.
    pub fn is_no_management_account_slr_exists_exception(&self) -> bool {
        matches!(self, Self::NoManagementAccountSlrExistsException(_))
    }
    /// Returns `true` if the error kind is `CreateTrailError::NotOrganizationMasterAccountException`.
    pub fn is_not_organization_master_account_exception(&self) -> bool {
        matches!(self, Self::NotOrganizationMasterAccountException(_))
    }
    /// Returns `true` if the error kind is `CreateTrailError::OperationNotPermittedException`.
    pub fn is_operation_not_permitted_exception(&self) -> bool {
        matches!(self, Self::OperationNotPermittedException(_))
    }
    /// Returns `true` if the error kind is `CreateTrailError::OrganizationNotInAllFeaturesModeException`.
    pub fn is_organization_not_in_all_features_mode_exception(&self) -> bool {
        matches!(self, Self::OrganizationNotInAllFeaturesModeException(_))
    }
    /// Returns `true` if the error kind is `CreateTrailError::OrganizationsNotInUseException`.
    pub fn is_organizations_not_in_use_exception(&self) -> bool {
        matches!(self, Self::OrganizationsNotInUseException(_))
    }
    /// Returns `true` if the error kind is `CreateTrailError::S3BucketDoesNotExistException`.
    pub fn is_s3_bucket_does_not_exist_exception(&self) -> bool {
        matches!(self, Self::S3BucketDoesNotExistException(_))
    }
    /// Returns `true` if the error kind is `CreateTrailError::TagsLimitExceededException`.
    pub fn is_tags_limit_exceeded_exception(&self) -> bool {
        matches!(self, Self::TagsLimitExceededException(_))
    }
    /// Returns `true` if the error kind is `CreateTrailError::TrailAlreadyExistsException`.
    pub fn is_trail_already_exists_exception(&self) -> bool {
        matches!(self, Self::TrailAlreadyExistsException(_))
    }
    /// Returns `true` if the error kind is `CreateTrailError::TrailNotProvidedException`.
    pub fn is_trail_not_provided_exception(&self) -> bool {
        matches!(self, Self::TrailNotProvidedException(_))
    }
    /// Returns `true` if the error kind is `CreateTrailError::UnsupportedOperationException`.
    pub fn is_unsupported_operation_exception(&self) -> bool {
        matches!(self, Self::UnsupportedOperationException(_))
    }
}
impl std::error::Error for CreateTrailError {
    fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::CloudTrailAccessNotEnabledException(_inner) => Some(_inner),
            Self::CloudTrailInvalidClientTokenIdException(_inner) => Some(_inner),
            Self::CloudWatchLogsDeliveryUnavailableException(_inner) => Some(_inner),
            Self::ConflictException(_inner) => Some(_inner),
            Self::InsufficientDependencyServiceAccessPermissionException(_inner) => Some(_inner),
            Self::InsufficientEncryptionPolicyException(_inner) => Some(_inner),
            Self::InsufficientS3BucketPolicyException(_inner) => Some(_inner),
            Self::InsufficientSnsTopicPolicyException(_inner) => Some(_inner),
            Self::InvalidCloudWatchLogsLogGroupArnException(_inner) => Some(_inner),
            Self::InvalidCloudWatchLogsRoleArnException(_inner) => Some(_inner),
            Self::InvalidKmsKeyIdException(_inner) => Some(_inner),
            Self::InvalidParameterCombinationException(_inner) => Some(_inner),
            Self::InvalidS3BucketNameException(_inner) => Some(_inner),
            Self::InvalidS3PrefixException(_inner) => Some(_inner),
            Self::InvalidSnsTopicNameException(_inner) => Some(_inner),
            Self::InvalidTagParameterException(_inner) => Some(_inner),
            Self::InvalidTrailNameException(_inner) => Some(_inner),
            Self::KmsException(_inner) => Some(_inner),
            Self::KmsKeyDisabledException(_inner) => Some(_inner),
            Self::KmsKeyNotFoundException(_inner) => Some(_inner),
            Self::MaximumNumberOfTrailsExceededException(_inner) => Some(_inner),
            Self::NoManagementAccountSlrExistsException(_inner) => Some(_inner),
            Self::NotOrganizationMasterAccountException(_inner) => Some(_inner),
            Self::OperationNotPermittedException(_inner) => Some(_inner),
            Self::OrganizationNotInAllFeaturesModeException(_inner) => Some(_inner),
            Self::OrganizationsNotInUseException(_inner) => Some(_inner),
            Self::S3BucketDoesNotExistException(_inner) => Some(_inner),
            Self::TagsLimitExceededException(_inner) => Some(_inner),
            Self::TrailAlreadyExistsException(_inner) => Some(_inner),
            Self::TrailNotProvidedException(_inner) => Some(_inner),
            Self::UnsupportedOperationException(_inner) => Some(_inner),
            Self::Unhandled(_inner) => Some(_inner),
        }
    }
}

pub use crate::operation::create_trail::_create_trail_output::CreateTrailOutput;

pub use crate::operation::create_trail::_create_trail_input::CreateTrailInput;

mod _create_trail_input;

mod _create_trail_output;

/// Builders
pub mod builders;
