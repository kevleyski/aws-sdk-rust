// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) mod shape_get_entitlements;

pub fn parse_http_error_metadata(
    response: &http::Response<bytes::Bytes>,
) -> Result<
    aws_smithy_types::error::metadata::Builder,
    aws_smithy_json::deserialize::error::DeserializeError,
> {
    crate::json_errors::parse_error_metadata(response.body(), response.headers())
}

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub(crate) mod shape_get_entitlements_input;

pub(crate) mod shape_internal_service_error_exception;

pub(crate) mod shape_invalid_parameter_exception;

pub(crate) mod shape_throttling_exception;

pub(crate) mod shape_entitlement_list;

pub(crate) mod shape_entitlement;

pub(crate) mod shape_entitlement_value;
