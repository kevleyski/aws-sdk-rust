// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;

/// See [`GetRawMessageContentInput`](crate::input::GetRawMessageContentInput).
pub mod get_raw_message_content_input {
    
    /// A builder for [`GetRawMessageContentInput`](crate::input::GetRawMessageContentInput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug, )]
    pub struct Builder {
        pub(crate) message_id: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The identifier of the email message to retrieve.</p>
        pub fn message_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.message_id = Some(input.into());
            self
        }
        /// <p>The identifier of the email message to retrieve.</p>
        pub fn set_message_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message_id = input; self
        }
        /// Consumes the builder and constructs a [`GetRawMessageContentInput`](crate::input::GetRawMessageContentInput).
        pub fn build(self) -> Result<crate::input::GetRawMessageContentInput, aws_smithy_http::operation::BuildError> {
            Ok(
                crate::input::GetRawMessageContentInput {
                    message_id: self.message_id
                    ,
                }
            )
        }
    }
    
    
}
impl GetRawMessageContentInput {
    /// Consumes the builder and constructs an Operation<[`GetRawMessageContent`](crate::operation::GetRawMessageContent)>
    #[allow(unused_mut)]#[allow(clippy::let_and_return)]#[allow(clippy::needless_borrow)]pub async fn make_operation(&self, _config: &crate::config::Config) -> std::result::Result<aws_smithy_http::operation::Operation<crate::operation::GetRawMessageContent, aws_http::retry::AwsResponseRetryClassifier>, aws_smithy_http::operation::BuildError> {
        let mut request = {
            fn uri_base(_input: &crate::input::GetRawMessageContentInput, output: &mut String) -> Result<(), aws_smithy_http::operation::BuildError> {
                let input_1 = &_input.message_id;
                let input_1 = input_1.as_ref().ok_or(aws_smithy_http::operation::BuildError::MissingField { field: "message_id", details: "cannot be empty or unset" })?;
                let message_id = aws_smithy_http::label::fmt_string(input_1, aws_smithy_http::label::EncodingStrategy::Default);
                if message_id.is_empty() {
                                return Err(aws_smithy_http::operation::BuildError::MissingField { field: "message_id", details: "cannot be empty or unset" })
                            }
                write!(output, "/messages/{messageId}", messageId = message_id).expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]fn update_http_builder(
                            input: &crate::input::GetRawMessageContentInput,
                            builder: http::request::Builder
                        ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
                let mut uri = String::new();
                uri_base(input, &mut uri)?;
                Ok(builder.method("GET").uri(uri))
            }
            let mut builder = update_http_builder(&self, http::request::Builder::new())?;
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]let body = aws_smithy_http::body::SdkBody::from(
            ""
        );
        let request = request.body(body).expect("should be valid request");
        let mut request = aws_smithy_http::operation::Request::from_parts(request, properties);
        request.properties_mut().insert(aws_smithy_http::http_versions::DEFAULT_HTTP_VERSION_LIST.clone());
        let mut user_agent = aws_http::user_agent::AwsUserAgent::new_from_environment(
                            aws_types::os_shim_internal::Env::real(),
                            crate::API_METADATA.clone(),
                        );
                        if let Some(app_name) = _config.app_name() {
                            user_agent = user_agent.with_app_name(app_name.clone());
                        }
                        request.properties_mut().insert(user_agent);
        let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
        request.properties_mut().insert(signing_config);
                            request.properties_mut().insert(aws_types::SigningService::from_static(_config.signing_service()));
                            if let Some(region) = &_config.region {
                                request.properties_mut().insert(aws_types::region::SigningRegion::from(region.clone()));
                            }
        let endpoint_params = aws_endpoint::Params::new(_config.region.clone());
                            request.properties_mut()
                                .insert::<aws_smithy_http::endpoint::Result>(_config
                                    .endpoint_resolver
                                    .resolve_endpoint(&endpoint_params));
        if let Some(region) = &_config.region {
                                request.properties_mut().insert(region.clone());
                            }
        aws_http::auth::set_provider(&mut request.properties_mut(), _config.credentials_provider.clone());
        let op = aws_smithy_http::operation::Operation::new(request, crate::operation::GetRawMessageContent::new())
                            .with_metadata(aws_smithy_http::operation::Metadata::new("GetRawMessageContent", "workmailmessageflow"));
        let op = op.with_retry_classifier(aws_http::retry::AwsResponseRetryClassifier::new());
        Ok(op)
    }
    /// Creates a new builder-style object to manufacture [`GetRawMessageContentInput`](crate::input::GetRawMessageContentInput).
    pub fn builder() -> crate::input::get_raw_message_content_input::Builder {
        crate::input::get_raw_message_content_input::Builder::default()
    }
}

/// See [`PutRawMessageContentInput`](crate::input::PutRawMessageContentInput).
pub mod put_raw_message_content_input {
    
    /// A builder for [`PutRawMessageContentInput`](crate::input::PutRawMessageContentInput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug, )]
    pub struct Builder {
        pub(crate) message_id: std::option::Option<std::string::String>,
        pub(crate) content: std::option::Option<crate::model::RawMessageContent>,
    }
    impl Builder {
        /// <p>The identifier of the email message being updated.</p>
        pub fn message_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.message_id = Some(input.into());
            self
        }
        /// <p>The identifier of the email message being updated.</p>
        pub fn set_message_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message_id = input; self
        }
        /// <p>Describes the raw message content of the updated email message.</p>
        pub fn content(mut self, input: crate::model::RawMessageContent) -> Self {
            self.content = Some(input);
            self
        }
        /// <p>Describes the raw message content of the updated email message.</p>
        pub fn set_content(mut self, input: std::option::Option<crate::model::RawMessageContent>) -> Self {
            self.content = input; self
        }
        /// Consumes the builder and constructs a [`PutRawMessageContentInput`](crate::input::PutRawMessageContentInput).
        pub fn build(self) -> Result<crate::input::PutRawMessageContentInput, aws_smithy_http::operation::BuildError> {
            Ok(
                crate::input::PutRawMessageContentInput {
                    message_id: self.message_id
                    ,
                    content: self.content
                    ,
                }
            )
        }
    }
    
    
}
impl PutRawMessageContentInput {
    /// Consumes the builder and constructs an Operation<[`PutRawMessageContent`](crate::operation::PutRawMessageContent)>
    #[allow(unused_mut)]#[allow(clippy::let_and_return)]#[allow(clippy::needless_borrow)]pub async fn make_operation(&self, _config: &crate::config::Config) -> std::result::Result<aws_smithy_http::operation::Operation<crate::operation::PutRawMessageContent, aws_http::retry::AwsResponseRetryClassifier>, aws_smithy_http::operation::BuildError> {
        let mut request = {
            fn uri_base(_input: &crate::input::PutRawMessageContentInput, output: &mut String) -> Result<(), aws_smithy_http::operation::BuildError> {
                let input_2 = &_input.message_id;
                let input_2 = input_2.as_ref().ok_or(aws_smithy_http::operation::BuildError::MissingField { field: "message_id", details: "cannot be empty or unset" })?;
                let message_id = aws_smithy_http::label::fmt_string(input_2, aws_smithy_http::label::EncodingStrategy::Default);
                if message_id.is_empty() {
                                return Err(aws_smithy_http::operation::BuildError::MissingField { field: "message_id", details: "cannot be empty or unset" })
                            }
                write!(output, "/messages/{messageId}", messageId = message_id).expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]fn update_http_builder(
                            input: &crate::input::PutRawMessageContentInput,
                            builder: http::request::Builder
                        ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
                let mut uri = String::new();
                uri_base(input, &mut uri)?;
                Ok(builder.method("POST").uri(uri))
            }
            let mut builder = update_http_builder(&self, http::request::Builder::new())?;
            builder = aws_smithy_http::header::set_request_header_if_absent(builder, http::header::CONTENT_TYPE, "application/json");
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]let body = aws_smithy_http::body::SdkBody::from(
            crate::operation_ser::serialize_operation_crate_operation_put_raw_message_content(&self)?
        );
        if let Some(content_length) = body.content_length() {
                                request = aws_smithy_http::header::set_request_header_if_absent(request, http::header::CONTENT_LENGTH, content_length);
                            }
        let request = request.body(body).expect("should be valid request");
        let mut request = aws_smithy_http::operation::Request::from_parts(request, properties);
        request.properties_mut().insert(aws_smithy_http::http_versions::DEFAULT_HTTP_VERSION_LIST.clone());
        let mut user_agent = aws_http::user_agent::AwsUserAgent::new_from_environment(
                            aws_types::os_shim_internal::Env::real(),
                            crate::API_METADATA.clone(),
                        );
                        if let Some(app_name) = _config.app_name() {
                            user_agent = user_agent.with_app_name(app_name.clone());
                        }
                        request.properties_mut().insert(user_agent);
        let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
        request.properties_mut().insert(signing_config);
                            request.properties_mut().insert(aws_types::SigningService::from_static(_config.signing_service()));
                            if let Some(region) = &_config.region {
                                request.properties_mut().insert(aws_types::region::SigningRegion::from(region.clone()));
                            }
        let endpoint_params = aws_endpoint::Params::new(_config.region.clone());
                            request.properties_mut()
                                .insert::<aws_smithy_http::endpoint::Result>(_config
                                    .endpoint_resolver
                                    .resolve_endpoint(&endpoint_params));
        if let Some(region) = &_config.region {
                                request.properties_mut().insert(region.clone());
                            }
        aws_http::auth::set_provider(&mut request.properties_mut(), _config.credentials_provider.clone());
        let op = aws_smithy_http::operation::Operation::new(request, crate::operation::PutRawMessageContent::new())
                            .with_metadata(aws_smithy_http::operation::Metadata::new("PutRawMessageContent", "workmailmessageflow"));
        let op = op.with_retry_classifier(aws_http::retry::AwsResponseRetryClassifier::new());
        Ok(op)
    }
    /// Creates a new builder-style object to manufacture [`PutRawMessageContentInput`](crate::input::PutRawMessageContentInput).
    pub fn builder() -> crate::input::put_raw_message_content_input::Builder {
        crate::input::put_raw_message_content_input::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]#[derive(std::clone::Clone, std::cmp::PartialEq, )]
pub struct PutRawMessageContentInput  {
    /// <p>The identifier of the email message being updated.</p>
    #[doc(hidden)]pub message_id: std::option::Option<std::string::String>,
    /// <p>Describes the raw message content of the updated email message.</p>
    #[doc(hidden)]pub content: std::option::Option<crate::model::RawMessageContent>,
}
impl PutRawMessageContentInput {
    /// <p>The identifier of the email message being updated.</p>
    pub fn message_id(&self) -> std::option::Option<& str> {
        self.message_id.as_deref()
    }
    /// <p>Describes the raw message content of the updated email message.</p>
    pub fn content(&self) -> std::option::Option<& crate::model::RawMessageContent> {
        self.content.as_ref()
    }
}
impl  std::fmt::Debug for PutRawMessageContentInput  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("PutRawMessageContentInput");
        formatter.field("message_id", &self.message_id);
        formatter.field("content", &self.content);
        formatter.finish()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]#[derive(std::clone::Clone, std::cmp::PartialEq, )]
pub struct GetRawMessageContentInput  {
    /// <p>The identifier of the email message to retrieve.</p>
    #[doc(hidden)]pub message_id: std::option::Option<std::string::String>,
}
impl GetRawMessageContentInput {
    /// <p>The identifier of the email message to retrieve.</p>
    pub fn message_id(&self) -> std::option::Option<& str> {
        self.message_id.as_deref()
    }
}
impl  std::fmt::Debug for GetRawMessageContentInput  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetRawMessageContentInput");
        formatter.field("message_id", &self.message_id);
        formatter.finish()
    }
}

