// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_grant_entitlement_request(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::GrantEntitlementRequest,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.data_transfer_subscriber_fee_percent != 0 {
        object.key("dataTransferSubscriberFeePercent").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.data_transfer_subscriber_fee_percent).into()),
        );
    }
    if let Some(var_1) = &input.description {
        object.key("description").string(var_1.as_str());
    }
    if let Some(var_2) = &input.encryption {
        #[allow(unused_mut)]
        let mut object_3 = object.key("encryption").start_object();
        crate::protocol_serde::shape_encryption::ser_encryption(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.entitlement_status {
        object.key("entitlementStatus").string(var_4.as_str());
    }
    if let Some(var_5) = &input.name {
        object.key("name").string(var_5.as_str());
    }
    if let Some(var_6) = &input.subscribers {
        let mut array_7 = object.key("subscribers").start_array();
        for item_8 in var_6 {
            {
                array_7.value().string(item_8.as_str());
            }
        }
        array_7.finish();
    }
    Ok(())
}
