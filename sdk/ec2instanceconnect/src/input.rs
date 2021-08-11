// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;
/// See [`SendSerialConsoleSshPublicKeyInput`](crate::input::SendSerialConsoleSshPublicKeyInput)
pub mod send_serial_console_ssh_public_key_input {
    /// A builder for [`SendSerialConsoleSshPublicKeyInput`](crate::input::SendSerialConsoleSshPublicKeyInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) instance_id: std::option::Option<std::string::String>,
        pub(crate) serial_port: std::option::Option<i32>,
        pub(crate) ssh_public_key: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The ID of the EC2 instance.</p>
        pub fn instance_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.instance_id = Some(input.into());
            self
        }
        pub fn set_instance_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.instance_id = input;
            self
        }
        /// <p>The serial port of the EC2 instance. Currently only port 0 is supported.</p>
        /// <p>Default: 0</p>
        pub fn serial_port(mut self, input: i32) -> Self {
            self.serial_port = Some(input);
            self
        }
        pub fn set_serial_port(mut self, input: std::option::Option<i32>) -> Self {
            self.serial_port = input;
            self
        }
        /// <p>The public key material. To use the public key, you must have the matching private
        /// key. For information about the supported key formats and lengths, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-key-pairs.html#how-to-generate-your-own-key-and-import-it-to-aws">Requirements for key pairs</a> in the <i>Amazon EC2 User
        /// Guide</i>.</p>
        pub fn ssh_public_key(mut self, input: impl Into<std::string::String>) -> Self {
            self.ssh_public_key = Some(input.into());
            self
        }
        pub fn set_ssh_public_key(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.ssh_public_key = input;
            self
        }
        /// Consumes the builder and constructs a [`SendSerialConsoleSshPublicKeyInput`](crate::input::SendSerialConsoleSshPublicKeyInput)
        pub fn build(
            self,
        ) -> std::result::Result<
            crate::input::SendSerialConsoleSshPublicKeyInput,
            smithy_http::operation::BuildError,
        > {
            Ok(crate::input::SendSerialConsoleSshPublicKeyInput {
                instance_id: self.instance_id,
                serial_port: self.serial_port.unwrap_or_default(),
                ssh_public_key: self.ssh_public_key,
            })
        }
    }
}
#[doc(hidden)]
pub type SendSerialConsoleSSHPublicKeyInputOperationOutputAlias =
    crate::operation::SendSerialConsoleSSHPublicKey;
#[doc(hidden)]
pub type SendSerialConsoleSSHPublicKeyInputOperationRetryAlias = aws_http::AwsErrorRetryPolicy;
impl SendSerialConsoleSshPublicKeyInput {
    /// Consumes the builder and constructs an Operation<[`SendSerialConsoleSSHPublicKey`](crate::operation::SendSerialConsoleSSHPublicKey)>
    #[allow(clippy::let_and_return)]
    pub fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        smithy_http::operation::Operation<
            crate::operation::SendSerialConsoleSSHPublicKey,
            aws_http::AwsErrorRetryPolicy,
        >,
        smithy_http::operation::BuildError,
    > {
        Ok({
            let request = self.request_builder_base()?;
            let body =
                crate::operation_ser::serialize_operation_send_serial_console_ssh_public_key(&self)
                    .map_err(|err| {
                        smithy_http::operation::BuildError::SerializationError(err.into())
                    })?;
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
                crate::operation::SendSerialConsoleSSHPublicKey::new(),
            )
            .with_metadata(smithy_http::operation::Metadata::new(
                "SendSerialConsoleSSHPublicKey",
                "ec2instanceconnect",
            ));
            let op = op.with_retry_policy(aws_http::AwsErrorRetryPolicy::new());
            op
        })
    }
    fn uri_base(&self, output: &mut String) -> Result<(), smithy_http::operation::BuildError> {
        write!(output, "/").expect("formatting should succeed");
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
        builder = smithy_http::header::set_header_if_absent(
            builder,
            "content-type",
            "application/x-amz-json-1.1",
        );
        builder = smithy_http::header::set_header_if_absent(
            builder,
            "x-amz-target",
            "AWSEC2InstanceConnectService.SendSerialConsoleSSHPublicKey",
        );
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
    /// Creates a new builder-style object to manufacture [`SendSerialConsoleSshPublicKeyInput`](crate::input::SendSerialConsoleSshPublicKeyInput)
    pub fn builder() -> crate::input::send_serial_console_ssh_public_key_input::Builder {
        crate::input::send_serial_console_ssh_public_key_input::Builder::default()
    }
}

/// See [`SendSshPublicKeyInput`](crate::input::SendSshPublicKeyInput)
pub mod send_ssh_public_key_input {
    /// A builder for [`SendSshPublicKeyInput`](crate::input::SendSshPublicKeyInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) instance_id: std::option::Option<std::string::String>,
        pub(crate) instance_os_user: std::option::Option<std::string::String>,
        pub(crate) ssh_public_key: std::option::Option<std::string::String>,
        pub(crate) availability_zone: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The ID of the EC2 instance.</p>
        pub fn instance_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.instance_id = Some(input.into());
            self
        }
        pub fn set_instance_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.instance_id = input;
            self
        }
        /// <p>The OS user on the EC2 instance for whom the key can be used to authenticate.</p>
        pub fn instance_os_user(mut self, input: impl Into<std::string::String>) -> Self {
            self.instance_os_user = Some(input.into());
            self
        }
        pub fn set_instance_os_user(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.instance_os_user = input;
            self
        }
        /// <p>The public key material. To use the public key, you must have the matching private key.</p>
        pub fn ssh_public_key(mut self, input: impl Into<std::string::String>) -> Self {
            self.ssh_public_key = Some(input.into());
            self
        }
        pub fn set_ssh_public_key(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.ssh_public_key = input;
            self
        }
        /// <p>The Availability Zone in which the EC2 instance was launched.</p>
        pub fn availability_zone(mut self, input: impl Into<std::string::String>) -> Self {
            self.availability_zone = Some(input.into());
            self
        }
        pub fn set_availability_zone(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.availability_zone = input;
            self
        }
        /// Consumes the builder and constructs a [`SendSshPublicKeyInput`](crate::input::SendSshPublicKeyInput)
        pub fn build(
            self,
        ) -> std::result::Result<
            crate::input::SendSshPublicKeyInput,
            smithy_http::operation::BuildError,
        > {
            Ok(crate::input::SendSshPublicKeyInput {
                instance_id: self.instance_id,
                instance_os_user: self.instance_os_user,
                ssh_public_key: self.ssh_public_key,
                availability_zone: self.availability_zone,
            })
        }
    }
}
#[doc(hidden)]
pub type SendSSHPublicKeyInputOperationOutputAlias = crate::operation::SendSSHPublicKey;
#[doc(hidden)]
pub type SendSSHPublicKeyInputOperationRetryAlias = aws_http::AwsErrorRetryPolicy;
impl SendSshPublicKeyInput {
    /// Consumes the builder and constructs an Operation<[`SendSSHPublicKey`](crate::operation::SendSSHPublicKey)>
    #[allow(clippy::let_and_return)]
    pub fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        smithy_http::operation::Operation<
            crate::operation::SendSSHPublicKey,
            aws_http::AwsErrorRetryPolicy,
        >,
        smithy_http::operation::BuildError,
    > {
        Ok({
            let request = self.request_builder_base()?;
            let body = crate::operation_ser::serialize_operation_send_ssh_public_key(&self)
                .map_err(|err| {
                    smithy_http::operation::BuildError::SerializationError(err.into())
                })?;
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
                crate::operation::SendSSHPublicKey::new(),
            )
            .with_metadata(smithy_http::operation::Metadata::new(
                "SendSSHPublicKey",
                "ec2instanceconnect",
            ));
            let op = op.with_retry_policy(aws_http::AwsErrorRetryPolicy::new());
            op
        })
    }
    fn uri_base(&self, output: &mut String) -> Result<(), smithy_http::operation::BuildError> {
        write!(output, "/").expect("formatting should succeed");
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
        builder = smithy_http::header::set_header_if_absent(
            builder,
            "content-type",
            "application/x-amz-json-1.1",
        );
        builder = smithy_http::header::set_header_if_absent(
            builder,
            "x-amz-target",
            "AWSEC2InstanceConnectService.SendSSHPublicKey",
        );
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
    /// Creates a new builder-style object to manufacture [`SendSshPublicKeyInput`](crate::input::SendSshPublicKeyInput)
    pub fn builder() -> crate::input::send_ssh_public_key_input::Builder {
        crate::input::send_ssh_public_key_input::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct SendSshPublicKeyInput {
    /// <p>The ID of the EC2 instance.</p>
    pub instance_id: std::option::Option<std::string::String>,
    /// <p>The OS user on the EC2 instance for whom the key can be used to authenticate.</p>
    pub instance_os_user: std::option::Option<std::string::String>,
    /// <p>The public key material. To use the public key, you must have the matching private key.</p>
    pub ssh_public_key: std::option::Option<std::string::String>,
    /// <p>The Availability Zone in which the EC2 instance was launched.</p>
    pub availability_zone: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for SendSshPublicKeyInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("SendSshPublicKeyInput");
        formatter.field("instance_id", &self.instance_id);
        formatter.field("instance_os_user", &self.instance_os_user);
        formatter.field("ssh_public_key", &self.ssh_public_key);
        formatter.field("availability_zone", &self.availability_zone);
        formatter.finish()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct SendSerialConsoleSshPublicKeyInput {
    /// <p>The ID of the EC2 instance.</p>
    pub instance_id: std::option::Option<std::string::String>,
    /// <p>The serial port of the EC2 instance. Currently only port 0 is supported.</p>
    /// <p>Default: 0</p>
    pub serial_port: i32,
    /// <p>The public key material. To use the public key, you must have the matching private
    /// key. For information about the supported key formats and lengths, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-key-pairs.html#how-to-generate-your-own-key-and-import-it-to-aws">Requirements for key pairs</a> in the <i>Amazon EC2 User
    /// Guide</i>.</p>
    pub ssh_public_key: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for SendSerialConsoleSshPublicKeyInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("SendSerialConsoleSshPublicKeyInput");
        formatter.field("instance_id", &self.instance_id);
        formatter.field("serial_port", &self.serial_port);
        formatter.field("ssh_public_key", &self.ssh_public_key);
        formatter.finish()
    }
}
