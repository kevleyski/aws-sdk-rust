// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes an association with a resource share and either a principal or a resource.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ResourceShareAssociation {
    /// <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resoure Name (ARN)</a> of the resource share.</p>
    #[doc(hidden)]
    pub resource_share_arn: std::option::Option<std::string::String>,
    /// <p>The name of the resource share.</p>
    #[doc(hidden)]
    pub resource_share_name: std::option::Option<std::string::String>,
    /// <p>The associated entity. This can be either of the following:</p>
    /// <ul>
    /// <li> <p>For a resource association, this is the <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resoure Name (ARN)</a> of the resource.</p> </li>
    /// <li> <p>For principal associations, this is one of the following:</p>
    /// <ul>
    /// <li> <p>The ID of an Amazon Web Services account</p> </li>
    /// <li> <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resoure Name (ARN)</a> of an organization in Organizations</p> </li>
    /// <li> <p>The ARN of an organizational unit (OU) in Organizations</p> </li>
    /// <li> <p>The ARN of an IAM role</p> </li>
    /// <li> <p>The ARN of an IAM user</p> </li>
    /// </ul> </li>
    /// </ul>
    #[doc(hidden)]
    pub associated_entity: std::option::Option<std::string::String>,
    /// <p>The type of entity included in this association.</p>
    #[doc(hidden)]
    pub association_type: std::option::Option<crate::types::ResourceShareAssociationType>,
    /// <p>The current status of the association.</p>
    #[doc(hidden)]
    pub status: std::option::Option<crate::types::ResourceShareAssociationStatus>,
    /// <p>A message about the status of the association.</p>
    #[doc(hidden)]
    pub status_message: std::option::Option<std::string::String>,
    /// <p>The date and time when the association was created.</p>
    #[doc(hidden)]
    pub creation_time: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The date and time when the association was last updated.</p>
    #[doc(hidden)]
    pub last_updated_time: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>Indicates whether the principal belongs to the same organization in Organizations as the Amazon Web Services account that owns the resource share.</p>
    #[doc(hidden)]
    pub external: std::option::Option<bool>,
}
impl ResourceShareAssociation {
    /// <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resoure Name (ARN)</a> of the resource share.</p>
    pub fn resource_share_arn(&self) -> std::option::Option<&str> {
        self.resource_share_arn.as_deref()
    }
    /// <p>The name of the resource share.</p>
    pub fn resource_share_name(&self) -> std::option::Option<&str> {
        self.resource_share_name.as_deref()
    }
    /// <p>The associated entity. This can be either of the following:</p>
    /// <ul>
    /// <li> <p>For a resource association, this is the <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resoure Name (ARN)</a> of the resource.</p> </li>
    /// <li> <p>For principal associations, this is one of the following:</p>
    /// <ul>
    /// <li> <p>The ID of an Amazon Web Services account</p> </li>
    /// <li> <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resoure Name (ARN)</a> of an organization in Organizations</p> </li>
    /// <li> <p>The ARN of an organizational unit (OU) in Organizations</p> </li>
    /// <li> <p>The ARN of an IAM role</p> </li>
    /// <li> <p>The ARN of an IAM user</p> </li>
    /// </ul> </li>
    /// </ul>
    pub fn associated_entity(&self) -> std::option::Option<&str> {
        self.associated_entity.as_deref()
    }
    /// <p>The type of entity included in this association.</p>
    pub fn association_type(
        &self,
    ) -> std::option::Option<&crate::types::ResourceShareAssociationType> {
        self.association_type.as_ref()
    }
    /// <p>The current status of the association.</p>
    pub fn status(&self) -> std::option::Option<&crate::types::ResourceShareAssociationStatus> {
        self.status.as_ref()
    }
    /// <p>A message about the status of the association.</p>
    pub fn status_message(&self) -> std::option::Option<&str> {
        self.status_message.as_deref()
    }
    /// <p>The date and time when the association was created.</p>
    pub fn creation_time(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.creation_time.as_ref()
    }
    /// <p>The date and time when the association was last updated.</p>
    pub fn last_updated_time(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.last_updated_time.as_ref()
    }
    /// <p>Indicates whether the principal belongs to the same organization in Organizations as the Amazon Web Services account that owns the resource share.</p>
    pub fn external(&self) -> std::option::Option<bool> {
        self.external
    }
}
impl ResourceShareAssociation {
    /// Creates a new builder-style object to manufacture [`ResourceShareAssociation`](crate::types::ResourceShareAssociation).
    pub fn builder() -> crate::types::builders::ResourceShareAssociationBuilder {
        crate::types::builders::ResourceShareAssociationBuilder::default()
    }
}

/// A builder for [`ResourceShareAssociation`](crate::types::ResourceShareAssociation).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct ResourceShareAssociationBuilder {
    pub(crate) resource_share_arn: std::option::Option<std::string::String>,
    pub(crate) resource_share_name: std::option::Option<std::string::String>,
    pub(crate) associated_entity: std::option::Option<std::string::String>,
    pub(crate) association_type: std::option::Option<crate::types::ResourceShareAssociationType>,
    pub(crate) status: std::option::Option<crate::types::ResourceShareAssociationStatus>,
    pub(crate) status_message: std::option::Option<std::string::String>,
    pub(crate) creation_time: std::option::Option<aws_smithy_types::DateTime>,
    pub(crate) last_updated_time: std::option::Option<aws_smithy_types::DateTime>,
    pub(crate) external: std::option::Option<bool>,
}
impl ResourceShareAssociationBuilder {
    /// <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resoure Name (ARN)</a> of the resource share.</p>
    pub fn resource_share_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.resource_share_arn = Some(input.into());
        self
    }
    /// <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resoure Name (ARN)</a> of the resource share.</p>
    pub fn set_resource_share_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.resource_share_arn = input;
        self
    }
    /// <p>The name of the resource share.</p>
    pub fn resource_share_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.resource_share_name = Some(input.into());
        self
    }
    /// <p>The name of the resource share.</p>
    pub fn set_resource_share_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.resource_share_name = input;
        self
    }
    /// <p>The associated entity. This can be either of the following:</p>
    /// <ul>
    /// <li> <p>For a resource association, this is the <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resoure Name (ARN)</a> of the resource.</p> </li>
    /// <li> <p>For principal associations, this is one of the following:</p>
    /// <ul>
    /// <li> <p>The ID of an Amazon Web Services account</p> </li>
    /// <li> <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resoure Name (ARN)</a> of an organization in Organizations</p> </li>
    /// <li> <p>The ARN of an organizational unit (OU) in Organizations</p> </li>
    /// <li> <p>The ARN of an IAM role</p> </li>
    /// <li> <p>The ARN of an IAM user</p> </li>
    /// </ul> </li>
    /// </ul>
    pub fn associated_entity(mut self, input: impl Into<std::string::String>) -> Self {
        self.associated_entity = Some(input.into());
        self
    }
    /// <p>The associated entity. This can be either of the following:</p>
    /// <ul>
    /// <li> <p>For a resource association, this is the <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resoure Name (ARN)</a> of the resource.</p> </li>
    /// <li> <p>For principal associations, this is one of the following:</p>
    /// <ul>
    /// <li> <p>The ID of an Amazon Web Services account</p> </li>
    /// <li> <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resoure Name (ARN)</a> of an organization in Organizations</p> </li>
    /// <li> <p>The ARN of an organizational unit (OU) in Organizations</p> </li>
    /// <li> <p>The ARN of an IAM role</p> </li>
    /// <li> <p>The ARN of an IAM user</p> </li>
    /// </ul> </li>
    /// </ul>
    pub fn set_associated_entity(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.associated_entity = input;
        self
    }
    /// <p>The type of entity included in this association.</p>
    pub fn association_type(mut self, input: crate::types::ResourceShareAssociationType) -> Self {
        self.association_type = Some(input);
        self
    }
    /// <p>The type of entity included in this association.</p>
    pub fn set_association_type(
        mut self,
        input: std::option::Option<crate::types::ResourceShareAssociationType>,
    ) -> Self {
        self.association_type = input;
        self
    }
    /// <p>The current status of the association.</p>
    pub fn status(mut self, input: crate::types::ResourceShareAssociationStatus) -> Self {
        self.status = Some(input);
        self
    }
    /// <p>The current status of the association.</p>
    pub fn set_status(
        mut self,
        input: std::option::Option<crate::types::ResourceShareAssociationStatus>,
    ) -> Self {
        self.status = input;
        self
    }
    /// <p>A message about the status of the association.</p>
    pub fn status_message(mut self, input: impl Into<std::string::String>) -> Self {
        self.status_message = Some(input.into());
        self
    }
    /// <p>A message about the status of the association.</p>
    pub fn set_status_message(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.status_message = input;
        self
    }
    /// <p>The date and time when the association was created.</p>
    pub fn creation_time(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.creation_time = Some(input);
        self
    }
    /// <p>The date and time when the association was created.</p>
    pub fn set_creation_time(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.creation_time = input;
        self
    }
    /// <p>The date and time when the association was last updated.</p>
    pub fn last_updated_time(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.last_updated_time = Some(input);
        self
    }
    /// <p>The date and time when the association was last updated.</p>
    pub fn set_last_updated_time(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.last_updated_time = input;
        self
    }
    /// <p>Indicates whether the principal belongs to the same organization in Organizations as the Amazon Web Services account that owns the resource share.</p>
    pub fn external(mut self, input: bool) -> Self {
        self.external = Some(input);
        self
    }
    /// <p>Indicates whether the principal belongs to the same organization in Organizations as the Amazon Web Services account that owns the resource share.</p>
    pub fn set_external(mut self, input: std::option::Option<bool>) -> Self {
        self.external = input;
        self
    }
    /// Consumes the builder and constructs a [`ResourceShareAssociation`](crate::types::ResourceShareAssociation).
    pub fn build(self) -> crate::types::ResourceShareAssociation {
        crate::types::ResourceShareAssociation {
            resource_share_arn: self.resource_share_arn,
            resource_share_name: self.resource_share_name,
            associated_entity: self.associated_entity,
            association_type: self.association_type,
            status: self.status,
            status_message: self.status_message,
            creation_time: self.creation_time,
            last_updated_time: self.last_updated_time,
            external: self.external,
        }
    }
}
