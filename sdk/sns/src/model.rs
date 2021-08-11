// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>The list of tags to be added to the specified topic.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct Tag {
    /// <p>The required key portion of the tag.</p>
    pub key: std::option::Option<std::string::String>,
    /// <p>The optional value portion of the tag.</p>
    pub value: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("Tag");
        formatter.field("key", &self.key);
        formatter.field("value", &self.value);
        formatter.finish()
    }
}
/// See [`Tag`](crate::model::Tag)
pub mod tag {
    /// A builder for [`Tag`](crate::model::Tag)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) key: std::option::Option<std::string::String>,
        pub(crate) value: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The required key portion of the tag.</p>
        pub fn key(mut self, input: impl Into<std::string::String>) -> Self {
            self.key = Some(input.into());
            self
        }
        pub fn set_key(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.key = input;
            self
        }
        /// <p>The optional value portion of the tag.</p>
        pub fn value(mut self, input: impl Into<std::string::String>) -> Self {
            self.value = Some(input.into());
            self
        }
        pub fn set_value(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.value = input;
            self
        }
        /// Consumes the builder and constructs a [`Tag`](crate::model::Tag)
        pub fn build(self) -> crate::model::Tag {
            crate::model::Tag {
                key: self.key,
                value: self.value,
            }
        }
    }
}
impl Tag {
    /// Creates a new builder-style object to manufacture [`Tag`](crate::model::Tag)
    pub fn builder() -> crate::model::tag::Builder {
        crate::model::tag::Builder::default()
    }
}

/// <p>The user-specified message attribute value. For string data types, the value attribute
/// has the same restrictions on the content as the message body. For more information, see
/// <a href="https://docs.aws.amazon.com/sns/latest/api/API_Publish.html">Publish</a>.</p>
/// <p>Name, type, and value must not be empty or null. In addition, the message body should
/// not be empty or null. All parts of the message attribute, including name, type, and
/// value, are included in the message size restriction, which is currently 256 KB (262,144
/// bytes). For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/SNSMessageAttributes.html">Amazon SNS message attributes</a> and
/// <a href="https://docs.aws.amazon.com/sns/latest/dg/sms_publish-to-phone.html">Publishing
/// to a mobile phone</a> in the <i>Amazon SNS Developer Guide.</i>
/// </p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct MessageAttributeValue {
    /// <p>Amazon SNS supports the following logical data types: String, String.Array, Number, and
    /// Binary. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/SNSMessageAttributes.html#SNSMessageAttributes.DataTypes">Message
    /// Attribute Data Types</a>.</p>
    pub data_type: std::option::Option<std::string::String>,
    /// <p>Strings are Unicode with UTF8 binary encoding. For a list of code values, see <a href="https://en.wikipedia.org/wiki/ASCII#ASCII_printable_characters">ASCII Printable
    /// Characters</a>.</p>
    pub string_value: std::option::Option<std::string::String>,
    /// <p>Binary type attributes can store any binary data, for example, compressed data,
    /// encrypted data, or images.</p>
    pub binary_value: std::option::Option<smithy_types::Blob>,
}
impl std::fmt::Debug for MessageAttributeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("MessageAttributeValue");
        formatter.field("data_type", &self.data_type);
        formatter.field("string_value", &self.string_value);
        formatter.field("binary_value", &self.binary_value);
        formatter.finish()
    }
}
/// See [`MessageAttributeValue`](crate::model::MessageAttributeValue)
pub mod message_attribute_value {
    /// A builder for [`MessageAttributeValue`](crate::model::MessageAttributeValue)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) data_type: std::option::Option<std::string::String>,
        pub(crate) string_value: std::option::Option<std::string::String>,
        pub(crate) binary_value: std::option::Option<smithy_types::Blob>,
    }
    impl Builder {
        /// <p>Amazon SNS supports the following logical data types: String, String.Array, Number, and
        /// Binary. For more information, see <a href="https://docs.aws.amazon.com/sns/latest/dg/SNSMessageAttributes.html#SNSMessageAttributes.DataTypes">Message
        /// Attribute Data Types</a>.</p>
        pub fn data_type(mut self, input: impl Into<std::string::String>) -> Self {
            self.data_type = Some(input.into());
            self
        }
        pub fn set_data_type(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.data_type = input;
            self
        }
        /// <p>Strings are Unicode with UTF8 binary encoding. For a list of code values, see <a href="https://en.wikipedia.org/wiki/ASCII#ASCII_printable_characters">ASCII Printable
        /// Characters</a>.</p>
        pub fn string_value(mut self, input: impl Into<std::string::String>) -> Self {
            self.string_value = Some(input.into());
            self
        }
        pub fn set_string_value(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.string_value = input;
            self
        }
        /// <p>Binary type attributes can store any binary data, for example, compressed data,
        /// encrypted data, or images.</p>
        pub fn binary_value(mut self, input: smithy_types::Blob) -> Self {
            self.binary_value = Some(input);
            self
        }
        pub fn set_binary_value(mut self, input: std::option::Option<smithy_types::Blob>) -> Self {
            self.binary_value = input;
            self
        }
        /// Consumes the builder and constructs a [`MessageAttributeValue`](crate::model::MessageAttributeValue)
        pub fn build(self) -> crate::model::MessageAttributeValue {
            crate::model::MessageAttributeValue {
                data_type: self.data_type,
                string_value: self.string_value,
                binary_value: self.binary_value,
            }
        }
    }
}
impl MessageAttributeValue {
    /// Creates a new builder-style object to manufacture [`MessageAttributeValue`](crate::model::MessageAttributeValue)
    pub fn builder() -> crate::model::message_attribute_value::Builder {
        crate::model::message_attribute_value::Builder::default()
    }
}

/// <p>A wrapper type for the topic's Amazon Resource Name (ARN). To retrieve a topic's
/// attributes, use <code>GetTopicAttributes</code>.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct Topic {
    /// <p>The topic's ARN.</p>
    pub topic_arn: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for Topic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("Topic");
        formatter.field("topic_arn", &self.topic_arn);
        formatter.finish()
    }
}
/// See [`Topic`](crate::model::Topic)
pub mod topic {
    /// A builder for [`Topic`](crate::model::Topic)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) topic_arn: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The topic's ARN.</p>
        pub fn topic_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.topic_arn = Some(input.into());
            self
        }
        pub fn set_topic_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.topic_arn = input;
            self
        }
        /// Consumes the builder and constructs a [`Topic`](crate::model::Topic)
        pub fn build(self) -> crate::model::Topic {
            crate::model::Topic {
                topic_arn: self.topic_arn,
            }
        }
    }
}
impl Topic {
    /// Creates a new builder-style object to manufacture [`Topic`](crate::model::Topic)
    pub fn builder() -> crate::model::topic::Builder {
        crate::model::topic::Builder::default()
    }
}

/// <p>A wrapper type for the attributes of an Amazon SNS subscription.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct Subscription {
    /// <p>The subscription's ARN.</p>
    pub subscription_arn: std::option::Option<std::string::String>,
    /// <p>The subscription's owner.</p>
    pub owner: std::option::Option<std::string::String>,
    /// <p>The subscription's protocol.</p>
    pub protocol: std::option::Option<std::string::String>,
    /// <p>The subscription's endpoint (format depends on the protocol).</p>
    pub endpoint: std::option::Option<std::string::String>,
    /// <p>The ARN of the subscription's topic.</p>
    pub topic_arn: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for Subscription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("Subscription");
        formatter.field("subscription_arn", &self.subscription_arn);
        formatter.field("owner", &self.owner);
        formatter.field("protocol", &self.protocol);
        formatter.field("endpoint", &self.endpoint);
        formatter.field("topic_arn", &self.topic_arn);
        formatter.finish()
    }
}
/// See [`Subscription`](crate::model::Subscription)
pub mod subscription {
    /// A builder for [`Subscription`](crate::model::Subscription)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) subscription_arn: std::option::Option<std::string::String>,
        pub(crate) owner: std::option::Option<std::string::String>,
        pub(crate) protocol: std::option::Option<std::string::String>,
        pub(crate) endpoint: std::option::Option<std::string::String>,
        pub(crate) topic_arn: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The subscription's ARN.</p>
        pub fn subscription_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.subscription_arn = Some(input.into());
            self
        }
        pub fn set_subscription_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.subscription_arn = input;
            self
        }
        /// <p>The subscription's owner.</p>
        pub fn owner(mut self, input: impl Into<std::string::String>) -> Self {
            self.owner = Some(input.into());
            self
        }
        pub fn set_owner(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.owner = input;
            self
        }
        /// <p>The subscription's protocol.</p>
        pub fn protocol(mut self, input: impl Into<std::string::String>) -> Self {
            self.protocol = Some(input.into());
            self
        }
        pub fn set_protocol(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.protocol = input;
            self
        }
        /// <p>The subscription's endpoint (format depends on the protocol).</p>
        pub fn endpoint(mut self, input: impl Into<std::string::String>) -> Self {
            self.endpoint = Some(input.into());
            self
        }
        pub fn set_endpoint(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.endpoint = input;
            self
        }
        /// <p>The ARN of the subscription's topic.</p>
        pub fn topic_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.topic_arn = Some(input.into());
            self
        }
        pub fn set_topic_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.topic_arn = input;
            self
        }
        /// Consumes the builder and constructs a [`Subscription`](crate::model::Subscription)
        pub fn build(self) -> crate::model::Subscription {
            crate::model::Subscription {
                subscription_arn: self.subscription_arn,
                owner: self.owner,
                protocol: self.protocol,
                endpoint: self.endpoint,
                topic_arn: self.topic_arn,
            }
        }
    }
}
impl Subscription {
    /// Creates a new builder-style object to manufacture [`Subscription`](crate::model::Subscription)
    pub fn builder() -> crate::model::subscription::Builder {
        crate::model::subscription::Builder::default()
    }
}

/// <p>A verified or pending destination phone number in the SMS sandbox.</p>
/// <p>When you start using Amazon SNS to send SMS messages, your account is in the
/// <i>SMS sandbox</i>. The SMS sandbox provides a safe environment for
/// you to try Amazon SNS features without risking your reputation as an SMS sender. While your
/// account is in the SMS sandbox, you can use all of the features of Amazon SNS. However, you can send
/// SMS messages only to verified destination phone numbers. For more information, including how to
/// move out of the sandbox to send messages without restrictions,
/// see <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-sms-sandbox.html">SMS sandbox</a> in
/// the <i>Amazon SNS Developer Guide</i>.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct SmsSandboxPhoneNumber {
    /// <p>The destination phone number.</p>
    pub phone_number: std::option::Option<std::string::String>,
    /// <p>The destination phone number's verification status.</p>
    pub status: std::option::Option<crate::model::SmsSandboxPhoneNumberVerificationStatus>,
}
impl std::fmt::Debug for SmsSandboxPhoneNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("SmsSandboxPhoneNumber");
        formatter.field("phone_number", &self.phone_number);
        formatter.field("status", &self.status);
        formatter.finish()
    }
}
/// See [`SmsSandboxPhoneNumber`](crate::model::SmsSandboxPhoneNumber)
pub mod sms_sandbox_phone_number {
    /// A builder for [`SmsSandboxPhoneNumber`](crate::model::SmsSandboxPhoneNumber)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) phone_number: std::option::Option<std::string::String>,
        pub(crate) status:
            std::option::Option<crate::model::SmsSandboxPhoneNumberVerificationStatus>,
    }
    impl Builder {
        /// <p>The destination phone number.</p>
        pub fn phone_number(mut self, input: impl Into<std::string::String>) -> Self {
            self.phone_number = Some(input.into());
            self
        }
        pub fn set_phone_number(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.phone_number = input;
            self
        }
        /// <p>The destination phone number's verification status.</p>
        pub fn status(
            mut self,
            input: crate::model::SmsSandboxPhoneNumberVerificationStatus,
        ) -> Self {
            self.status = Some(input);
            self
        }
        pub fn set_status(
            mut self,
            input: std::option::Option<crate::model::SmsSandboxPhoneNumberVerificationStatus>,
        ) -> Self {
            self.status = input;
            self
        }
        /// Consumes the builder and constructs a [`SmsSandboxPhoneNumber`](crate::model::SmsSandboxPhoneNumber)
        pub fn build(self) -> crate::model::SmsSandboxPhoneNumber {
            crate::model::SmsSandboxPhoneNumber {
                phone_number: self.phone_number,
                status: self.status,
            }
        }
    }
}
impl SmsSandboxPhoneNumber {
    /// Creates a new builder-style object to manufacture [`SmsSandboxPhoneNumber`](crate::model::SmsSandboxPhoneNumber)
    pub fn builder() -> crate::model::sms_sandbox_phone_number::Builder {
        crate::model::sms_sandbox_phone_number::Builder::default()
    }
}

/// Enum listing out all supported destination phone number verification statuses. The following enum values are
/// supported.
/// 1. PENDING : The destination phone number is pending verification.
/// 2. VERIFIED : The destination phone number is verified.
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum SmsSandboxPhoneNumberVerificationStatus {
    Pending,
    Verified,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for SmsSandboxPhoneNumberVerificationStatus {
    fn from(s: &str) -> Self {
        match s {
            "Pending" => SmsSandboxPhoneNumberVerificationStatus::Pending,
            "Verified" => SmsSandboxPhoneNumberVerificationStatus::Verified,
            other => SmsSandboxPhoneNumberVerificationStatus::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for SmsSandboxPhoneNumberVerificationStatus {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(SmsSandboxPhoneNumberVerificationStatus::from(s))
    }
}
impl SmsSandboxPhoneNumberVerificationStatus {
    pub fn as_str(&self) -> &str {
        match self {
            SmsSandboxPhoneNumberVerificationStatus::Pending => "Pending",
            SmsSandboxPhoneNumberVerificationStatus::Verified => "Verified",
            SmsSandboxPhoneNumberVerificationStatus::Unknown(s) => s.as_ref(),
        }
    }
    pub fn values() -> &'static [&'static str] {
        &["Pending", "Verified"]
    }
}
impl AsRef<str> for SmsSandboxPhoneNumberVerificationStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>Platform application object.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct PlatformApplication {
    /// <p>PlatformApplicationArn for platform application object.</p>
    pub platform_application_arn: std::option::Option<std::string::String>,
    /// <p>Attributes for platform application object.</p>
    pub attributes:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
}
impl std::fmt::Debug for PlatformApplication {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("PlatformApplication");
        formatter.field("platform_application_arn", &self.platform_application_arn);
        formatter.field("attributes", &self.attributes);
        formatter.finish()
    }
}
/// See [`PlatformApplication`](crate::model::PlatformApplication)
pub mod platform_application {
    /// A builder for [`PlatformApplication`](crate::model::PlatformApplication)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) platform_application_arn: std::option::Option<std::string::String>,
        pub(crate) attributes: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    }
    impl Builder {
        /// <p>PlatformApplicationArn for platform application object.</p>
        pub fn platform_application_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.platform_application_arn = Some(input.into());
            self
        }
        pub fn set_platform_application_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.platform_application_arn = input;
            self
        }
        pub fn attributes(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            let mut hash_map = self.attributes.unwrap_or_default();
            hash_map.insert(k.into(), v.into());
            self.attributes = Some(hash_map);
            self
        }
        pub fn set_attributes(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.attributes = input;
            self
        }
        /// Consumes the builder and constructs a [`PlatformApplication`](crate::model::PlatformApplication)
        pub fn build(self) -> crate::model::PlatformApplication {
            crate::model::PlatformApplication {
                platform_application_arn: self.platform_application_arn,
                attributes: self.attributes,
            }
        }
    }
}
impl PlatformApplication {
    /// Creates a new builder-style object to manufacture [`PlatformApplication`](crate::model::PlatformApplication)
    pub fn builder() -> crate::model::platform_application::Builder {
        crate::model::platform_application::Builder::default()
    }
}

/// <p>A list of phone numbers and their metadata.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct PhoneNumberInformation {
    /// <p>The date and time when the phone number was created.</p>
    pub created_at: std::option::Option<smithy_types::Instant>,
    /// <p>The phone number.</p>
    pub phone_number: std::option::Option<std::string::String>,
    /// <p>The status of the phone number.</p>
    pub status: std::option::Option<std::string::String>,
    /// <p>The two-character code for the country or region, in ISO 3166-1 alpha-2 format.</p>
    pub iso2_country_code: std::option::Option<std::string::String>,
    /// <p>The list of supported routes.</p>
    pub route_type: std::option::Option<crate::model::RouteType>,
    /// <p>The capabilities of each phone number.</p>
    pub number_capabilities: std::option::Option<std::vec::Vec<crate::model::NumberCapability>>,
}
impl std::fmt::Debug for PhoneNumberInformation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("PhoneNumberInformation");
        formatter.field("created_at", &self.created_at);
        formatter.field("phone_number", &self.phone_number);
        formatter.field("status", &self.status);
        formatter.field("iso2_country_code", &self.iso2_country_code);
        formatter.field("route_type", &self.route_type);
        formatter.field("number_capabilities", &self.number_capabilities);
        formatter.finish()
    }
}
/// See [`PhoneNumberInformation`](crate::model::PhoneNumberInformation)
pub mod phone_number_information {
    /// A builder for [`PhoneNumberInformation`](crate::model::PhoneNumberInformation)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) created_at: std::option::Option<smithy_types::Instant>,
        pub(crate) phone_number: std::option::Option<std::string::String>,
        pub(crate) status: std::option::Option<std::string::String>,
        pub(crate) iso2_country_code: std::option::Option<std::string::String>,
        pub(crate) route_type: std::option::Option<crate::model::RouteType>,
        pub(crate) number_capabilities:
            std::option::Option<std::vec::Vec<crate::model::NumberCapability>>,
    }
    impl Builder {
        /// <p>The date and time when the phone number was created.</p>
        pub fn created_at(mut self, input: smithy_types::Instant) -> Self {
            self.created_at = Some(input);
            self
        }
        pub fn set_created_at(mut self, input: std::option::Option<smithy_types::Instant>) -> Self {
            self.created_at = input;
            self
        }
        /// <p>The phone number.</p>
        pub fn phone_number(mut self, input: impl Into<std::string::String>) -> Self {
            self.phone_number = Some(input.into());
            self
        }
        pub fn set_phone_number(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.phone_number = input;
            self
        }
        /// <p>The status of the phone number.</p>
        pub fn status(mut self, input: impl Into<std::string::String>) -> Self {
            self.status = Some(input.into());
            self
        }
        pub fn set_status(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.status = input;
            self
        }
        /// <p>The two-character code for the country or region, in ISO 3166-1 alpha-2 format.</p>
        pub fn iso2_country_code(mut self, input: impl Into<std::string::String>) -> Self {
            self.iso2_country_code = Some(input.into());
            self
        }
        pub fn set_iso2_country_code(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.iso2_country_code = input;
            self
        }
        /// <p>The list of supported routes.</p>
        pub fn route_type(mut self, input: crate::model::RouteType) -> Self {
            self.route_type = Some(input);
            self
        }
        pub fn set_route_type(
            mut self,
            input: std::option::Option<crate::model::RouteType>,
        ) -> Self {
            self.route_type = input;
            self
        }
        pub fn number_capabilities(
            mut self,
            input: impl Into<crate::model::NumberCapability>,
        ) -> Self {
            let mut v = self.number_capabilities.unwrap_or_default();
            v.push(input.into());
            self.number_capabilities = Some(v);
            self
        }
        pub fn set_number_capabilities(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::NumberCapability>>,
        ) -> Self {
            self.number_capabilities = input;
            self
        }
        /// Consumes the builder and constructs a [`PhoneNumberInformation`](crate::model::PhoneNumberInformation)
        pub fn build(self) -> crate::model::PhoneNumberInformation {
            crate::model::PhoneNumberInformation {
                created_at: self.created_at,
                phone_number: self.phone_number,
                status: self.status,
                iso2_country_code: self.iso2_country_code,
                route_type: self.route_type,
                number_capabilities: self.number_capabilities,
            }
        }
    }
}
impl PhoneNumberInformation {
    /// Creates a new builder-style object to manufacture [`PhoneNumberInformation`](crate::model::PhoneNumberInformation)
    pub fn builder() -> crate::model::phone_number_information::Builder {
        crate::model::phone_number_information::Builder::default()
    }
}

/// Enum listing out all supported number capabilities.
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum NumberCapability {
    Mms,
    Sms,
    Voice,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for NumberCapability {
    fn from(s: &str) -> Self {
        match s {
            "MMS" => NumberCapability::Mms,
            "SMS" => NumberCapability::Sms,
            "VOICE" => NumberCapability::Voice,
            other => NumberCapability::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for NumberCapability {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(NumberCapability::from(s))
    }
}
impl NumberCapability {
    pub fn as_str(&self) -> &str {
        match self {
            NumberCapability::Mms => "MMS",
            NumberCapability::Sms => "SMS",
            NumberCapability::Voice => "VOICE",
            NumberCapability::Unknown(s) => s.as_ref(),
        }
    }
    pub fn values() -> &'static [&'static str] {
        &["MMS", "SMS", "VOICE"]
    }
}
impl AsRef<str> for NumberCapability {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// Enum listing out all supported route types. The following enum values are supported.
/// 1. Transactional : Non-marketing traffic
/// 2. Promotional : Marketing
/// 3. Premium : Premium routes for OTP delivery to the carriers
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum RouteType {
    Premium,
    Promotional,
    Transactional,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for RouteType {
    fn from(s: &str) -> Self {
        match s {
            "Premium" => RouteType::Premium,
            "Promotional" => RouteType::Promotional,
            "Transactional" => RouteType::Transactional,
            other => RouteType::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for RouteType {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(RouteType::from(s))
    }
}
impl RouteType {
    pub fn as_str(&self) -> &str {
        match self {
            RouteType::Premium => "Premium",
            RouteType::Promotional => "Promotional",
            RouteType::Transactional => "Transactional",
            RouteType::Unknown(s) => s.as_ref(),
        }
    }
    pub fn values() -> &'static [&'static str] {
        &["Premium", "Promotional", "Transactional"]
    }
}
impl AsRef<str> for RouteType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>Endpoint for mobile app and device.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct Endpoint {
    /// <p>EndpointArn for mobile app and device.</p>
    pub endpoint_arn: std::option::Option<std::string::String>,
    /// <p>Attributes for endpoint.</p>
    pub attributes:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
}
impl std::fmt::Debug for Endpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("Endpoint");
        formatter.field("endpoint_arn", &self.endpoint_arn);
        formatter.field("attributes", &self.attributes);
        formatter.finish()
    }
}
/// See [`Endpoint`](crate::model::Endpoint)
pub mod endpoint {
    /// A builder for [`Endpoint`](crate::model::Endpoint)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) endpoint_arn: std::option::Option<std::string::String>,
        pub(crate) attributes: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    }
    impl Builder {
        /// <p>EndpointArn for mobile app and device.</p>
        pub fn endpoint_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.endpoint_arn = Some(input.into());
            self
        }
        pub fn set_endpoint_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.endpoint_arn = input;
            self
        }
        pub fn attributes(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            let mut hash_map = self.attributes.unwrap_or_default();
            hash_map.insert(k.into(), v.into());
            self.attributes = Some(hash_map);
            self
        }
        pub fn set_attributes(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.attributes = input;
            self
        }
        /// Consumes the builder and constructs a [`Endpoint`](crate::model::Endpoint)
        pub fn build(self) -> crate::model::Endpoint {
            crate::model::Endpoint {
                endpoint_arn: self.endpoint_arn,
                attributes: self.attributes,
            }
        }
    }
}
impl Endpoint {
    /// Creates a new builder-style object to manufacture [`Endpoint`](crate::model::Endpoint)
    pub fn builder() -> crate::model::endpoint::Builder {
        crate::model::endpoint::Builder::default()
    }
}

/// Supported language code for sending OTP message
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum LanguageCodeString {
    DeDe,
    EnGb,
    EnUs,
    Es419,
    EsEs,
    FrCa,
    FrFr,
    ItIt,
    JpJp,
    KrKr,
    PtBr,
    ZhCn,
    ZhTw,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for LanguageCodeString {
    fn from(s: &str) -> Self {
        match s {
            "de-DE" => LanguageCodeString::DeDe,
            "en-GB" => LanguageCodeString::EnGb,
            "en-US" => LanguageCodeString::EnUs,
            "es-419" => LanguageCodeString::Es419,
            "es-ES" => LanguageCodeString::EsEs,
            "fr-CA" => LanguageCodeString::FrCa,
            "fr-FR" => LanguageCodeString::FrFr,
            "it-IT" => LanguageCodeString::ItIt,
            "ja-JP" => LanguageCodeString::JpJp,
            "kr-KR" => LanguageCodeString::KrKr,
            "pt-BR" => LanguageCodeString::PtBr,
            "zh-CN" => LanguageCodeString::ZhCn,
            "zh-TW" => LanguageCodeString::ZhTw,
            other => LanguageCodeString::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for LanguageCodeString {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(LanguageCodeString::from(s))
    }
}
impl LanguageCodeString {
    pub fn as_str(&self) -> &str {
        match self {
            LanguageCodeString::DeDe => "de-DE",
            LanguageCodeString::EnGb => "en-GB",
            LanguageCodeString::EnUs => "en-US",
            LanguageCodeString::Es419 => "es-419",
            LanguageCodeString::EsEs => "es-ES",
            LanguageCodeString::FrCa => "fr-CA",
            LanguageCodeString::FrFr => "fr-FR",
            LanguageCodeString::ItIt => "it-IT",
            LanguageCodeString::JpJp => "ja-JP",
            LanguageCodeString::KrKr => "kr-KR",
            LanguageCodeString::PtBr => "pt-BR",
            LanguageCodeString::ZhCn => "zh-CN",
            LanguageCodeString::ZhTw => "zh-TW",
            LanguageCodeString::Unknown(s) => s.as_ref(),
        }
    }
    pub fn values() -> &'static [&'static str] {
        &[
            "de-DE", "en-GB", "en-US", "es-419", "es-ES", "fr-CA", "fr-FR", "it-IT", "ja-JP",
            "kr-KR", "pt-BR", "zh-CN", "zh-TW",
        ]
    }
}
impl AsRef<str> for LanguageCodeString {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
