// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_block_public_access_configuration_input(
    _input: &crate::operation::get_block_public_access_configuration::GetBlockPublicAccessConfigurationInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    Ok(aws_smithy_http::body::SdkBody::from("{}"))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_block_public_access_configuration_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::operation::get_block_public_access_configuration::GetBlockPublicAccessConfigurationOutput, crate::operation::get_block_public_access_configuration::GetBlockPublicAccessConfigurationError>{
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::operation::get_block_public_access_configuration::GetBlockPublicAccessConfigurationError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::operation::get_block_public_access_configuration::GetBlockPublicAccessConfigurationError::unhandled(generic))
                            };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalServerException" => crate::operation::get_block_public_access_configuration::GetBlockPublicAccessConfigurationError::InternalServerException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalServerExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::get_block_public_access_configuration::GetBlockPublicAccessConfigurationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidRequestException" => crate::operation::get_block_public_access_configuration::GetBlockPublicAccessConfigurationError::InvalidRequestException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidRequestExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::get_block_public_access_configuration::GetBlockPublicAccessConfigurationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::get_block_public_access_configuration::GetBlockPublicAccessConfigurationError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_block_public_access_configuration_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::operation::get_block_public_access_configuration::GetBlockPublicAccessConfigurationOutput, crate::operation::get_block_public_access_configuration::GetBlockPublicAccessConfigurationError>{
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_block_public_access_configuration::builders::GetBlockPublicAccessConfigurationOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_get_block_public_access_configuration::de_get_block_public_access_configuration(response.body().as_ref(), output).map_err(crate::operation::get_block_public_access_configuration::GetBlockPublicAccessConfigurationError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_get_block_public_access_configuration(value: &[u8], mut builder: crate::operation::get_block_public_access_configuration::builders::GetBlockPublicAccessConfigurationOutputBuilder) -> Result<crate::operation::get_block_public_access_configuration::builders::GetBlockPublicAccessConfigurationOutputBuilder, aws_smithy_json::deserialize::error::DeserializeError>{
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
                    "BlockPublicAccessConfiguration" => {
                        builder = builder.set_block_public_access_configuration(
                            crate::protocol_serde::shape_block_public_access_configuration::de_block_public_access_configuration(tokens)?
                        );
                    }
                    "BlockPublicAccessConfigurationMetadata" => {
                        builder = builder.set_block_public_access_configuration_metadata(
                            crate::protocol_serde::shape_block_public_access_configuration_metadata::de_block_public_access_configuration_metadata(tokens)?
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
