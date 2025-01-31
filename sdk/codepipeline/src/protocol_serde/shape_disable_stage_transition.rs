// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_disable_stage_transition_input(
    input: &crate::operation::disable_stage_transition::DisableStageTransitionInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_disable_stage_transition_input::ser_disable_stage_transition_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_disable_stage_transition_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::disable_stage_transition::DisableStageTransitionOutput,
    crate::operation::disable_stage_transition::DisableStageTransitionError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(
        crate::operation::disable_stage_transition::DisableStageTransitionError::unhandled,
    )?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code =
        match generic.code() {
            Some(code) => code,
            None => return Err(
                crate::operation::disable_stage_transition::DisableStageTransitionError::unhandled(
                    generic,
                ),
            ),
        };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "PipelineNotFoundException" => crate::operation::disable_stage_transition::DisableStageTransitionError::PipelineNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::PipelineNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_pipeline_not_found_exception::de_pipeline_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::disable_stage_transition::DisableStageTransitionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "StageNotFoundException" => crate::operation::disable_stage_transition::DisableStageTransitionError::StageNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::StageNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_stage_not_found_exception::de_stage_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::disable_stage_transition::DisableStageTransitionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ValidationException" => crate::operation::disable_stage_transition::DisableStageTransitionError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ValidationExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::disable_stage_transition::DisableStageTransitionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::disable_stage_transition::DisableStageTransitionError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_disable_stage_transition_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::disable_stage_transition::DisableStageTransitionOutput,
    crate::operation::disable_stage_transition::DisableStageTransitionError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::disable_stage_transition::builders::DisableStageTransitionOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
