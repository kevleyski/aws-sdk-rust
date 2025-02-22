// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_revoke_domain_access_input(
    input: &crate::operation::revoke_domain_access::RevokeDomainAccessInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_revoke_domain_access_input::ser_revoke_domain_access_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_revoke_domain_access_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::revoke_domain_access::RevokeDomainAccessOutput,
    crate::operation::revoke_domain_access::RevokeDomainAccessError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::revoke_domain_access::RevokeDomainAccessError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(
                crate::operation::revoke_domain_access::RevokeDomainAccessError::unhandled(generic),
            )
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalServerErrorException" => crate::operation::revoke_domain_access::RevokeDomainAccessError::InternalServerErrorException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalServerErrorExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_server_error_exception::de_internal_server_error_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::revoke_domain_access::RevokeDomainAccessError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidRequestException" => crate::operation::revoke_domain_access::RevokeDomainAccessError::InvalidRequestException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidRequestExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::revoke_domain_access::RevokeDomainAccessError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::revoke_domain_access::RevokeDomainAccessError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::revoke_domain_access::RevokeDomainAccessError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TooManyRequestsException" => crate::operation::revoke_domain_access::RevokeDomainAccessError::TooManyRequestsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::revoke_domain_access::RevokeDomainAccessError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnauthorizedException" => crate::operation::revoke_domain_access::RevokeDomainAccessError::UnauthorizedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::UnauthorizedExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unauthorized_exception::de_unauthorized_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::revoke_domain_access::RevokeDomainAccessError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::revoke_domain_access::RevokeDomainAccessError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_revoke_domain_access_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::revoke_domain_access::RevokeDomainAccessOutput,
    crate::operation::revoke_domain_access::RevokeDomainAccessError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::revoke_domain_access::builders::RevokeDomainAccessOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
