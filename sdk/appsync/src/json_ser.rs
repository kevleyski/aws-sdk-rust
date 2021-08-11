// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_create_api_cache_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateApiCacheInput,
) {
    if let Some(var_1) = &input.api_caching_behavior {
        object.key("apiCachingBehavior").string(var_1.as_str());
    }
    if input.at_rest_encryption_enabled {
        object
            .key("atRestEncryptionEnabled")
            .boolean(input.at_rest_encryption_enabled);
    }
    if input.transit_encryption_enabled {
        object
            .key("transitEncryptionEnabled")
            .boolean(input.transit_encryption_enabled);
    }
    {
        object.key("ttl").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.ttl).into()),
        );
    }
    if let Some(var_2) = &input.r#type {
        object.key("type").string(var_2.as_str());
    }
}

pub fn serialize_structure_create_api_key_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateApiKeyInput,
) {
    if let Some(var_3) = &input.description {
        object.key("description").string(var_3);
    }
    if input.expires != 0 {
        object.key("expires").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.expires).into()),
        );
    }
}

pub fn serialize_structure_create_data_source_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateDataSourceInput,
) {
    if let Some(var_4) = &input.description {
        object.key("description").string(var_4);
    }
    if let Some(var_5) = &input.dynamodb_config {
        let mut object_6 = object.key("dynamodbConfig").start_object();
        crate::json_ser::serialize_structure_dynamodb_data_source_config(&mut object_6, var_5);
        object_6.finish();
    }
    if let Some(var_7) = &input.elasticsearch_config {
        let mut object_8 = object.key("elasticsearchConfig").start_object();
        crate::json_ser::serialize_structure_elasticsearch_data_source_config(&mut object_8, var_7);
        object_8.finish();
    }
    if let Some(var_9) = &input.http_config {
        let mut object_10 = object.key("httpConfig").start_object();
        crate::json_ser::serialize_structure_http_data_source_config(&mut object_10, var_9);
        object_10.finish();
    }
    if let Some(var_11) = &input.lambda_config {
        let mut object_12 = object.key("lambdaConfig").start_object();
        crate::json_ser::serialize_structure_lambda_data_source_config(&mut object_12, var_11);
        object_12.finish();
    }
    if let Some(var_13) = &input.name {
        object.key("name").string(var_13);
    }
    if let Some(var_14) = &input.relational_database_config {
        let mut object_15 = object.key("relationalDatabaseConfig").start_object();
        crate::json_ser::serialize_structure_relational_database_data_source_config(
            &mut object_15,
            var_14,
        );
        object_15.finish();
    }
    if let Some(var_16) = &input.service_role_arn {
        object.key("serviceRoleArn").string(var_16);
    }
    if let Some(var_17) = &input.r#type {
        object.key("type").string(var_17.as_str());
    }
}

pub fn serialize_structure_create_function_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateFunctionInput,
) {
    if let Some(var_18) = &input.data_source_name {
        object.key("dataSourceName").string(var_18);
    }
    if let Some(var_19) = &input.description {
        object.key("description").string(var_19);
    }
    if let Some(var_20) = &input.function_version {
        object.key("functionVersion").string(var_20);
    }
    if let Some(var_21) = &input.name {
        object.key("name").string(var_21);
    }
    if let Some(var_22) = &input.request_mapping_template {
        object.key("requestMappingTemplate").string(var_22);
    }
    if let Some(var_23) = &input.response_mapping_template {
        object.key("responseMappingTemplate").string(var_23);
    }
    if let Some(var_24) = &input.sync_config {
        let mut object_25 = object.key("syncConfig").start_object();
        crate::json_ser::serialize_structure_sync_config(&mut object_25, var_24);
        object_25.finish();
    }
}

pub fn serialize_structure_create_graphql_api_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateGraphqlApiInput,
) {
    if let Some(var_26) = &input.additional_authentication_providers {
        let mut array_27 = object
            .key("additionalAuthenticationProviders")
            .start_array();
        for item_28 in var_26 {
            {
                let mut object_29 = array_27.value().start_object();
                crate::json_ser::serialize_structure_additional_authentication_provider(
                    &mut object_29,
                    item_28,
                );
                object_29.finish();
            }
        }
        array_27.finish();
    }
    if let Some(var_30) = &input.authentication_type {
        object.key("authenticationType").string(var_30.as_str());
    }
    if let Some(var_31) = &input.lambda_authorizer_config {
        let mut object_32 = object.key("lambdaAuthorizerConfig").start_object();
        crate::json_ser::serialize_structure_lambda_authorizer_config(&mut object_32, var_31);
        object_32.finish();
    }
    if let Some(var_33) = &input.log_config {
        let mut object_34 = object.key("logConfig").start_object();
        crate::json_ser::serialize_structure_log_config(&mut object_34, var_33);
        object_34.finish();
    }
    if let Some(var_35) = &input.name {
        object.key("name").string(var_35);
    }
    if let Some(var_36) = &input.open_id_connect_config {
        let mut object_37 = object.key("openIDConnectConfig").start_object();
        crate::json_ser::serialize_structure_open_id_connect_config(&mut object_37, var_36);
        object_37.finish();
    }
    if let Some(var_38) = &input.tags {
        let mut object_39 = object.key("tags").start_object();
        for (key_40, value_41) in var_38 {
            {
                object_39.key(key_40).string(value_41);
            }
        }
        object_39.finish();
    }
    if let Some(var_42) = &input.user_pool_config {
        let mut object_43 = object.key("userPoolConfig").start_object();
        crate::json_ser::serialize_structure_user_pool_config(&mut object_43, var_42);
        object_43.finish();
    }
    if input.xray_enabled {
        object.key("xrayEnabled").boolean(input.xray_enabled);
    }
}

pub fn serialize_structure_create_resolver_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateResolverInput,
) {
    if let Some(var_44) = &input.caching_config {
        let mut object_45 = object.key("cachingConfig").start_object();
        crate::json_ser::serialize_structure_caching_config(&mut object_45, var_44);
        object_45.finish();
    }
    if let Some(var_46) = &input.data_source_name {
        object.key("dataSourceName").string(var_46);
    }
    if let Some(var_47) = &input.field_name {
        object.key("fieldName").string(var_47);
    }
    if let Some(var_48) = &input.kind {
        object.key("kind").string(var_48.as_str());
    }
    if let Some(var_49) = &input.pipeline_config {
        let mut object_50 = object.key("pipelineConfig").start_object();
        crate::json_ser::serialize_structure_pipeline_config(&mut object_50, var_49);
        object_50.finish();
    }
    if let Some(var_51) = &input.request_mapping_template {
        object.key("requestMappingTemplate").string(var_51);
    }
    if let Some(var_52) = &input.response_mapping_template {
        object.key("responseMappingTemplate").string(var_52);
    }
    if let Some(var_53) = &input.sync_config {
        let mut object_54 = object.key("syncConfig").start_object();
        crate::json_ser::serialize_structure_sync_config(&mut object_54, var_53);
        object_54.finish();
    }
}

pub fn serialize_structure_create_type_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateTypeInput,
) {
    if let Some(var_55) = &input.definition {
        object.key("definition").string(var_55);
    }
    if let Some(var_56) = &input.format {
        object.key("format").string(var_56.as_str());
    }
}

pub fn serialize_structure_start_schema_creation_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartSchemaCreationInput,
) {
    if let Some(var_57) = &input.definition {
        object
            .key("definition")
            .string_unchecked(&smithy_types::base64::encode(var_57));
    }
}

pub fn serialize_structure_tag_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) {
    if let Some(var_58) = &input.tags {
        let mut object_59 = object.key("tags").start_object();
        for (key_60, value_61) in var_58 {
            {
                object_59.key(key_60).string(value_61);
            }
        }
        object_59.finish();
    }
}

pub fn serialize_structure_update_api_cache_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateApiCacheInput,
) {
    if let Some(var_62) = &input.api_caching_behavior {
        object.key("apiCachingBehavior").string(var_62.as_str());
    }
    {
        object.key("ttl").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.ttl).into()),
        );
    }
    if let Some(var_63) = &input.r#type {
        object.key("type").string(var_63.as_str());
    }
}

pub fn serialize_structure_update_api_key_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateApiKeyInput,
) {
    if let Some(var_64) = &input.description {
        object.key("description").string(var_64);
    }
    if input.expires != 0 {
        object.key("expires").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.expires).into()),
        );
    }
}

pub fn serialize_structure_update_data_source_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateDataSourceInput,
) {
    if let Some(var_65) = &input.description {
        object.key("description").string(var_65);
    }
    if let Some(var_66) = &input.dynamodb_config {
        let mut object_67 = object.key("dynamodbConfig").start_object();
        crate::json_ser::serialize_structure_dynamodb_data_source_config(&mut object_67, var_66);
        object_67.finish();
    }
    if let Some(var_68) = &input.elasticsearch_config {
        let mut object_69 = object.key("elasticsearchConfig").start_object();
        crate::json_ser::serialize_structure_elasticsearch_data_source_config(
            &mut object_69,
            var_68,
        );
        object_69.finish();
    }
    if let Some(var_70) = &input.http_config {
        let mut object_71 = object.key("httpConfig").start_object();
        crate::json_ser::serialize_structure_http_data_source_config(&mut object_71, var_70);
        object_71.finish();
    }
    if let Some(var_72) = &input.lambda_config {
        let mut object_73 = object.key("lambdaConfig").start_object();
        crate::json_ser::serialize_structure_lambda_data_source_config(&mut object_73, var_72);
        object_73.finish();
    }
    if let Some(var_74) = &input.relational_database_config {
        let mut object_75 = object.key("relationalDatabaseConfig").start_object();
        crate::json_ser::serialize_structure_relational_database_data_source_config(
            &mut object_75,
            var_74,
        );
        object_75.finish();
    }
    if let Some(var_76) = &input.service_role_arn {
        object.key("serviceRoleArn").string(var_76);
    }
    if let Some(var_77) = &input.r#type {
        object.key("type").string(var_77.as_str());
    }
}

pub fn serialize_structure_update_function_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateFunctionInput,
) {
    if let Some(var_78) = &input.data_source_name {
        object.key("dataSourceName").string(var_78);
    }
    if let Some(var_79) = &input.description {
        object.key("description").string(var_79);
    }
    if let Some(var_80) = &input.function_version {
        object.key("functionVersion").string(var_80);
    }
    if let Some(var_81) = &input.name {
        object.key("name").string(var_81);
    }
    if let Some(var_82) = &input.request_mapping_template {
        object.key("requestMappingTemplate").string(var_82);
    }
    if let Some(var_83) = &input.response_mapping_template {
        object.key("responseMappingTemplate").string(var_83);
    }
    if let Some(var_84) = &input.sync_config {
        let mut object_85 = object.key("syncConfig").start_object();
        crate::json_ser::serialize_structure_sync_config(&mut object_85, var_84);
        object_85.finish();
    }
}

pub fn serialize_structure_update_graphql_api_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateGraphqlApiInput,
) {
    if let Some(var_86) = &input.additional_authentication_providers {
        let mut array_87 = object
            .key("additionalAuthenticationProviders")
            .start_array();
        for item_88 in var_86 {
            {
                let mut object_89 = array_87.value().start_object();
                crate::json_ser::serialize_structure_additional_authentication_provider(
                    &mut object_89,
                    item_88,
                );
                object_89.finish();
            }
        }
        array_87.finish();
    }
    if let Some(var_90) = &input.authentication_type {
        object.key("authenticationType").string(var_90.as_str());
    }
    if let Some(var_91) = &input.lambda_authorizer_config {
        let mut object_92 = object.key("lambdaAuthorizerConfig").start_object();
        crate::json_ser::serialize_structure_lambda_authorizer_config(&mut object_92, var_91);
        object_92.finish();
    }
    if let Some(var_93) = &input.log_config {
        let mut object_94 = object.key("logConfig").start_object();
        crate::json_ser::serialize_structure_log_config(&mut object_94, var_93);
        object_94.finish();
    }
    if let Some(var_95) = &input.name {
        object.key("name").string(var_95);
    }
    if let Some(var_96) = &input.open_id_connect_config {
        let mut object_97 = object.key("openIDConnectConfig").start_object();
        crate::json_ser::serialize_structure_open_id_connect_config(&mut object_97, var_96);
        object_97.finish();
    }
    if let Some(var_98) = &input.user_pool_config {
        let mut object_99 = object.key("userPoolConfig").start_object();
        crate::json_ser::serialize_structure_user_pool_config(&mut object_99, var_98);
        object_99.finish();
    }
    if input.xray_enabled {
        object.key("xrayEnabled").boolean(input.xray_enabled);
    }
}

pub fn serialize_structure_update_resolver_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateResolverInput,
) {
    if let Some(var_100) = &input.caching_config {
        let mut object_101 = object.key("cachingConfig").start_object();
        crate::json_ser::serialize_structure_caching_config(&mut object_101, var_100);
        object_101.finish();
    }
    if let Some(var_102) = &input.data_source_name {
        object.key("dataSourceName").string(var_102);
    }
    if let Some(var_103) = &input.kind {
        object.key("kind").string(var_103.as_str());
    }
    if let Some(var_104) = &input.pipeline_config {
        let mut object_105 = object.key("pipelineConfig").start_object();
        crate::json_ser::serialize_structure_pipeline_config(&mut object_105, var_104);
        object_105.finish();
    }
    if let Some(var_106) = &input.request_mapping_template {
        object.key("requestMappingTemplate").string(var_106);
    }
    if let Some(var_107) = &input.response_mapping_template {
        object.key("responseMappingTemplate").string(var_107);
    }
    if let Some(var_108) = &input.sync_config {
        let mut object_109 = object.key("syncConfig").start_object();
        crate::json_ser::serialize_structure_sync_config(&mut object_109, var_108);
        object_109.finish();
    }
}

pub fn serialize_structure_update_type_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateTypeInput,
) {
    if let Some(var_110) = &input.definition {
        object.key("definition").string(var_110);
    }
    if let Some(var_111) = &input.format {
        object.key("format").string(var_111.as_str());
    }
}

pub fn serialize_structure_dynamodb_data_source_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DynamodbDataSourceConfig,
) {
    if let Some(var_112) = &input.table_name {
        object.key("tableName").string(var_112);
    }
    if let Some(var_113) = &input.aws_region {
        object.key("awsRegion").string(var_113);
    }
    if input.use_caller_credentials {
        object
            .key("useCallerCredentials")
            .boolean(input.use_caller_credentials);
    }
    if let Some(var_114) = &input.delta_sync_config {
        let mut object_115 = object.key("deltaSyncConfig").start_object();
        crate::json_ser::serialize_structure_delta_sync_config(&mut object_115, var_114);
        object_115.finish();
    }
    if input.versioned {
        object.key("versioned").boolean(input.versioned);
    }
}

pub fn serialize_structure_elasticsearch_data_source_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ElasticsearchDataSourceConfig,
) {
    if let Some(var_116) = &input.endpoint {
        object.key("endpoint").string(var_116);
    }
    if let Some(var_117) = &input.aws_region {
        object.key("awsRegion").string(var_117);
    }
}

pub fn serialize_structure_http_data_source_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::HttpDataSourceConfig,
) {
    if let Some(var_118) = &input.endpoint {
        object.key("endpoint").string(var_118);
    }
    if let Some(var_119) = &input.authorization_config {
        let mut object_120 = object.key("authorizationConfig").start_object();
        crate::json_ser::serialize_structure_authorization_config(&mut object_120, var_119);
        object_120.finish();
    }
}

pub fn serialize_structure_lambda_data_source_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LambdaDataSourceConfig,
) {
    if let Some(var_121) = &input.lambda_function_arn {
        object.key("lambdaFunctionArn").string(var_121);
    }
}

pub fn serialize_structure_relational_database_data_source_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RelationalDatabaseDataSourceConfig,
) {
    if let Some(var_122) = &input.relational_database_source_type {
        object
            .key("relationalDatabaseSourceType")
            .string(var_122.as_str());
    }
    if let Some(var_123) = &input.rds_http_endpoint_config {
        let mut object_124 = object.key("rdsHttpEndpointConfig").start_object();
        crate::json_ser::serialize_structure_rds_http_endpoint_config(&mut object_124, var_123);
        object_124.finish();
    }
}

pub fn serialize_structure_sync_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SyncConfig,
) {
    if let Some(var_125) = &input.conflict_handler {
        object.key("conflictHandler").string(var_125.as_str());
    }
    if let Some(var_126) = &input.conflict_detection {
        object.key("conflictDetection").string(var_126.as_str());
    }
    if let Some(var_127) = &input.lambda_conflict_handler_config {
        let mut object_128 = object.key("lambdaConflictHandlerConfig").start_object();
        crate::json_ser::serialize_structure_lambda_conflict_handler_config(
            &mut object_128,
            var_127,
        );
        object_128.finish();
    }
}

pub fn serialize_structure_additional_authentication_provider(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AdditionalAuthenticationProvider,
) {
    if let Some(var_129) = &input.authentication_type {
        object.key("authenticationType").string(var_129.as_str());
    }
    if let Some(var_130) = &input.open_id_connect_config {
        let mut object_131 = object.key("openIDConnectConfig").start_object();
        crate::json_ser::serialize_structure_open_id_connect_config(&mut object_131, var_130);
        object_131.finish();
    }
    if let Some(var_132) = &input.user_pool_config {
        let mut object_133 = object.key("userPoolConfig").start_object();
        crate::json_ser::serialize_structure_cognito_user_pool_config(&mut object_133, var_132);
        object_133.finish();
    }
    if let Some(var_134) = &input.lambda_authorizer_config {
        let mut object_135 = object.key("lambdaAuthorizerConfig").start_object();
        crate::json_ser::serialize_structure_lambda_authorizer_config(&mut object_135, var_134);
        object_135.finish();
    }
}

pub fn serialize_structure_lambda_authorizer_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LambdaAuthorizerConfig,
) {
    if input.authorizer_result_ttl_in_seconds != 0 {
        object.key("authorizerResultTtlInSeconds").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.authorizer_result_ttl_in_seconds).into()),
        );
    }
    if let Some(var_136) = &input.authorizer_uri {
        object.key("authorizerUri").string(var_136);
    }
    if let Some(var_137) = &input.identity_validation_expression {
        object.key("identityValidationExpression").string(var_137);
    }
}

pub fn serialize_structure_log_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LogConfig,
) {
    if let Some(var_138) = &input.field_log_level {
        object.key("fieldLogLevel").string(var_138.as_str());
    }
    if let Some(var_139) = &input.cloud_watch_logs_role_arn {
        object.key("cloudWatchLogsRoleArn").string(var_139);
    }
    if input.exclude_verbose_content {
        object
            .key("excludeVerboseContent")
            .boolean(input.exclude_verbose_content);
    }
}

pub fn serialize_structure_open_id_connect_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::OpenIdConnectConfig,
) {
    if let Some(var_140) = &input.issuer {
        object.key("issuer").string(var_140);
    }
    if let Some(var_141) = &input.client_id {
        object.key("clientId").string(var_141);
    }
    if input.iat_ttl != 0 {
        object.key("iatTTL").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.iat_ttl).into()),
        );
    }
    if input.auth_ttl != 0 {
        object.key("authTTL").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.auth_ttl).into()),
        );
    }
}

pub fn serialize_structure_user_pool_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::UserPoolConfig,
) {
    if let Some(var_142) = &input.user_pool_id {
        object.key("userPoolId").string(var_142);
    }
    if let Some(var_143) = &input.aws_region {
        object.key("awsRegion").string(var_143);
    }
    if let Some(var_144) = &input.default_action {
        object.key("defaultAction").string(var_144.as_str());
    }
    if let Some(var_145) = &input.app_id_client_regex {
        object.key("appIdClientRegex").string(var_145);
    }
}

pub fn serialize_structure_caching_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CachingConfig,
) {
    if input.ttl != 0 {
        object.key("ttl").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.ttl).into()),
        );
    }
    if let Some(var_146) = &input.caching_keys {
        let mut array_147 = object.key("cachingKeys").start_array();
        for item_148 in var_146 {
            {
                array_147.value().string(item_148);
            }
        }
        array_147.finish();
    }
}

pub fn serialize_structure_pipeline_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::PipelineConfig,
) {
    if let Some(var_149) = &input.functions {
        let mut array_150 = object.key("functions").start_array();
        for item_151 in var_149 {
            {
                array_150.value().string(item_151);
            }
        }
        array_150.finish();
    }
}

pub fn serialize_structure_delta_sync_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DeltaSyncConfig,
) {
    if input.base_table_ttl != 0 {
        object.key("baseTableTTL").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.base_table_ttl).into()),
        );
    }
    if let Some(var_152) = &input.delta_sync_table_name {
        object.key("deltaSyncTableName").string(var_152);
    }
    if input.delta_sync_table_ttl != 0 {
        object.key("deltaSyncTableTTL").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.delta_sync_table_ttl).into()),
        );
    }
}

pub fn serialize_structure_authorization_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AuthorizationConfig,
) {
    if let Some(var_153) = &input.authorization_type {
        object.key("authorizationType").string(var_153.as_str());
    }
    if let Some(var_154) = &input.aws_iam_config {
        let mut object_155 = object.key("awsIamConfig").start_object();
        crate::json_ser::serialize_structure_aws_iam_config(&mut object_155, var_154);
        object_155.finish();
    }
}

pub fn serialize_structure_rds_http_endpoint_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RdsHttpEndpointConfig,
) {
    if let Some(var_156) = &input.aws_region {
        object.key("awsRegion").string(var_156);
    }
    if let Some(var_157) = &input.db_cluster_identifier {
        object.key("dbClusterIdentifier").string(var_157);
    }
    if let Some(var_158) = &input.database_name {
        object.key("databaseName").string(var_158);
    }
    if let Some(var_159) = &input.schema {
        object.key("schema").string(var_159);
    }
    if let Some(var_160) = &input.aws_secret_store_arn {
        object.key("awsSecretStoreArn").string(var_160);
    }
}

pub fn serialize_structure_lambda_conflict_handler_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LambdaConflictHandlerConfig,
) {
    if let Some(var_161) = &input.lambda_conflict_handler_arn {
        object.key("lambdaConflictHandlerArn").string(var_161);
    }
}

pub fn serialize_structure_cognito_user_pool_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CognitoUserPoolConfig,
) {
    if let Some(var_162) = &input.user_pool_id {
        object.key("userPoolId").string(var_162);
    }
    if let Some(var_163) = &input.aws_region {
        object.key("awsRegion").string(var_163);
    }
    if let Some(var_164) = &input.app_id_client_regex {
        object.key("appIdClientRegex").string(var_164);
    }
}

pub fn serialize_structure_aws_iam_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AwsIamConfig,
) {
    if let Some(var_165) = &input.signing_region {
        object.key("signingRegion").string(var_165);
    }
    if let Some(var_166) = &input.signing_service_name {
        object.key("signingServiceName").string(var_166);
    }
}
