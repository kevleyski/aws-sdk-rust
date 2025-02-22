// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_tag_resource_input(
    input: &crate::operation::tag_resource::TagResourceInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_tag_resource_input::ser_tag_resource_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_tag_resource_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::tag_resource::TagResourceOutput,
    crate::operation::tag_resource::TagResourceError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::tag_resource::TagResourceError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::tag_resource::TagResourceError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "DirectConnectClientException" => {
            crate::operation::tag_resource::TagResourceError::DirectConnectClientException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::DirectConnectClientExceptionBuilder::default(
                        );
                    let _ = response;
                    output = crate::protocol_serde::shape_direct_connect_client_exception::de_direct_connect_client_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::tag_resource::TagResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "DirectConnectServerException" => {
            crate::operation::tag_resource::TagResourceError::DirectConnectServerException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::DirectConnectServerExceptionBuilder::default(
                        );
                    let _ = response;
                    output = crate::protocol_serde::shape_direct_connect_server_exception::de_direct_connect_server_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::tag_resource::TagResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "DuplicateTagKeysException" => {
            crate::operation::tag_resource::TagResourceError::DuplicateTagKeysException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::DuplicateTagKeysExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_duplicate_tag_keys_exception::de_duplicate_tag_keys_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::tag_resource::TagResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "TooManyTagsException" => {
            crate::operation::tag_resource::TagResourceError::TooManyTagsException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::TooManyTagsExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_too_many_tags_exception::de_too_many_tags_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::tag_resource::TagResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::tag_resource::TagResourceError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_tag_resource_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::tag_resource::TagResourceOutput,
    crate::operation::tag_resource::TagResourceError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::tag_resource::builders::TagResourceOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
