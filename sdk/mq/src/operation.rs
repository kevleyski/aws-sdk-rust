// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>Creates a broker. Note: This API is asynchronous.</p> <p>To create a broker, you must either use the AmazonMQFullAccess IAM policy or include the following EC2 permissions in your IAM policy.</p> <ul><li><p>ec2:CreateNetworkInterface</p> <p>This permission is required to allow Amazon MQ to create an elastic network interface (ENI) on behalf of your account.</p></li> <li><p>ec2:CreateNetworkInterfacePermission</p> <p>This permission is required to attach the ENI to the broker instance.</p></li> <li><p>ec2:DeleteNetworkInterface</p></li> <li><p>ec2:DeleteNetworkInterfacePermission</p></li> <li><p>ec2:DetachNetworkInterface</p></li> <li><p>ec2:DescribeInternetGateways</p></li> <li><p>ec2:DescribeNetworkInterfaces</p></li> <li><p>ec2:DescribeNetworkInterfacePermissions</p></li> <li><p>ec2:DescribeRouteTables</p></li> <li><p>ec2:DescribeSecurityGroups</p></li> <li><p>ec2:DescribeSubnets</p></li> <li><p>ec2:DescribeVpcs</p></li></ul> <p>For more information, see <a href="https://docs.aws.amazon.com//amazon-mq/latest/developer-guide/amazon-mq-setting-up.html#create-iam-user">Create an IAM User and Get Your AWS Credentials</a> and <a href="https://docs.aws.amazon.com/amazon-mq/latest/developer-guide/connecting-to-amazon-mq.html#never-modify-delete-elastic-network-interface">Never Modify or Delete the Amazon MQ Elastic Network Interface</a> in the <i>Amazon MQ Developer Guide</i>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateBroker {
    _private: (),
}
impl CreateBroker {
    /// Creates a new builder-style object to manufacture [`CreateBrokerInput`](crate::input::CreateBrokerInput)
    pub fn builder() -> crate::input::create_broker_input::Builder {
        crate::input::create_broker_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateBroker {
    type Output =
        std::result::Result<crate::output::CreateBrokerOutput, crate::error::CreateBrokerError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_broker_error(response)
        } else {
            crate::operation_deser::parse_create_broker_response(response)
        }
    }
}

/// <p>Creates a new configuration for the specified configuration name. Amazon MQ uses the default configuration (the engine type and version).</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateConfiguration {
    _private: (),
}
impl CreateConfiguration {
    /// Creates a new builder-style object to manufacture [`CreateConfigurationInput`](crate::input::CreateConfigurationInput)
    pub fn builder() -> crate::input::create_configuration_input::Builder {
        crate::input::create_configuration_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateConfiguration {
    type Output = std::result::Result<
        crate::output::CreateConfigurationOutput,
        crate::error::CreateConfigurationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_configuration_error(response)
        } else {
            crate::operation_deser::parse_create_configuration_response(response)
        }
    }
}

/// <p>Add a tag to a resource.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateTags {
    _private: (),
}
impl CreateTags {
    /// Creates a new builder-style object to manufacture [`CreateTagsInput`](crate::input::CreateTagsInput)
    pub fn builder() -> crate::input::create_tags_input::Builder {
        crate::input::create_tags_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateTags {
    type Output =
        std::result::Result<crate::output::CreateTagsOutput, crate::error::CreateTagsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 204 {
            crate::operation_deser::parse_create_tags_error(response)
        } else {
            crate::operation_deser::parse_create_tags_response(response)
        }
    }
}

/// <p>Creates an ActiveMQ user.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateUser {
    _private: (),
}
impl CreateUser {
    /// Creates a new builder-style object to manufacture [`CreateUserInput`](crate::input::CreateUserInput)
    pub fn builder() -> crate::input::create_user_input::Builder {
        crate::input::create_user_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateUser {
    type Output =
        std::result::Result<crate::output::CreateUserOutput, crate::error::CreateUserError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_user_error(response)
        } else {
            crate::operation_deser::parse_create_user_response(response)
        }
    }
}

/// <p>Deletes a broker. Note: This API is asynchronous.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteBroker {
    _private: (),
}
impl DeleteBroker {
    /// Creates a new builder-style object to manufacture [`DeleteBrokerInput`](crate::input::DeleteBrokerInput)
    pub fn builder() -> crate::input::delete_broker_input::Builder {
        crate::input::delete_broker_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteBroker {
    type Output =
        std::result::Result<crate::output::DeleteBrokerOutput, crate::error::DeleteBrokerError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_broker_error(response)
        } else {
            crate::operation_deser::parse_delete_broker_response(response)
        }
    }
}

/// <p>Removes a tag from a resource.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteTags {
    _private: (),
}
impl DeleteTags {
    /// Creates a new builder-style object to manufacture [`DeleteTagsInput`](crate::input::DeleteTagsInput)
    pub fn builder() -> crate::input::delete_tags_input::Builder {
        crate::input::delete_tags_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteTags {
    type Output =
        std::result::Result<crate::output::DeleteTagsOutput, crate::error::DeleteTagsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 204 {
            crate::operation_deser::parse_delete_tags_error(response)
        } else {
            crate::operation_deser::parse_delete_tags_response(response)
        }
    }
}

/// <p>Deletes an ActiveMQ user.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteUser {
    _private: (),
}
impl DeleteUser {
    /// Creates a new builder-style object to manufacture [`DeleteUserInput`](crate::input::DeleteUserInput)
    pub fn builder() -> crate::input::delete_user_input::Builder {
        crate::input::delete_user_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteUser {
    type Output =
        std::result::Result<crate::output::DeleteUserOutput, crate::error::DeleteUserError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_user_error(response)
        } else {
            crate::operation_deser::parse_delete_user_response(response)
        }
    }
}

/// <p>Returns information about the specified broker.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeBroker {
    _private: (),
}
impl DescribeBroker {
    /// Creates a new builder-style object to manufacture [`DescribeBrokerInput`](crate::input::DescribeBrokerInput)
    pub fn builder() -> crate::input::describe_broker_input::Builder {
        crate::input::describe_broker_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeBroker {
    type Output =
        std::result::Result<crate::output::DescribeBrokerOutput, crate::error::DescribeBrokerError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_broker_error(response)
        } else {
            crate::operation_deser::parse_describe_broker_response(response)
        }
    }
}

/// <p>Describe available engine types and versions.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeBrokerEngineTypes {
    _private: (),
}
impl DescribeBrokerEngineTypes {
    /// Creates a new builder-style object to manufacture [`DescribeBrokerEngineTypesInput`](crate::input::DescribeBrokerEngineTypesInput)
    pub fn builder() -> crate::input::describe_broker_engine_types_input::Builder {
        crate::input::describe_broker_engine_types_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeBrokerEngineTypes {
    type Output = std::result::Result<
        crate::output::DescribeBrokerEngineTypesOutput,
        crate::error::DescribeBrokerEngineTypesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_broker_engine_types_error(response)
        } else {
            crate::operation_deser::parse_describe_broker_engine_types_response(response)
        }
    }
}

/// <p>Describe available broker instance options.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeBrokerInstanceOptions {
    _private: (),
}
impl DescribeBrokerInstanceOptions {
    /// Creates a new builder-style object to manufacture [`DescribeBrokerInstanceOptionsInput`](crate::input::DescribeBrokerInstanceOptionsInput)
    pub fn builder() -> crate::input::describe_broker_instance_options_input::Builder {
        crate::input::describe_broker_instance_options_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeBrokerInstanceOptions {
    type Output = std::result::Result<
        crate::output::DescribeBrokerInstanceOptionsOutput,
        crate::error::DescribeBrokerInstanceOptionsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_broker_instance_options_error(response)
        } else {
            crate::operation_deser::parse_describe_broker_instance_options_response(response)
        }
    }
}

/// <p>Returns information about the specified configuration.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeConfiguration {
    _private: (),
}
impl DescribeConfiguration {
    /// Creates a new builder-style object to manufacture [`DescribeConfigurationInput`](crate::input::DescribeConfigurationInput)
    pub fn builder() -> crate::input::describe_configuration_input::Builder {
        crate::input::describe_configuration_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeConfiguration {
    type Output = std::result::Result<
        crate::output::DescribeConfigurationOutput,
        crate::error::DescribeConfigurationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_configuration_error(response)
        } else {
            crate::operation_deser::parse_describe_configuration_response(response)
        }
    }
}

/// <p>Returns the specified configuration revision for the specified configuration.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeConfigurationRevision {
    _private: (),
}
impl DescribeConfigurationRevision {
    /// Creates a new builder-style object to manufacture [`DescribeConfigurationRevisionInput`](crate::input::DescribeConfigurationRevisionInput)
    pub fn builder() -> crate::input::describe_configuration_revision_input::Builder {
        crate::input::describe_configuration_revision_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeConfigurationRevision {
    type Output = std::result::Result<
        crate::output::DescribeConfigurationRevisionOutput,
        crate::error::DescribeConfigurationRevisionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_configuration_revision_error(response)
        } else {
            crate::operation_deser::parse_describe_configuration_revision_response(response)
        }
    }
}

/// <p>Returns information about an ActiveMQ user.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeUser {
    _private: (),
}
impl DescribeUser {
    /// Creates a new builder-style object to manufacture [`DescribeUserInput`](crate::input::DescribeUserInput)
    pub fn builder() -> crate::input::describe_user_input::Builder {
        crate::input::describe_user_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeUser {
    type Output =
        std::result::Result<crate::output::DescribeUserOutput, crate::error::DescribeUserError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_user_error(response)
        } else {
            crate::operation_deser::parse_describe_user_response(response)
        }
    }
}

/// <p>Returns a list of all brokers.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListBrokers {
    _private: (),
}
impl ListBrokers {
    /// Creates a new builder-style object to manufacture [`ListBrokersInput`](crate::input::ListBrokersInput)
    pub fn builder() -> crate::input::list_brokers_input::Builder {
        crate::input::list_brokers_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListBrokers {
    type Output =
        std::result::Result<crate::output::ListBrokersOutput, crate::error::ListBrokersError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_brokers_error(response)
        } else {
            crate::operation_deser::parse_list_brokers_response(response)
        }
    }
}

/// <p>Returns a list of all revisions for the specified configuration.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListConfigurationRevisions {
    _private: (),
}
impl ListConfigurationRevisions {
    /// Creates a new builder-style object to manufacture [`ListConfigurationRevisionsInput`](crate::input::ListConfigurationRevisionsInput)
    pub fn builder() -> crate::input::list_configuration_revisions_input::Builder {
        crate::input::list_configuration_revisions_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListConfigurationRevisions {
    type Output = std::result::Result<
        crate::output::ListConfigurationRevisionsOutput,
        crate::error::ListConfigurationRevisionsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_configuration_revisions_error(response)
        } else {
            crate::operation_deser::parse_list_configuration_revisions_response(response)
        }
    }
}

/// <p>Returns a list of all configurations.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListConfigurations {
    _private: (),
}
impl ListConfigurations {
    /// Creates a new builder-style object to manufacture [`ListConfigurationsInput`](crate::input::ListConfigurationsInput)
    pub fn builder() -> crate::input::list_configurations_input::Builder {
        crate::input::list_configurations_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListConfigurations {
    type Output = std::result::Result<
        crate::output::ListConfigurationsOutput,
        crate::error::ListConfigurationsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_configurations_error(response)
        } else {
            crate::operation_deser::parse_list_configurations_response(response)
        }
    }
}

/// <p>Lists tags for a resource.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListTags {
    _private: (),
}
impl ListTags {
    /// Creates a new builder-style object to manufacture [`ListTagsInput`](crate::input::ListTagsInput)
    pub fn builder() -> crate::input::list_tags_input::Builder {
        crate::input::list_tags_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListTags {
    type Output = std::result::Result<crate::output::ListTagsOutput, crate::error::ListTagsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_tags_error(response)
        } else {
            crate::operation_deser::parse_list_tags_response(response)
        }
    }
}

/// <p>Returns a list of all ActiveMQ users.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListUsers {
    _private: (),
}
impl ListUsers {
    /// Creates a new builder-style object to manufacture [`ListUsersInput`](crate::input::ListUsersInput)
    pub fn builder() -> crate::input::list_users_input::Builder {
        crate::input::list_users_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListUsers {
    type Output = std::result::Result<crate::output::ListUsersOutput, crate::error::ListUsersError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_users_error(response)
        } else {
            crate::operation_deser::parse_list_users_response(response)
        }
    }
}

/// <p>Reboots a broker. Note: This API is asynchronous.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct RebootBroker {
    _private: (),
}
impl RebootBroker {
    /// Creates a new builder-style object to manufacture [`RebootBrokerInput`](crate::input::RebootBrokerInput)
    pub fn builder() -> crate::input::reboot_broker_input::Builder {
        crate::input::reboot_broker_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for RebootBroker {
    type Output =
        std::result::Result<crate::output::RebootBrokerOutput, crate::error::RebootBrokerError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_reboot_broker_error(response)
        } else {
            crate::operation_deser::parse_reboot_broker_response(response)
        }
    }
}

/// <p>Adds a pending configuration change to a broker.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateBroker {
    _private: (),
}
impl UpdateBroker {
    /// Creates a new builder-style object to manufacture [`UpdateBrokerInput`](crate::input::UpdateBrokerInput)
    pub fn builder() -> crate::input::update_broker_input::Builder {
        crate::input::update_broker_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdateBroker {
    type Output =
        std::result::Result<crate::output::UpdateBrokerOutput, crate::error::UpdateBrokerError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_broker_error(response)
        } else {
            crate::operation_deser::parse_update_broker_response(response)
        }
    }
}

/// <p>Updates the specified configuration.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateConfiguration {
    _private: (),
}
impl UpdateConfiguration {
    /// Creates a new builder-style object to manufacture [`UpdateConfigurationInput`](crate::input::UpdateConfigurationInput)
    pub fn builder() -> crate::input::update_configuration_input::Builder {
        crate::input::update_configuration_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdateConfiguration {
    type Output = std::result::Result<
        crate::output::UpdateConfigurationOutput,
        crate::error::UpdateConfigurationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_configuration_error(response)
        } else {
            crate::operation_deser::parse_update_configuration_response(response)
        }
    }
}

/// <p>Updates the information for an ActiveMQ user.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateUser {
    _private: (),
}
impl UpdateUser {
    /// Creates a new builder-style object to manufacture [`UpdateUserInput`](crate::input::UpdateUserInput)
    pub fn builder() -> crate::input::update_user_input::Builder {
        crate::input::update_user_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdateUser {
    type Output =
        std::result::Result<crate::output::UpdateUserOutput, crate::error::UpdateUserError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_user_error(response)
        } else {
            crate::operation_deser::parse_update_user_response(response)
        }
    }
}
