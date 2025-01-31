// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_add_tags_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::add_tags::AddTagsOutput,
    crate::operation::add_tags::AddTagsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::add_tags::AddTagsError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::add_tags::AddTagsError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "DuplicateTagKeys" => {
            crate::operation::add_tags::AddTagsError::DuplicateTagKeysException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::DuplicateTagKeysExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_duplicate_tag_keys_exception::de_duplicate_tag_keys_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::add_tags::AddTagsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ListenerNotFound" => {
            crate::operation::add_tags::AddTagsError::ListenerNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ListenerNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_listener_not_found_exception::de_listener_not_found_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::add_tags::AddTagsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "LoadBalancerNotFound" => {
            crate::operation::add_tags::AddTagsError::LoadBalancerNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::LoadBalancerNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_load_balancer_not_found_exception::de_load_balancer_not_found_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::add_tags::AddTagsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "RuleNotFound" => crate::operation::add_tags::AddTagsError::RuleNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output =
                    crate::types::error::builders::RuleNotFoundExceptionBuilder::default();
                let _ = response;
                output = crate::protocol_serde::shape_rule_not_found_exception::de_rule_not_found_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::add_tags::AddTagsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "TargetGroupNotFound" => {
            crate::operation::add_tags::AddTagsError::TargetGroupNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::TargetGroupNotFoundExceptionBuilder::default(
                        );
                    let _ = response;
                    output = crate::protocol_serde::shape_target_group_not_found_exception::de_target_group_not_found_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::add_tags::AddTagsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "TooManyTags" => crate::operation::add_tags::AddTagsError::TooManyTagsException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output =
                    crate::types::error::builders::TooManyTagsExceptionBuilder::default();
                let _ = response;
                output = crate::protocol_serde::shape_too_many_tags_exception::de_too_many_tags_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::add_tags::AddTagsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::add_tags::AddTagsError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_add_tags_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::add_tags::AddTagsOutput,
    crate::operation::add_tags::AddTagsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::add_tags::builders::AddTagsOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
