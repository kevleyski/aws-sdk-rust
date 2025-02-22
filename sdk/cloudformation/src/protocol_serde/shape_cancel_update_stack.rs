// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_cancel_update_stack_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::cancel_update_stack::CancelUpdateStackOutput,
    crate::operation::cancel_update_stack::CancelUpdateStackError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::cancel_update_stack::CancelUpdateStackError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(
                crate::operation::cancel_update_stack::CancelUpdateStackError::unhandled(generic),
            )
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "TokenAlreadyExistsException" => crate::operation::cancel_update_stack::CancelUpdateStackError::TokenAlreadyExistsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::TokenAlreadyExistsExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_token_already_exists_exception::de_token_already_exists_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::cancel_update_stack::CancelUpdateStackError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::cancel_update_stack::CancelUpdateStackError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_cancel_update_stack_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::cancel_update_stack::CancelUpdateStackOutput,
    crate::operation::cancel_update_stack::CancelUpdateStackError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::cancel_update_stack::builders::CancelUpdateStackOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
