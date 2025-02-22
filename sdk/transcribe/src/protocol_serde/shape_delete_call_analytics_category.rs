// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_call_analytics_category_input(
    input: &crate::operation::delete_call_analytics_category::DeleteCallAnalyticsCategoryInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_delete_call_analytics_category_input::ser_delete_call_analytics_category_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_call_analytics_category_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::delete_call_analytics_category::DeleteCallAnalyticsCategoryOutput,
    crate::operation::delete_call_analytics_category::DeleteCallAnalyticsCategoryError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::operation::delete_call_analytics_category::DeleteCallAnalyticsCategoryError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::operation::delete_call_analytics_category::DeleteCallAnalyticsCategoryError::unhandled(generic))
                            };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "BadRequestException" => crate::operation::delete_call_analytics_category::DeleteCallAnalyticsCategoryError::BadRequestException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::BadRequestExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_bad_request_exception::de_bad_request_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_call_analytics_category::DeleteCallAnalyticsCategoryError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalFailureException" => crate::operation::delete_call_analytics_category::DeleteCallAnalyticsCategoryError::InternalFailureException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalFailureExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_failure_exception::de_internal_failure_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_call_analytics_category::DeleteCallAnalyticsCategoryError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "LimitExceededException" => crate::operation::delete_call_analytics_category::DeleteCallAnalyticsCategoryError::LimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::LimitExceededExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_call_analytics_category::DeleteCallAnalyticsCategoryError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NotFoundException" => crate::operation::delete_call_analytics_category::DeleteCallAnalyticsCategoryError::NotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::NotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_not_found_exception::de_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_call_analytics_category::DeleteCallAnalyticsCategoryError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::delete_call_analytics_category::DeleteCallAnalyticsCategoryError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_call_analytics_category_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::delete_call_analytics_category::DeleteCallAnalyticsCategoryOutput,
    crate::operation::delete_call_analytics_category::DeleteCallAnalyticsCategoryError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::delete_call_analytics_category::builders::DeleteCallAnalyticsCategoryOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
