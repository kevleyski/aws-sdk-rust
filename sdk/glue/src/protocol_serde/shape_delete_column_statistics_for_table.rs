// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_column_statistics_for_table_input(
    input: &crate::operation::delete_column_statistics_for_table::DeleteColumnStatisticsForTableInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_delete_column_statistics_for_table_input::ser_delete_column_statistics_for_table_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_column_statistics_for_table_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::delete_column_statistics_for_table::DeleteColumnStatisticsForTableOutput,
    crate::operation::delete_column_statistics_for_table::DeleteColumnStatisticsForTableError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::operation::delete_column_statistics_for_table::DeleteColumnStatisticsForTableError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::operation::delete_column_statistics_for_table::DeleteColumnStatisticsForTableError::unhandled(generic))
                            };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "EntityNotFoundException" => crate::operation::delete_column_statistics_for_table::DeleteColumnStatisticsForTableError::EntityNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::EntityNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_entity_not_found_exception::de_entity_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_column_statistics_for_table::DeleteColumnStatisticsForTableError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "GlueEncryptionException" => crate::operation::delete_column_statistics_for_table::DeleteColumnStatisticsForTableError::GlueEncryptionException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::GlueEncryptionExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_glue_encryption_exception::de_glue_encryption_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_column_statistics_for_table::DeleteColumnStatisticsForTableError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalServiceException" => crate::operation::delete_column_statistics_for_table::DeleteColumnStatisticsForTableError::InternalServiceException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalServiceExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_service_exception::de_internal_service_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_column_statistics_for_table::DeleteColumnStatisticsForTableError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidInputException" => crate::operation::delete_column_statistics_for_table::DeleteColumnStatisticsForTableError::InvalidInputException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidInputExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_input_exception::de_invalid_input_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_column_statistics_for_table::DeleteColumnStatisticsForTableError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "OperationTimeoutException" => crate::operation::delete_column_statistics_for_table::DeleteColumnStatisticsForTableError::OperationTimeoutException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::OperationTimeoutExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_operation_timeout_exception::de_operation_timeout_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_column_statistics_for_table::DeleteColumnStatisticsForTableError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::delete_column_statistics_for_table::DeleteColumnStatisticsForTableError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_column_statistics_for_table_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::delete_column_statistics_for_table::DeleteColumnStatisticsForTableOutput,
    crate::operation::delete_column_statistics_for_table::DeleteColumnStatisticsForTableError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::delete_column_statistics_for_table::builders::DeleteColumnStatisticsForTableOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
