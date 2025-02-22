// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_folder_headers(
    input: &crate::operation::update_folder::UpdateFolderInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_1) = &input.authentication_token {
        let formatted_2 = inner_1.as_str();
        if !formatted_2.is_empty() {
            let header_value = formatted_2;
            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                aws_smithy_http::operation::error::BuildError::invalid_field(
                    "authentication_token",
                    format!(
                        "`{}` cannot be used as a header value: {}",
                        &"*** Sensitive Data Redacted ***", err
                    ),
                )
            })?;
            builder = builder.header("Authentication", header_value);
        }
    }
    Ok(builder)
}

pub fn ser_update_folder_input(
    input: &crate::operation::update_folder::UpdateFolderInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_update_folder_input::ser_update_folder_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_folder_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::update_folder::UpdateFolderOutput,
    crate::operation::update_folder::UpdateFolderError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::update_folder::UpdateFolderError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::update_folder::UpdateFolderError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ConcurrentModificationException" => {
            crate::operation::update_folder::UpdateFolderError::ConcurrentModificationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ConcurrentModificationExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_concurrent_modification_exception::de_concurrent_modification_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_folder::UpdateFolderError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ConflictingOperationException" => {
            crate::operation::update_folder::UpdateFolderError::ConflictingOperationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ConflictingOperationExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_conflicting_operation_exception::de_conflicting_operation_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_folder::UpdateFolderError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "EntityAlreadyExistsException" => {
            crate::operation::update_folder::UpdateFolderError::EntityAlreadyExistsException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::EntityAlreadyExistsExceptionBuilder::default(
                        );
                    let _ = response;
                    output = crate::protocol_serde::shape_entity_already_exists_exception::de_entity_already_exists_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_folder::UpdateFolderError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "EntityNotExistsException" => {
            crate::operation::update_folder::UpdateFolderError::EntityNotExistsException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::EntityNotExistsExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_entity_not_exists_exception::de_entity_not_exists_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_folder::UpdateFolderError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "FailedDependencyException" => {
            crate::operation::update_folder::UpdateFolderError::FailedDependencyException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::FailedDependencyExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_failed_dependency_exception::de_failed_dependency_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_folder::UpdateFolderError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "LimitExceededException" => {
            crate::operation::update_folder::UpdateFolderError::LimitExceededException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::LimitExceededExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_folder::UpdateFolderError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ProhibitedStateException" => {
            crate::operation::update_folder::UpdateFolderError::ProhibitedStateException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ProhibitedStateExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_prohibited_state_exception::de_prohibited_state_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_folder::UpdateFolderError::unhandled)?;
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
            crate::operation::update_folder::UpdateFolderError::ServiceUnavailableException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ServiceUnavailableExceptionBuilder::default(
                        );
                    let _ = response;
                    output = crate::protocol_serde::shape_service_unavailable_exception::de_service_unavailable_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_folder::UpdateFolderError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "UnauthorizedOperationException" => {
            crate::operation::update_folder::UpdateFolderError::UnauthorizedOperationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::UnauthorizedOperationExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unauthorized_operation_exception::de_unauthorized_operation_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_folder::UpdateFolderError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "UnauthorizedResourceAccessException" => {
            crate::operation::update_folder::UpdateFolderError::UnauthorizedResourceAccessException(
                {
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::UnauthorizedResourceAccessExceptionBuilder::default();
                        let _ = response;
                        output = crate::protocol_serde::shape_unauthorized_resource_access_exception::de_unauthorized_resource_access_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_folder::UpdateFolderError::unhandled)?;
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
        _ => crate::operation::update_folder::UpdateFolderError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_folder_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::update_folder::UpdateFolderOutput,
    crate::operation::update_folder::UpdateFolderError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::update_folder::builders::UpdateFolderOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
