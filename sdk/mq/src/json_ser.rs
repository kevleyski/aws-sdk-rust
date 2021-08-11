// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_create_broker_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateBrokerInput,
) {
    if let Some(var_1) = &input.authentication_strategy {
        object.key("authenticationStrategy").string(var_1.as_str());
    }
    {
        object
            .key("autoMinorVersionUpgrade")
            .boolean(input.auto_minor_version_upgrade);
    }
    if let Some(var_2) = &input.broker_name {
        object.key("brokerName").string(var_2);
    }
    if let Some(var_3) = &input.configuration {
        let mut object_4 = object.key("configuration").start_object();
        crate::json_ser::serialize_structure_configuration_id(&mut object_4, var_3);
        object_4.finish();
    }
    if let Some(var_5) = &input.creator_request_id {
        object.key("creatorRequestId").string(var_5);
    }
    if let Some(var_6) = &input.deployment_mode {
        object.key("deploymentMode").string(var_6.as_str());
    }
    if let Some(var_7) = &input.encryption_options {
        let mut object_8 = object.key("encryptionOptions").start_object();
        crate::json_ser::serialize_structure_encryption_options(&mut object_8, var_7);
        object_8.finish();
    }
    if let Some(var_9) = &input.engine_type {
        object.key("engineType").string(var_9.as_str());
    }
    if let Some(var_10) = &input.engine_version {
        object.key("engineVersion").string(var_10);
    }
    if let Some(var_11) = &input.host_instance_type {
        object.key("hostInstanceType").string(var_11);
    }
    if let Some(var_12) = &input.ldap_server_metadata {
        let mut object_13 = object.key("ldapServerMetadata").start_object();
        crate::json_ser::serialize_structure_ldap_server_metadata_input(&mut object_13, var_12);
        object_13.finish();
    }
    if let Some(var_14) = &input.logs {
        let mut object_15 = object.key("logs").start_object();
        crate::json_ser::serialize_structure_logs(&mut object_15, var_14);
        object_15.finish();
    }
    if let Some(var_16) = &input.maintenance_window_start_time {
        let mut object_17 = object.key("maintenanceWindowStartTime").start_object();
        crate::json_ser::serialize_structure_weekly_start_time(&mut object_17, var_16);
        object_17.finish();
    }
    {
        object
            .key("publiclyAccessible")
            .boolean(input.publicly_accessible);
    }
    if let Some(var_18) = &input.security_groups {
        let mut array_19 = object.key("securityGroups").start_array();
        for item_20 in var_18 {
            {
                array_19.value().string(item_20);
            }
        }
        array_19.finish();
    }
    if let Some(var_21) = &input.storage_type {
        object.key("storageType").string(var_21.as_str());
    }
    if let Some(var_22) = &input.subnet_ids {
        let mut array_23 = object.key("subnetIds").start_array();
        for item_24 in var_22 {
            {
                array_23.value().string(item_24);
            }
        }
        array_23.finish();
    }
    if let Some(var_25) = &input.tags {
        let mut object_26 = object.key("tags").start_object();
        for (key_27, value_28) in var_25 {
            {
                object_26.key(key_27).string(value_28);
            }
        }
        object_26.finish();
    }
    if let Some(var_29) = &input.users {
        let mut array_30 = object.key("users").start_array();
        for item_31 in var_29 {
            {
                let mut object_32 = array_30.value().start_object();
                crate::json_ser::serialize_structure_user(&mut object_32, item_31);
                object_32.finish();
            }
        }
        array_30.finish();
    }
}

pub fn serialize_structure_create_configuration_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateConfigurationInput,
) {
    if let Some(var_33) = &input.authentication_strategy {
        object.key("authenticationStrategy").string(var_33.as_str());
    }
    if let Some(var_34) = &input.engine_type {
        object.key("engineType").string(var_34.as_str());
    }
    if let Some(var_35) = &input.engine_version {
        object.key("engineVersion").string(var_35);
    }
    if let Some(var_36) = &input.name {
        object.key("name").string(var_36);
    }
    if let Some(var_37) = &input.tags {
        let mut object_38 = object.key("tags").start_object();
        for (key_39, value_40) in var_37 {
            {
                object_38.key(key_39).string(value_40);
            }
        }
        object_38.finish();
    }
}

pub fn serialize_structure_create_tags_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateTagsInput,
) {
    if let Some(var_41) = &input.tags {
        let mut object_42 = object.key("tags").start_object();
        for (key_43, value_44) in var_41 {
            {
                object_42.key(key_43).string(value_44);
            }
        }
        object_42.finish();
    }
}

pub fn serialize_structure_create_user_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateUserInput,
) {
    if input.console_access {
        object.key("consoleAccess").boolean(input.console_access);
    }
    if let Some(var_45) = &input.groups {
        let mut array_46 = object.key("groups").start_array();
        for item_47 in var_45 {
            {
                array_46.value().string(item_47);
            }
        }
        array_46.finish();
    }
    if let Some(var_48) = &input.password {
        object.key("password").string(var_48);
    }
}

pub fn serialize_structure_update_broker_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateBrokerInput,
) {
    if let Some(var_49) = &input.authentication_strategy {
        object.key("authenticationStrategy").string(var_49.as_str());
    }
    if input.auto_minor_version_upgrade {
        object
            .key("autoMinorVersionUpgrade")
            .boolean(input.auto_minor_version_upgrade);
    }
    if let Some(var_50) = &input.configuration {
        let mut object_51 = object.key("configuration").start_object();
        crate::json_ser::serialize_structure_configuration_id(&mut object_51, var_50);
        object_51.finish();
    }
    if let Some(var_52) = &input.engine_version {
        object.key("engineVersion").string(var_52);
    }
    if let Some(var_53) = &input.host_instance_type {
        object.key("hostInstanceType").string(var_53);
    }
    if let Some(var_54) = &input.ldap_server_metadata {
        let mut object_55 = object.key("ldapServerMetadata").start_object();
        crate::json_ser::serialize_structure_ldap_server_metadata_input(&mut object_55, var_54);
        object_55.finish();
    }
    if let Some(var_56) = &input.logs {
        let mut object_57 = object.key("logs").start_object();
        crate::json_ser::serialize_structure_logs(&mut object_57, var_56);
        object_57.finish();
    }
    if let Some(var_58) = &input.maintenance_window_start_time {
        let mut object_59 = object.key("maintenanceWindowStartTime").start_object();
        crate::json_ser::serialize_structure_weekly_start_time(&mut object_59, var_58);
        object_59.finish();
    }
    if let Some(var_60) = &input.security_groups {
        let mut array_61 = object.key("securityGroups").start_array();
        for item_62 in var_60 {
            {
                array_61.value().string(item_62);
            }
        }
        array_61.finish();
    }
}

pub fn serialize_structure_update_configuration_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateConfigurationInput,
) {
    if let Some(var_63) = &input.data {
        object.key("data").string(var_63);
    }
    if let Some(var_64) = &input.description {
        object.key("description").string(var_64);
    }
}

pub fn serialize_structure_update_user_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateUserInput,
) {
    if input.console_access {
        object.key("consoleAccess").boolean(input.console_access);
    }
    if let Some(var_65) = &input.groups {
        let mut array_66 = object.key("groups").start_array();
        for item_67 in var_65 {
            {
                array_66.value().string(item_67);
            }
        }
        array_66.finish();
    }
    if let Some(var_68) = &input.password {
        object.key("password").string(var_68);
    }
}

pub fn serialize_structure_configuration_id(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ConfigurationId,
) {
    if let Some(var_69) = &input.id {
        object.key("id").string(var_69);
    }
    if input.revision != 0 {
        object.key("revision").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.revision).into()),
        );
    }
}

pub fn serialize_structure_encryption_options(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::EncryptionOptions,
) {
    if let Some(var_70) = &input.kms_key_id {
        object.key("kmsKeyId").string(var_70);
    }
    {
        object
            .key("useAwsOwnedKey")
            .boolean(input.use_aws_owned_key);
    }
}

pub fn serialize_structure_ldap_server_metadata_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LdapServerMetadataInput,
) {
    if let Some(var_71) = &input.hosts {
        let mut array_72 = object.key("hosts").start_array();
        for item_73 in var_71 {
            {
                array_72.value().string(item_73);
            }
        }
        array_72.finish();
    }
    if let Some(var_74) = &input.role_base {
        object.key("roleBase").string(var_74);
    }
    if let Some(var_75) = &input.role_name {
        object.key("roleName").string(var_75);
    }
    if let Some(var_76) = &input.role_search_matching {
        object.key("roleSearchMatching").string(var_76);
    }
    if input.role_search_subtree {
        object
            .key("roleSearchSubtree")
            .boolean(input.role_search_subtree);
    }
    if let Some(var_77) = &input.service_account_password {
        object.key("serviceAccountPassword").string(var_77);
    }
    if let Some(var_78) = &input.service_account_username {
        object.key("serviceAccountUsername").string(var_78);
    }
    if let Some(var_79) = &input.user_base {
        object.key("userBase").string(var_79);
    }
    if let Some(var_80) = &input.user_role_name {
        object.key("userRoleName").string(var_80);
    }
    if let Some(var_81) = &input.user_search_matching {
        object.key("userSearchMatching").string(var_81);
    }
    if input.user_search_subtree {
        object
            .key("userSearchSubtree")
            .boolean(input.user_search_subtree);
    }
}

pub fn serialize_structure_logs(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Logs,
) {
    if input.audit {
        object.key("audit").boolean(input.audit);
    }
    if input.general {
        object.key("general").boolean(input.general);
    }
}

pub fn serialize_structure_weekly_start_time(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::WeeklyStartTime,
) {
    if let Some(var_82) = &input.day_of_week {
        object.key("dayOfWeek").string(var_82.as_str());
    }
    if let Some(var_83) = &input.time_of_day {
        object.key("timeOfDay").string(var_83);
    }
    if let Some(var_84) = &input.time_zone {
        object.key("timeZone").string(var_84);
    }
}

pub fn serialize_structure_user(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::User,
) {
    if input.console_access {
        object.key("consoleAccess").boolean(input.console_access);
    }
    if let Some(var_85) = &input.groups {
        let mut array_86 = object.key("groups").start_array();
        for item_87 in var_85 {
            {
                array_86.value().string(item_87);
            }
        }
        array_86.finish();
    }
    if let Some(var_88) = &input.password {
        object.key("password").string(var_88);
    }
    if let Some(var_89) = &input.username {
        object.key("username").string(var_89);
    }
}
