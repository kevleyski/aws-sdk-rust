// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_remove_endpoints_input(
    input: &crate::operation::remove_endpoints::RemoveEndpointsInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_remove_endpoints_input::ser_remove_endpoints_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_remove_endpoints_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::remove_endpoints::RemoveEndpointsOutput,
    crate::operation::remove_endpoints::RemoveEndpointsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::remove_endpoints::RemoveEndpointsError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(
                crate::operation::remove_endpoints::RemoveEndpointsError::unhandled(generic),
            )
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => {
            crate::operation::remove_endpoints::RemoveEndpointsError::AccessDeniedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::remove_endpoints::RemoveEndpointsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "EndpointGroupNotFoundException" => {
            crate::operation::remove_endpoints::RemoveEndpointsError::EndpointGroupNotFoundException(
                {
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::EndpointGroupNotFoundExceptionBuilder::default();
                        let _ = response;
                        output = crate::protocol_serde::shape_endpoint_group_not_found_exception::de_endpoint_group_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::remove_endpoints::RemoveEndpointsError::unhandled)?;
                        let output = output.meta(generic);
                        output.build()
                    };
                    if tmp.message.is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                },
            )
        }
        "InternalServiceErrorException" => {
            crate::operation::remove_endpoints::RemoveEndpointsError::InternalServiceErrorException(
                {
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalServiceErrorExceptionBuilder::default();
                        let _ = response;
                        output = crate::protocol_serde::shape_internal_service_error_exception::de_internal_service_error_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::remove_endpoints::RemoveEndpointsError::unhandled)?;
                        let output = output.meta(generic);
                        output.build()
                    };
                    if tmp.message.is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                },
            )
        }
        "InvalidArgumentException" => {
            crate::operation::remove_endpoints::RemoveEndpointsError::InvalidArgumentException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::InvalidArgumentExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_argument_exception::de_invalid_argument_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::remove_endpoints::RemoveEndpointsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "TransactionInProgressException" => {
            crate::operation::remove_endpoints::RemoveEndpointsError::TransactionInProgressException(
                {
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::TransactionInProgressExceptionBuilder::default();
                        let _ = response;
                        output = crate::protocol_serde::shape_transaction_in_progress_exception::de_transaction_in_progress_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::remove_endpoints::RemoveEndpointsError::unhandled)?;
                        let output = output.meta(generic);
                        output.build()
                    };
                    if tmp.message.is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                },
            )
        }
        _ => crate::operation::remove_endpoints::RemoveEndpointsError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_remove_endpoints_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::remove_endpoints::RemoveEndpointsOutput,
    crate::operation::remove_endpoints::RemoveEndpointsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::remove_endpoints::builders::RemoveEndpointsOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
