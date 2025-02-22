// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_accelerator_input(
    input: &crate::operation::delete_accelerator::DeleteAcceleratorInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_delete_accelerator_input::ser_delete_accelerator_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_accelerator_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::delete_accelerator::DeleteAcceleratorOutput,
    crate::operation::delete_accelerator::DeleteAcceleratorError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::delete_accelerator::DeleteAcceleratorError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(
                crate::operation::delete_accelerator::DeleteAcceleratorError::unhandled(generic),
            )
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AcceleratorNotDisabledException" => crate::operation::delete_accelerator::DeleteAcceleratorError::AcceleratorNotDisabledException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::AcceleratorNotDisabledExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_accelerator_not_disabled_exception::de_accelerator_not_disabled_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_accelerator::DeleteAcceleratorError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "AcceleratorNotFoundException" => crate::operation::delete_accelerator::DeleteAcceleratorError::AcceleratorNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::AcceleratorNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_accelerator_not_found_exception::de_accelerator_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_accelerator::DeleteAcceleratorError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "AssociatedListenerFoundException" => crate::operation::delete_accelerator::DeleteAcceleratorError::AssociatedListenerFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::AssociatedListenerFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_associated_listener_found_exception::de_associated_listener_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_accelerator::DeleteAcceleratorError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalServiceErrorException" => crate::operation::delete_accelerator::DeleteAcceleratorError::InternalServiceErrorException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalServiceErrorExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_service_error_exception::de_internal_service_error_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_accelerator::DeleteAcceleratorError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidArgumentException" => crate::operation::delete_accelerator::DeleteAcceleratorError::InvalidArgumentException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidArgumentExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_argument_exception::de_invalid_argument_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_accelerator::DeleteAcceleratorError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::delete_accelerator::DeleteAcceleratorError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_accelerator_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::delete_accelerator::DeleteAcceleratorOutput,
    crate::operation::delete_accelerator::DeleteAcceleratorError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::delete_accelerator::builders::DeleteAcceleratorOutputBuilder::default(
            );
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
