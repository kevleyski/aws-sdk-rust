// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_claim_device_input(
    input: &crate::operation::claim_device::ClaimDeviceInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_claim_device_input::ser_claim_device_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_claim_device_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::claim_device::ClaimDeviceOutput,
    crate::operation::claim_device::ClaimDeviceError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::claim_device::ClaimDeviceError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::claim_device::ClaimDeviceError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "BadGatewayException" => {
            crate::operation::claim_device::ClaimDeviceError::BadGatewayException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::BadGatewayExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_bad_gateway_exception::de_bad_gateway_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::claim_device::ClaimDeviceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "BadRequestException" => {
            crate::operation::claim_device::ClaimDeviceError::BadRequestException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::BadRequestExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_bad_request_exception::de_bad_request_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::claim_device::ClaimDeviceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ForbiddenException" => {
            crate::operation::claim_device::ClaimDeviceError::ForbiddenException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ForbiddenExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_forbidden_exception::de_forbidden_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::claim_device::ClaimDeviceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "GatewayTimeoutException" => {
            crate::operation::claim_device::ClaimDeviceError::GatewayTimeoutException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::GatewayTimeoutExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_gateway_timeout_exception::de_gateway_timeout_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::claim_device::ClaimDeviceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InternalServerErrorException" => {
            crate::operation::claim_device::ClaimDeviceError::InternalServerErrorException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::InternalServerErrorExceptionBuilder::default(
                        );
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_server_error_exception::de_internal_server_error_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::claim_device::ClaimDeviceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "NotFoundException" => {
            crate::operation::claim_device::ClaimDeviceError::NotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::NotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_not_found_exception::de_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::claim_device::ClaimDeviceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "TooManyRequestsException" => {
            crate::operation::claim_device::ClaimDeviceError::TooManyRequestsException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::claim_device::ClaimDeviceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "UnprocessableEntityException" => {
            crate::operation::claim_device::ClaimDeviceError::UnprocessableEntityException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::UnprocessableEntityExceptionBuilder::default(
                        );
                    let _ = response;
                    output = crate::protocol_serde::shape_unprocessable_entity_exception::de_unprocessable_entity_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::claim_device::ClaimDeviceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::claim_device::ClaimDeviceError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_claim_device_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::claim_device::ClaimDeviceOutput,
    crate::operation::claim_device::ClaimDeviceError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::claim_device::builders::ClaimDeviceOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
