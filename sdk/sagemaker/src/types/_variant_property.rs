// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specifies a production variant property type for an Endpoint.</p>
/// <p>If you are updating an endpoint with the <code>UpdateEndpointInput$RetainAllVariantProperties</code> option set to <code>true</code>, the <code>VariantProperty</code> objects listed in <code>UpdateEndpointInput$ExcludeRetainedVariantProperties</code> override the existing variant properties of the endpoint.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct VariantProperty {
    /// <p>The type of variant property. The supported values are:</p>
    /// <ul>
    /// <li> <p> <code>DesiredInstanceCount</code>: Overrides the existing variant instance counts using the <code>ProductionVariant$InitialInstanceCount</code> values in the <code>CreateEndpointConfigInput$ProductionVariants</code>.</p> </li>
    /// <li> <p> <code>DesiredWeight</code>: Overrides the existing variant weights using the <code>ProductionVariant$InitialVariantWeight</code> values in the <code>CreateEndpointConfigInput$ProductionVariants</code>.</p> </li>
    /// <li> <p> <code>DataCaptureConfig</code>: (Not currently supported.)</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub variant_property_type: std::option::Option<crate::types::VariantPropertyType>,
}
impl VariantProperty {
    /// <p>The type of variant property. The supported values are:</p>
    /// <ul>
    /// <li> <p> <code>DesiredInstanceCount</code>: Overrides the existing variant instance counts using the <code>ProductionVariant$InitialInstanceCount</code> values in the <code>CreateEndpointConfigInput$ProductionVariants</code>.</p> </li>
    /// <li> <p> <code>DesiredWeight</code>: Overrides the existing variant weights using the <code>ProductionVariant$InitialVariantWeight</code> values in the <code>CreateEndpointConfigInput$ProductionVariants</code>.</p> </li>
    /// <li> <p> <code>DataCaptureConfig</code>: (Not currently supported.)</p> </li>
    /// </ul>
    pub fn variant_property_type(&self) -> std::option::Option<&crate::types::VariantPropertyType> {
        self.variant_property_type.as_ref()
    }
}
impl VariantProperty {
    /// Creates a new builder-style object to manufacture [`VariantProperty`](crate::types::VariantProperty).
    pub fn builder() -> crate::types::builders::VariantPropertyBuilder {
        crate::types::builders::VariantPropertyBuilder::default()
    }
}

/// A builder for [`VariantProperty`](crate::types::VariantProperty).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct VariantPropertyBuilder {
    pub(crate) variant_property_type: std::option::Option<crate::types::VariantPropertyType>,
}
impl VariantPropertyBuilder {
    /// <p>The type of variant property. The supported values are:</p>
    /// <ul>
    /// <li> <p> <code>DesiredInstanceCount</code>: Overrides the existing variant instance counts using the <code>ProductionVariant$InitialInstanceCount</code> values in the <code>CreateEndpointConfigInput$ProductionVariants</code>.</p> </li>
    /// <li> <p> <code>DesiredWeight</code>: Overrides the existing variant weights using the <code>ProductionVariant$InitialVariantWeight</code> values in the <code>CreateEndpointConfigInput$ProductionVariants</code>.</p> </li>
    /// <li> <p> <code>DataCaptureConfig</code>: (Not currently supported.)</p> </li>
    /// </ul>
    pub fn variant_property_type(mut self, input: crate::types::VariantPropertyType) -> Self {
        self.variant_property_type = Some(input);
        self
    }
    /// <p>The type of variant property. The supported values are:</p>
    /// <ul>
    /// <li> <p> <code>DesiredInstanceCount</code>: Overrides the existing variant instance counts using the <code>ProductionVariant$InitialInstanceCount</code> values in the <code>CreateEndpointConfigInput$ProductionVariants</code>.</p> </li>
    /// <li> <p> <code>DesiredWeight</code>: Overrides the existing variant weights using the <code>ProductionVariant$InitialVariantWeight</code> values in the <code>CreateEndpointConfigInput$ProductionVariants</code>.</p> </li>
    /// <li> <p> <code>DataCaptureConfig</code>: (Not currently supported.)</p> </li>
    /// </ul>
    pub fn set_variant_property_type(
        mut self,
        input: std::option::Option<crate::types::VariantPropertyType>,
    ) -> Self {
        self.variant_property_type = input;
        self
    }
    /// Consumes the builder and constructs a [`VariantProperty`](crate::types::VariantProperty).
    pub fn build(self) -> crate::types::VariantProperty {
        crate::types::VariantProperty {
            variant_property_type: self.variant_property_type,
        }
    }
}
