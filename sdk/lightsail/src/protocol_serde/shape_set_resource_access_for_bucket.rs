// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_set_resource_access_for_bucket_input(
    input: &crate::operation::set_resource_access_for_bucket::SetResourceAccessForBucketInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_set_resource_access_for_bucket_input::ser_set_resource_access_for_bucket_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_set_resource_access_for_bucket_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::set_resource_access_for_bucket::SetResourceAccessForBucketOutput,
    crate::operation::set_resource_access_for_bucket::SetResourceAccessForBucketError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::operation::set_resource_access_for_bucket::SetResourceAccessForBucketError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::operation::set_resource_access_for_bucket::SetResourceAccessForBucketError::unhandled(generic))
                            };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::operation::set_resource_access_for_bucket::SetResourceAccessForBucketError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::set_resource_access_for_bucket::SetResourceAccessForBucketError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidInputException" => crate::operation::set_resource_access_for_bucket::SetResourceAccessForBucketError::InvalidInputException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidInputExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_input_exception::de_invalid_input_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::set_resource_access_for_bucket::SetResourceAccessForBucketError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NotFoundException" => crate::operation::set_resource_access_for_bucket::SetResourceAccessForBucketError::NotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::NotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_not_found_exception::de_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::set_resource_access_for_bucket::SetResourceAccessForBucketError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceException" => crate::operation::set_resource_access_for_bucket::SetResourceAccessForBucketError::ServiceException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ServiceExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_exception::de_service_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::set_resource_access_for_bucket::SetResourceAccessForBucketError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnauthenticatedException" => crate::operation::set_resource_access_for_bucket::SetResourceAccessForBucketError::UnauthenticatedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::UnauthenticatedExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unauthenticated_exception::de_unauthenticated_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::set_resource_access_for_bucket::SetResourceAccessForBucketError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::set_resource_access_for_bucket::SetResourceAccessForBucketError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_set_resource_access_for_bucket_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::set_resource_access_for_bucket::SetResourceAccessForBucketOutput,
    crate::operation::set_resource_access_for_bucket::SetResourceAccessForBucketError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::set_resource_access_for_bucket::builders::SetResourceAccessForBucketOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_set_resource_access_for_bucket::de_set_resource_access_for_bucket(response.body().as_ref(), output).map_err(crate::operation::set_resource_access_for_bucket::SetResourceAccessForBucketError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_set_resource_access_for_bucket(value: &[u8], mut builder: crate::operation::set_resource_access_for_bucket::builders::SetResourceAccessForBucketOutputBuilder) -> Result<crate::operation::set_resource_access_for_bucket::builders::SetResourceAccessForBucketOutputBuilder, aws_smithy_json::deserialize::error::DeserializeError>{
    let mut tokens_owned =
        aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value))
            .peekable();
    let tokens = &mut tokens_owned;
    aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "operations" => {
                        builder = builder.set_operations(
                            crate::protocol_serde::shape_operation_list::de_operation_list(tokens)?,
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            other => {
                return Err(
                    aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                        "expected object key or end object, found: {:?}",
                        other
                    )),
                )
            }
        }
    }
    if tokens.next().is_some() {
        return Err(
            aws_smithy_json::deserialize::error::DeserializeError::custom(
                "found more JSON tokens after completing parsing",
            ),
        );
    }
    Ok(builder)
}
