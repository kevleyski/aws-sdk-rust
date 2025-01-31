// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_stop_data_collection_by_agent_ids_input(
    input: &crate::operation::stop_data_collection_by_agent_ids::StopDataCollectionByAgentIdsInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_stop_data_collection_by_agent_ids_input::ser_stop_data_collection_by_agent_ids_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_stop_data_collection_by_agent_ids_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::stop_data_collection_by_agent_ids::StopDataCollectionByAgentIdsOutput,
    crate::operation::stop_data_collection_by_agent_ids::StopDataCollectionByAgentIdsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::operation::stop_data_collection_by_agent_ids::StopDataCollectionByAgentIdsError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::operation::stop_data_collection_by_agent_ids::StopDataCollectionByAgentIdsError::unhandled(generic))
                            };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AuthorizationErrorException" => crate::operation::stop_data_collection_by_agent_ids::StopDataCollectionByAgentIdsError::AuthorizationErrorException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::AuthorizationErrorExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_authorization_error_exception::de_authorization_error_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::stop_data_collection_by_agent_ids::StopDataCollectionByAgentIdsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "HomeRegionNotSetException" => crate::operation::stop_data_collection_by_agent_ids::StopDataCollectionByAgentIdsError::HomeRegionNotSetException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::HomeRegionNotSetExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_home_region_not_set_exception::de_home_region_not_set_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::stop_data_collection_by_agent_ids::StopDataCollectionByAgentIdsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidParameterException" => crate::operation::stop_data_collection_by_agent_ids::StopDataCollectionByAgentIdsError::InvalidParameterException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidParameterExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::stop_data_collection_by_agent_ids::StopDataCollectionByAgentIdsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidParameterValueException" => crate::operation::stop_data_collection_by_agent_ids::StopDataCollectionByAgentIdsError::InvalidParameterValueException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidParameterValueExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_value_exception::de_invalid_parameter_value_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::stop_data_collection_by_agent_ids::StopDataCollectionByAgentIdsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServerInternalErrorException" => crate::operation::stop_data_collection_by_agent_ids::StopDataCollectionByAgentIdsError::ServerInternalErrorException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ServerInternalErrorExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_server_internal_error_exception::de_server_internal_error_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::stop_data_collection_by_agent_ids::StopDataCollectionByAgentIdsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::stop_data_collection_by_agent_ids::StopDataCollectionByAgentIdsError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_stop_data_collection_by_agent_ids_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::stop_data_collection_by_agent_ids::StopDataCollectionByAgentIdsOutput,
    crate::operation::stop_data_collection_by_agent_ids::StopDataCollectionByAgentIdsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::stop_data_collection_by_agent_ids::builders::StopDataCollectionByAgentIdsOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_stop_data_collection_by_agent_ids::de_stop_data_collection_by_agent_ids(response.body().as_ref(), output).map_err(crate::operation::stop_data_collection_by_agent_ids::StopDataCollectionByAgentIdsError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_stop_data_collection_by_agent_ids(value: &[u8], mut builder: crate::operation::stop_data_collection_by_agent_ids::builders::StopDataCollectionByAgentIdsOutputBuilder) -> Result<crate::operation::stop_data_collection_by_agent_ids::builders::StopDataCollectionByAgentIdsOutputBuilder, aws_smithy_json::deserialize::error::DeserializeError>{
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
                    "agentsConfigurationStatus" => {
                        builder = builder.set_agents_configuration_status(
                            crate::protocol_serde::shape_agent_configuration_status_list::de_agent_configuration_status_list(tokens)?
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
