// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct UpdateFleetInput {
    /// <p> The ID of the fleet to update. </p>
    #[doc(hidden)]
    pub fleet_id: std::option::Option<std::string::String>,
    /// <p> An updated description of the fleet. </p>
    #[doc(hidden)]
    pub description: std::option::Option<std::string::String>,
}
impl UpdateFleetInput {
    /// <p> The ID of the fleet to update. </p>
    pub fn fleet_id(&self) -> std::option::Option<&str> {
        self.fleet_id.as_deref()
    }
    /// <p> An updated description of the fleet. </p>
    pub fn description(&self) -> std::option::Option<&str> {
        self.description.as_deref()
    }
}
impl UpdateFleetInput {
    /// Creates a new builder-style object to manufacture [`UpdateFleetInput`](crate::operation::update_fleet::UpdateFleetInput).
    pub fn builder() -> crate::operation::update_fleet::builders::UpdateFleetInputBuilder {
        crate::operation::update_fleet::builders::UpdateFleetInputBuilder::default()
    }
}

/// A builder for [`UpdateFleetInput`](crate::operation::update_fleet::UpdateFleetInput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct UpdateFleetInputBuilder {
    pub(crate) fleet_id: std::option::Option<std::string::String>,
    pub(crate) description: std::option::Option<std::string::String>,
}
impl UpdateFleetInputBuilder {
    /// <p> The ID of the fleet to update. </p>
    pub fn fleet_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.fleet_id = Some(input.into());
        self
    }
    /// <p> The ID of the fleet to update. </p>
    pub fn set_fleet_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.fleet_id = input;
        self
    }
    /// <p> An updated description of the fleet. </p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.description = Some(input.into());
        self
    }
    /// <p> An updated description of the fleet. </p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// Consumes the builder and constructs a [`UpdateFleetInput`](crate::operation::update_fleet::UpdateFleetInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::update_fleet::UpdateFleetInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(crate::operation::update_fleet::UpdateFleetInput {
            fleet_id: self.fleet_id,
            description: self.description,
        })
    }
}
