// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>The constraints that you want all returned products to match.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct Filter {
    /// <p>The type of filter that you want to use.</p>
    /// <p>Valid values are: <code>TERM_MATCH</code>. <code>TERM_MATCH</code> returns only
    /// products that match both the given filter field and the given value.</p>
    pub r#type: std::option::Option<crate::model::FilterType>,
    /// <p>The product metadata field that you want to filter on. You can filter by just the
    /// service code to see all products for a specific service, filter
    /// by just the attribute name to see a specific attribute for multiple services, or use both a service code
    /// and an attribute name to retrieve only products that match both fields.</p>
    /// <p>Valid values include: <code>ServiceCode</code>, and all attribute names</p>
    /// <p>For example, you can filter by the <code>AmazonEC2</code> service code and the
    /// <code>volumeType</code> attribute name to get the prices for only Amazon EC2 volumes.</p>
    pub field: std::option::Option<std::string::String>,
    /// <p>The service code or attribute value that you want to filter by. If you are filtering by
    /// service code this is the actual service code, such as <code>AmazonEC2</code>. If you are
    /// filtering by attribute name, this is the attribute value that you want the returned products
    /// to match, such as a <code>Provisioned IOPS</code> volume.</p>
    pub value: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("Filter");
        formatter.field("r#type", &self.r#type);
        formatter.field("field", &self.field);
        formatter.field("value", &self.value);
        formatter.finish()
    }
}
/// See [`Filter`](crate::model::Filter)
pub mod filter {
    /// A builder for [`Filter`](crate::model::Filter)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) r#type: std::option::Option<crate::model::FilterType>,
        pub(crate) field: std::option::Option<std::string::String>,
        pub(crate) value: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The type of filter that you want to use.</p>
        /// <p>Valid values are: <code>TERM_MATCH</code>. <code>TERM_MATCH</code> returns only
        /// products that match both the given filter field and the given value.</p>
        pub fn r#type(mut self, input: crate::model::FilterType) -> Self {
            self.r#type = Some(input);
            self
        }
        pub fn set_type(mut self, input: std::option::Option<crate::model::FilterType>) -> Self {
            self.r#type = input;
            self
        }
        /// <p>The product metadata field that you want to filter on. You can filter by just the
        /// service code to see all products for a specific service, filter
        /// by just the attribute name to see a specific attribute for multiple services, or use both a service code
        /// and an attribute name to retrieve only products that match both fields.</p>
        /// <p>Valid values include: <code>ServiceCode</code>, and all attribute names</p>
        /// <p>For example, you can filter by the <code>AmazonEC2</code> service code and the
        /// <code>volumeType</code> attribute name to get the prices for only Amazon EC2 volumes.</p>
        pub fn field(mut self, input: impl Into<std::string::String>) -> Self {
            self.field = Some(input.into());
            self
        }
        pub fn set_field(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.field = input;
            self
        }
        /// <p>The service code or attribute value that you want to filter by. If you are filtering by
        /// service code this is the actual service code, such as <code>AmazonEC2</code>. If you are
        /// filtering by attribute name, this is the attribute value that you want the returned products
        /// to match, such as a <code>Provisioned IOPS</code> volume.</p>
        pub fn value(mut self, input: impl Into<std::string::String>) -> Self {
            self.value = Some(input.into());
            self
        }
        pub fn set_value(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.value = input;
            self
        }
        /// Consumes the builder and constructs a [`Filter`](crate::model::Filter)
        pub fn build(self) -> crate::model::Filter {
            crate::model::Filter {
                r#type: self.r#type,
                field: self.field,
                value: self.value,
            }
        }
    }
}
impl Filter {
    /// Creates a new builder-style object to manufacture [`Filter`](crate::model::Filter)
    pub fn builder() -> crate::model::filter::Builder {
        crate::model::filter::Builder::default()
    }
}

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
pub enum FilterType {
    TermMatch,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for FilterType {
    fn from(s: &str) -> Self {
        match s {
            "TERM_MATCH" => FilterType::TermMatch,
            other => FilterType::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for FilterType {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(FilterType::from(s))
    }
}
impl FilterType {
    pub fn as_str(&self) -> &str {
        match self {
            FilterType::TermMatch => "TERM_MATCH",
            FilterType::Unknown(s) => s.as_ref(),
        }
    }
    pub fn values() -> &'static [&'static str] {
        &["TERM_MATCH"]
    }
}
impl AsRef<str> for FilterType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>The values of a given attribute, such as <code>Throughput Optimized HDD</code> or <code>Provisioned
/// IOPS</code> for the <code>Amazon EC2</code>
/// <code>volumeType</code> attribute.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct AttributeValue {
    /// <p>The specific value of an <code>attributeName</code>.</p>
    pub value: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for AttributeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("AttributeValue");
        formatter.field("value", &self.value);
        formatter.finish()
    }
}
/// See [`AttributeValue`](crate::model::AttributeValue)
pub mod attribute_value {
    /// A builder for [`AttributeValue`](crate::model::AttributeValue)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) value: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The specific value of an <code>attributeName</code>.</p>
        pub fn value(mut self, input: impl Into<std::string::String>) -> Self {
            self.value = Some(input.into());
            self
        }
        pub fn set_value(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.value = input;
            self
        }
        /// Consumes the builder and constructs a [`AttributeValue`](crate::model::AttributeValue)
        pub fn build(self) -> crate::model::AttributeValue {
            crate::model::AttributeValue { value: self.value }
        }
    }
}
impl AttributeValue {
    /// Creates a new builder-style object to manufacture [`AttributeValue`](crate::model::AttributeValue)
    pub fn builder() -> crate::model::attribute_value::Builder {
        crate::model::attribute_value::Builder::default()
    }
}

/// <p>The metadata for a service, such as the service code and available attribute names.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct Service {
    /// <p>The code for the Amazon Web Services service.</p>
    pub service_code: std::option::Option<std::string::String>,
    /// <p>The attributes that are available for this service.</p>
    pub attribute_names: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl std::fmt::Debug for Service {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("Service");
        formatter.field("service_code", &self.service_code);
        formatter.field("attribute_names", &self.attribute_names);
        formatter.finish()
    }
}
/// See [`Service`](crate::model::Service)
pub mod service {
    /// A builder for [`Service`](crate::model::Service)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) service_code: std::option::Option<std::string::String>,
        pub(crate) attribute_names: std::option::Option<std::vec::Vec<std::string::String>>,
    }
    impl Builder {
        /// <p>The code for the Amazon Web Services service.</p>
        pub fn service_code(mut self, input: impl Into<std::string::String>) -> Self {
            self.service_code = Some(input.into());
            self
        }
        pub fn set_service_code(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.service_code = input;
            self
        }
        pub fn attribute_names(mut self, input: impl Into<std::string::String>) -> Self {
            let mut v = self.attribute_names.unwrap_or_default();
            v.push(input.into());
            self.attribute_names = Some(v);
            self
        }
        pub fn set_attribute_names(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.attribute_names = input;
            self
        }
        /// Consumes the builder and constructs a [`Service`](crate::model::Service)
        pub fn build(self) -> crate::model::Service {
            crate::model::Service {
                service_code: self.service_code,
                attribute_names: self.attribute_names,
            }
        }
    }
}
impl Service {
    /// Creates a new builder-style object to manufacture [`Service`](crate::model::Service)
    pub fn builder() -> crate::model::service::Builder {
        crate::model::service::Builder::default()
    }
}
