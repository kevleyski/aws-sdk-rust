// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_object_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::delete_object::DeleteObjectOutput,
    crate::operation::delete_object::DeleteObjectError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::delete_object::DeleteObjectError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::delete_object::DeleteObjectError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => {
            crate::operation::delete_object::DeleteObjectError::AccessDeniedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_object::DeleteObjectError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "IllegalArgumentException" => {
            crate::operation::delete_object::DeleteObjectError::IllegalArgumentException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::IllegalArgumentExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_illegal_argument_exception::de_illegal_argument_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_object::DeleteObjectError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ResourceNotFoundException" => {
            crate::operation::delete_object::DeleteObjectError::ResourceNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_object::DeleteObjectError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "RetryableException" => {
            crate::operation::delete_object::DeleteObjectError::RetryableException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::RetryableExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_retryable_exception::de_retryable_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_object::DeleteObjectError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ServiceInternalException" => {
            crate::operation::delete_object::DeleteObjectError::ServiceInternalException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ServiceInternalExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_internal_exception::de_service_internal_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_object::DeleteObjectError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ServiceUnavailableException" => {
            crate::operation::delete_object::DeleteObjectError::ServiceUnavailableException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ServiceUnavailableExceptionBuilder::default(
                        );
                    let _ = response;
                    output = crate::protocol_serde::shape_service_unavailable_exception::de_service_unavailable_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_object::DeleteObjectError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ThrottlingException" => {
            crate::operation::delete_object::DeleteObjectError::ThrottlingException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ThrottlingExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_object::DeleteObjectError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::delete_object::DeleteObjectError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_object_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::delete_object::DeleteObjectOutput,
    crate::operation::delete_object::DeleteObjectError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::delete_object::builders::DeleteObjectOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
