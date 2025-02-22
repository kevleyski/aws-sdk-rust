// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_remove_tags_from_resource_input(
    input: &crate::operation::remove_tags_from_resource::RemoveTagsFromResourceInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_remove_tags_from_resource_input::ser_remove_tags_from_resource_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_remove_tags_from_resource_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::remove_tags_from_resource::RemoveTagsFromResourceOutput,
    crate::operation::remove_tags_from_resource::RemoveTagsFromResourceError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(
        crate::operation::remove_tags_from_resource::RemoveTagsFromResourceError::unhandled,
    )?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code =
        match generic.code() {
            Some(code) => code,
            None => return Err(
                crate::operation::remove_tags_from_resource::RemoveTagsFromResourceError::unhandled(
                    generic,
                ),
            ),
        };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalServerError" => crate::operation::remove_tags_from_resource::RemoveTagsFromResourceError::InternalServerError({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalServerErrorBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_server_error::de_internal_server_error_json_err(response.body().as_ref(), output).map_err(crate::operation::remove_tags_from_resource::RemoveTagsFromResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidResourceId" => crate::operation::remove_tags_from_resource::RemoveTagsFromResourceError::InvalidResourceId({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidResourceIdBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_resource_id::de_invalid_resource_id_json_err(response.body().as_ref(), output).map_err(crate::operation::remove_tags_from_resource::RemoveTagsFromResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidResourceType" => crate::operation::remove_tags_from_resource::RemoveTagsFromResourceError::InvalidResourceType({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidResourceTypeBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_resource_type::de_invalid_resource_type_json_err(response.body().as_ref(), output).map_err(crate::operation::remove_tags_from_resource::RemoveTagsFromResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TooManyUpdates" => crate::operation::remove_tags_from_resource::RemoveTagsFromResourceError::TooManyUpdates({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::TooManyUpdatesBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_too_many_updates::de_too_many_updates_json_err(response.body().as_ref(), output).map_err(crate::operation::remove_tags_from_resource::RemoveTagsFromResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::remove_tags_from_resource::RemoveTagsFromResourceError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_remove_tags_from_resource_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::remove_tags_from_resource::RemoveTagsFromResourceOutput,
    crate::operation::remove_tags_from_resource::RemoveTagsFromResourceError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::remove_tags_from_resource::builders::RemoveTagsFromResourceOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
