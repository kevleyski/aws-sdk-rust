// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_create_bot_version_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateBotVersionInput,
) {
    if let Some(var_1) = &input.checksum {
        object.key("checksum").string(var_1);
    }
}

pub fn serialize_structure_create_intent_version_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateIntentVersionInput,
) {
    if let Some(var_2) = &input.checksum {
        object.key("checksum").string(var_2);
    }
}

pub fn serialize_structure_create_slot_type_version_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateSlotTypeVersionInput,
) {
    if let Some(var_3) = &input.checksum {
        object.key("checksum").string(var_3);
    }
}

pub fn serialize_structure_put_bot_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutBotInput,
) {
    if let Some(var_4) = &input.abort_statement {
        let mut object_5 = object.key("abortStatement").start_object();
        crate::json_ser::serialize_structure_statement(&mut object_5, var_4);
        object_5.finish();
    }
    if let Some(var_6) = &input.checksum {
        object.key("checksum").string(var_6);
    }
    if let Some(var_7) = &input.child_directed {
        object.key("childDirected").boolean(*var_7);
    }
    if let Some(var_8) = &input.clarification_prompt {
        let mut object_9 = object.key("clarificationPrompt").start_object();
        crate::json_ser::serialize_structure_prompt(&mut object_9, var_8);
        object_9.finish();
    }
    if let Some(var_10) = &input.create_version {
        object.key("createVersion").boolean(*var_10);
    }
    if let Some(var_11) = &input.description {
        object.key("description").string(var_11);
    }
    if let Some(var_12) = &input.detect_sentiment {
        object.key("detectSentiment").boolean(*var_12);
    }
    if let Some(var_13) = &input.enable_model_improvements {
        object.key("enableModelImprovements").boolean(*var_13);
    }
    if let Some(var_14) = &input.idle_session_ttl_in_seconds {
        object.key("idleSessionTTLInSeconds").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_14).into()),
        );
    }
    if let Some(var_15) = &input.intents {
        let mut array_16 = object.key("intents").start_array();
        for item_17 in var_15 {
            {
                let mut object_18 = array_16.value().start_object();
                crate::json_ser::serialize_structure_intent(&mut object_18, item_17);
                object_18.finish();
            }
        }
        array_16.finish();
    }
    if let Some(var_19) = &input.locale {
        object.key("locale").string(var_19.as_str());
    }
    if let Some(var_20) = &input.nlu_intent_confidence_threshold {
        object.key("nluIntentConfidenceThreshold").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::Float((*var_20).into()),
        );
    }
    if let Some(var_21) = &input.process_behavior {
        object.key("processBehavior").string(var_21.as_str());
    }
    if let Some(var_22) = &input.tags {
        let mut array_23 = object.key("tags").start_array();
        for item_24 in var_22 {
            {
                let mut object_25 = array_23.value().start_object();
                crate::json_ser::serialize_structure_tag(&mut object_25, item_24);
                object_25.finish();
            }
        }
        array_23.finish();
    }
    if let Some(var_26) = &input.voice_id {
        object.key("voiceId").string(var_26);
    }
}

pub fn serialize_structure_put_bot_alias_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutBotAliasInput,
) {
    if let Some(var_27) = &input.bot_version {
        object.key("botVersion").string(var_27);
    }
    if let Some(var_28) = &input.checksum {
        object.key("checksum").string(var_28);
    }
    if let Some(var_29) = &input.conversation_logs {
        let mut object_30 = object.key("conversationLogs").start_object();
        crate::json_ser::serialize_structure_conversation_logs_request(&mut object_30, var_29);
        object_30.finish();
    }
    if let Some(var_31) = &input.description {
        object.key("description").string(var_31);
    }
    if let Some(var_32) = &input.tags {
        let mut array_33 = object.key("tags").start_array();
        for item_34 in var_32 {
            {
                let mut object_35 = array_33.value().start_object();
                crate::json_ser::serialize_structure_tag(&mut object_35, item_34);
                object_35.finish();
            }
        }
        array_33.finish();
    }
}

pub fn serialize_structure_put_intent_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutIntentInput,
) {
    if let Some(var_36) = &input.checksum {
        object.key("checksum").string(var_36);
    }
    if let Some(var_37) = &input.conclusion_statement {
        let mut object_38 = object.key("conclusionStatement").start_object();
        crate::json_ser::serialize_structure_statement(&mut object_38, var_37);
        object_38.finish();
    }
    if let Some(var_39) = &input.confirmation_prompt {
        let mut object_40 = object.key("confirmationPrompt").start_object();
        crate::json_ser::serialize_structure_prompt(&mut object_40, var_39);
        object_40.finish();
    }
    if let Some(var_41) = &input.create_version {
        object.key("createVersion").boolean(*var_41);
    }
    if let Some(var_42) = &input.description {
        object.key("description").string(var_42);
    }
    if let Some(var_43) = &input.dialog_code_hook {
        let mut object_44 = object.key("dialogCodeHook").start_object();
        crate::json_ser::serialize_structure_code_hook(&mut object_44, var_43);
        object_44.finish();
    }
    if let Some(var_45) = &input.follow_up_prompt {
        let mut object_46 = object.key("followUpPrompt").start_object();
        crate::json_ser::serialize_structure_follow_up_prompt(&mut object_46, var_45);
        object_46.finish();
    }
    if let Some(var_47) = &input.fulfillment_activity {
        let mut object_48 = object.key("fulfillmentActivity").start_object();
        crate::json_ser::serialize_structure_fulfillment_activity(&mut object_48, var_47);
        object_48.finish();
    }
    if let Some(var_49) = &input.input_contexts {
        let mut array_50 = object.key("inputContexts").start_array();
        for item_51 in var_49 {
            {
                let mut object_52 = array_50.value().start_object();
                crate::json_ser::serialize_structure_input_context(&mut object_52, item_51);
                object_52.finish();
            }
        }
        array_50.finish();
    }
    if let Some(var_53) = &input.kendra_configuration {
        let mut object_54 = object.key("kendraConfiguration").start_object();
        crate::json_ser::serialize_structure_kendra_configuration(&mut object_54, var_53);
        object_54.finish();
    }
    if let Some(var_55) = &input.output_contexts {
        let mut array_56 = object.key("outputContexts").start_array();
        for item_57 in var_55 {
            {
                let mut object_58 = array_56.value().start_object();
                crate::json_ser::serialize_structure_output_context(&mut object_58, item_57);
                object_58.finish();
            }
        }
        array_56.finish();
    }
    if let Some(var_59) = &input.parent_intent_signature {
        object.key("parentIntentSignature").string(var_59);
    }
    if let Some(var_60) = &input.rejection_statement {
        let mut object_61 = object.key("rejectionStatement").start_object();
        crate::json_ser::serialize_structure_statement(&mut object_61, var_60);
        object_61.finish();
    }
    if let Some(var_62) = &input.sample_utterances {
        let mut array_63 = object.key("sampleUtterances").start_array();
        for item_64 in var_62 {
            {
                array_63.value().string(item_64);
            }
        }
        array_63.finish();
    }
    if let Some(var_65) = &input.slots {
        let mut array_66 = object.key("slots").start_array();
        for item_67 in var_65 {
            {
                let mut object_68 = array_66.value().start_object();
                crate::json_ser::serialize_structure_slot(&mut object_68, item_67);
                object_68.finish();
            }
        }
        array_66.finish();
    }
}

pub fn serialize_structure_put_slot_type_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutSlotTypeInput,
) {
    if let Some(var_69) = &input.checksum {
        object.key("checksum").string(var_69);
    }
    if let Some(var_70) = &input.create_version {
        object.key("createVersion").boolean(*var_70);
    }
    if let Some(var_71) = &input.description {
        object.key("description").string(var_71);
    }
    if let Some(var_72) = &input.enumeration_values {
        let mut array_73 = object.key("enumerationValues").start_array();
        for item_74 in var_72 {
            {
                let mut object_75 = array_73.value().start_object();
                crate::json_ser::serialize_structure_enumeration_value(&mut object_75, item_74);
                object_75.finish();
            }
        }
        array_73.finish();
    }
    if let Some(var_76) = &input.parent_slot_type_signature {
        object.key("parentSlotTypeSignature").string(var_76);
    }
    if let Some(var_77) = &input.slot_type_configurations {
        let mut array_78 = object.key("slotTypeConfigurations").start_array();
        for item_79 in var_77 {
            {
                let mut object_80 = array_78.value().start_object();
                crate::json_ser::serialize_structure_slot_type_configuration(
                    &mut object_80,
                    item_79,
                );
                object_80.finish();
            }
        }
        array_78.finish();
    }
    if let Some(var_81) = &input.value_selection_strategy {
        object.key("valueSelectionStrategy").string(var_81.as_str());
    }
}

pub fn serialize_structure_start_import_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartImportInput,
) {
    if let Some(var_82) = &input.merge_strategy {
        object.key("mergeStrategy").string(var_82.as_str());
    }
    if let Some(var_83) = &input.payload {
        object
            .key("payload")
            .string_unchecked(&smithy_types::base64::encode(var_83));
    }
    if let Some(var_84) = &input.resource_type {
        object.key("resourceType").string(var_84.as_str());
    }
    if let Some(var_85) = &input.tags {
        let mut array_86 = object.key("tags").start_array();
        for item_87 in var_85 {
            {
                let mut object_88 = array_86.value().start_object();
                crate::json_ser::serialize_structure_tag(&mut object_88, item_87);
                object_88.finish();
            }
        }
        array_86.finish();
    }
}

pub fn serialize_structure_start_migration_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartMigrationInput,
) {
    if let Some(var_89) = &input.migration_strategy {
        object.key("migrationStrategy").string(var_89.as_str());
    }
    if let Some(var_90) = &input.v1_bot_name {
        object.key("v1BotName").string(var_90);
    }
    if let Some(var_91) = &input.v1_bot_version {
        object.key("v1BotVersion").string(var_91);
    }
    if let Some(var_92) = &input.v2_bot_name {
        object.key("v2BotName").string(var_92);
    }
    if let Some(var_93) = &input.v2_bot_role {
        object.key("v2BotRole").string(var_93);
    }
}

pub fn serialize_structure_tag_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) {
    if let Some(var_94) = &input.tags {
        let mut array_95 = object.key("tags").start_array();
        for item_96 in var_94 {
            {
                let mut object_97 = array_95.value().start_object();
                crate::json_ser::serialize_structure_tag(&mut object_97, item_96);
                object_97.finish();
            }
        }
        array_95.finish();
    }
}

pub fn serialize_structure_statement(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Statement,
) {
    if let Some(var_98) = &input.messages {
        let mut array_99 = object.key("messages").start_array();
        for item_100 in var_98 {
            {
                let mut object_101 = array_99.value().start_object();
                crate::json_ser::serialize_structure_message(&mut object_101, item_100);
                object_101.finish();
            }
        }
        array_99.finish();
    }
    if let Some(var_102) = &input.response_card {
        object.key("responseCard").string(var_102);
    }
}

pub fn serialize_structure_prompt(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Prompt,
) {
    if let Some(var_103) = &input.messages {
        let mut array_104 = object.key("messages").start_array();
        for item_105 in var_103 {
            {
                let mut object_106 = array_104.value().start_object();
                crate::json_ser::serialize_structure_message(&mut object_106, item_105);
                object_106.finish();
            }
        }
        array_104.finish();
    }
    if let Some(var_107) = &input.max_attempts {
        object.key("maxAttempts").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_107).into()),
        );
    }
    if let Some(var_108) = &input.response_card {
        object.key("responseCard").string(var_108);
    }
}

pub fn serialize_structure_intent(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Intent,
) {
    if let Some(var_109) = &input.intent_name {
        object.key("intentName").string(var_109);
    }
    if let Some(var_110) = &input.intent_version {
        object.key("intentVersion").string(var_110);
    }
}

pub fn serialize_structure_tag(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) {
    if let Some(var_111) = &input.key {
        object.key("key").string(var_111);
    }
    if let Some(var_112) = &input.value {
        object.key("value").string(var_112);
    }
}

pub fn serialize_structure_conversation_logs_request(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ConversationLogsRequest,
) {
    if let Some(var_113) = &input.log_settings {
        let mut array_114 = object.key("logSettings").start_array();
        for item_115 in var_113 {
            {
                let mut object_116 = array_114.value().start_object();
                crate::json_ser::serialize_structure_log_settings_request(
                    &mut object_116,
                    item_115,
                );
                object_116.finish();
            }
        }
        array_114.finish();
    }
    if let Some(var_117) = &input.iam_role_arn {
        object.key("iamRoleArn").string(var_117);
    }
}

pub fn serialize_structure_code_hook(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CodeHook,
) {
    if let Some(var_118) = &input.uri {
        object.key("uri").string(var_118);
    }
    if let Some(var_119) = &input.message_version {
        object.key("messageVersion").string(var_119);
    }
}

pub fn serialize_structure_follow_up_prompt(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::FollowUpPrompt,
) {
    if let Some(var_120) = &input.prompt {
        let mut object_121 = object.key("prompt").start_object();
        crate::json_ser::serialize_structure_prompt(&mut object_121, var_120);
        object_121.finish();
    }
    if let Some(var_122) = &input.rejection_statement {
        let mut object_123 = object.key("rejectionStatement").start_object();
        crate::json_ser::serialize_structure_statement(&mut object_123, var_122);
        object_123.finish();
    }
}

pub fn serialize_structure_fulfillment_activity(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::FulfillmentActivity,
) {
    if let Some(var_124) = &input.r#type {
        object.key("type").string(var_124.as_str());
    }
    if let Some(var_125) = &input.code_hook {
        let mut object_126 = object.key("codeHook").start_object();
        crate::json_ser::serialize_structure_code_hook(&mut object_126, var_125);
        object_126.finish();
    }
}

pub fn serialize_structure_input_context(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::InputContext,
) {
    if let Some(var_127) = &input.name {
        object.key("name").string(var_127);
    }
}

pub fn serialize_structure_kendra_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::KendraConfiguration,
) {
    if let Some(var_128) = &input.kendra_index {
        object.key("kendraIndex").string(var_128);
    }
    if let Some(var_129) = &input.query_filter_string {
        object.key("queryFilterString").string(var_129);
    }
    if let Some(var_130) = &input.role {
        object.key("role").string(var_130);
    }
}

pub fn serialize_structure_output_context(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::OutputContext,
) {
    if let Some(var_131) = &input.name {
        object.key("name").string(var_131);
    }
    if let Some(var_132) = &input.time_to_live_in_seconds {
        object.key("timeToLiveInSeconds").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_132).into()),
        );
    }
    if let Some(var_133) = &input.turns_to_live {
        object.key("turnsToLive").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_133).into()),
        );
    }
}

pub fn serialize_structure_slot(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Slot,
) {
    if let Some(var_134) = &input.name {
        object.key("name").string(var_134);
    }
    if let Some(var_135) = &input.description {
        object.key("description").string(var_135);
    }
    if let Some(var_136) = &input.slot_constraint {
        object.key("slotConstraint").string(var_136.as_str());
    }
    if let Some(var_137) = &input.slot_type {
        object.key("slotType").string(var_137);
    }
    if let Some(var_138) = &input.slot_type_version {
        object.key("slotTypeVersion").string(var_138);
    }
    if let Some(var_139) = &input.value_elicitation_prompt {
        let mut object_140 = object.key("valueElicitationPrompt").start_object();
        crate::json_ser::serialize_structure_prompt(&mut object_140, var_139);
        object_140.finish();
    }
    if let Some(var_141) = &input.priority {
        object.key("priority").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_141).into()),
        );
    }
    if let Some(var_142) = &input.sample_utterances {
        let mut array_143 = object.key("sampleUtterances").start_array();
        for item_144 in var_142 {
            {
                array_143.value().string(item_144);
            }
        }
        array_143.finish();
    }
    if let Some(var_145) = &input.response_card {
        object.key("responseCard").string(var_145);
    }
    if let Some(var_146) = &input.obfuscation_setting {
        object.key("obfuscationSetting").string(var_146.as_str());
    }
    if let Some(var_147) = &input.default_value_spec {
        let mut object_148 = object.key("defaultValueSpec").start_object();
        crate::json_ser::serialize_structure_slot_default_value_spec(&mut object_148, var_147);
        object_148.finish();
    }
}

pub fn serialize_structure_enumeration_value(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::EnumerationValue,
) {
    if let Some(var_149) = &input.value {
        object.key("value").string(var_149);
    }
    if let Some(var_150) = &input.synonyms {
        let mut array_151 = object.key("synonyms").start_array();
        for item_152 in var_150 {
            {
                array_151.value().string(item_152);
            }
        }
        array_151.finish();
    }
}

pub fn serialize_structure_slot_type_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SlotTypeConfiguration,
) {
    if let Some(var_153) = &input.regex_configuration {
        let mut object_154 = object.key("regexConfiguration").start_object();
        crate::json_ser::serialize_structure_slot_type_regex_configuration(
            &mut object_154,
            var_153,
        );
        object_154.finish();
    }
}

pub fn serialize_structure_message(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Message,
) {
    if let Some(var_155) = &input.content_type {
        object.key("contentType").string(var_155.as_str());
    }
    if let Some(var_156) = &input.content {
        object.key("content").string(var_156);
    }
    if let Some(var_157) = &input.group_number {
        object.key("groupNumber").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_157).into()),
        );
    }
}

pub fn serialize_structure_log_settings_request(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LogSettingsRequest,
) {
    if let Some(var_158) = &input.log_type {
        object.key("logType").string(var_158.as_str());
    }
    if let Some(var_159) = &input.destination {
        object.key("destination").string(var_159.as_str());
    }
    if let Some(var_160) = &input.kms_key_arn {
        object.key("kmsKeyArn").string(var_160);
    }
    if let Some(var_161) = &input.resource_arn {
        object.key("resourceArn").string(var_161);
    }
}

pub fn serialize_structure_slot_default_value_spec(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SlotDefaultValueSpec,
) {
    if let Some(var_162) = &input.default_value_list {
        let mut array_163 = object.key("defaultValueList").start_array();
        for item_164 in var_162 {
            {
                let mut object_165 = array_163.value().start_object();
                crate::json_ser::serialize_structure_slot_default_value(&mut object_165, item_164);
                object_165.finish();
            }
        }
        array_163.finish();
    }
}

pub fn serialize_structure_slot_type_regex_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SlotTypeRegexConfiguration,
) {
    if let Some(var_166) = &input.pattern {
        object.key("pattern").string(var_166);
    }
}

pub fn serialize_structure_slot_default_value(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SlotDefaultValue,
) {
    if let Some(var_167) = &input.default_value {
        object.key("defaultValue").string(var_167);
    }
}
