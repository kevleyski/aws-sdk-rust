// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_monitor_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_monitor::CreateMonitorInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.client_token {
        object.key("ClientToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.internet_measurements_log_delivery {
        #[allow(unused_mut)]
        let mut object_3 = object.key("InternetMeasurementsLogDelivery").start_object();
        crate::protocol_serde::shape_internet_measurements_log_delivery::ser_internet_measurements_log_delivery(&mut object_3, var_2)?;
        object_3.finish();
    }
    {
        object.key("MaxCityNetworksToMonitor").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.max_city_networks_to_monitor).into()),
        );
    }
    if let Some(var_4) = &input.monitor_name {
        object.key("MonitorName").string(var_4.as_str());
    }
    if let Some(var_5) = &input.resources {
        let mut array_6 = object.key("Resources").start_array();
        for item_7 in var_5 {
            {
                array_6.value().string(item_7.as_str());
            }
        }
        array_6.finish();
    }
    if let Some(var_8) = &input.tags {
        #[allow(unused_mut)]
        let mut object_9 = object.key("Tags").start_object();
        for (key_10, value_11) in var_8 {
            {
                object_9.key(key_10.as_str()).string(value_11.as_str());
            }
        }
        object_9.finish();
    }
    Ok(())
}
