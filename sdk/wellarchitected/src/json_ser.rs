// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_associate_lenses_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AssociateLensesInput,
) {
    if let Some(var_1) = &input.lens_aliases {
        let mut array_2 = object.key("LensAliases").start_array();
        for item_3 in var_1 {
            {
                array_2.value().string(item_3);
            }
        }
        array_2.finish();
    }
}

pub fn serialize_structure_create_milestone_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateMilestoneInput,
) {
    if let Some(var_4) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_4);
    }
    if let Some(var_5) = &input.milestone_name {
        object.key("MilestoneName").string(var_5);
    }
}

pub fn serialize_structure_create_workload_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateWorkloadInput,
) {
    if let Some(var_6) = &input.account_ids {
        let mut array_7 = object.key("AccountIds").start_array();
        for item_8 in var_6 {
            {
                array_7.value().string(item_8);
            }
        }
        array_7.finish();
    }
    if let Some(var_9) = &input.architectural_design {
        object.key("ArchitecturalDesign").string(var_9);
    }
    if let Some(var_10) = &input.aws_regions {
        let mut array_11 = object.key("AwsRegions").start_array();
        for item_12 in var_10 {
            {
                array_11.value().string(item_12);
            }
        }
        array_11.finish();
    }
    if let Some(var_13) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_13);
    }
    if let Some(var_14) = &input.description {
        object.key("Description").string(var_14);
    }
    if let Some(var_15) = &input.environment {
        object.key("Environment").string(var_15.as_str());
    }
    if let Some(var_16) = &input.industry {
        object.key("Industry").string(var_16);
    }
    if let Some(var_17) = &input.industry_type {
        object.key("IndustryType").string(var_17);
    }
    if let Some(var_18) = &input.lenses {
        let mut array_19 = object.key("Lenses").start_array();
        for item_20 in var_18 {
            {
                array_19.value().string(item_20);
            }
        }
        array_19.finish();
    }
    if let Some(var_21) = &input.non_aws_regions {
        let mut array_22 = object.key("NonAwsRegions").start_array();
        for item_23 in var_21 {
            {
                array_22.value().string(item_23);
            }
        }
        array_22.finish();
    }
    if let Some(var_24) = &input.notes {
        object.key("Notes").string(var_24);
    }
    if let Some(var_25) = &input.pillar_priorities {
        let mut array_26 = object.key("PillarPriorities").start_array();
        for item_27 in var_25 {
            {
                array_26.value().string(item_27);
            }
        }
        array_26.finish();
    }
    if let Some(var_28) = &input.review_owner {
        object.key("ReviewOwner").string(var_28);
    }
    if let Some(var_29) = &input.tags {
        let mut object_30 = object.key("Tags").start_object();
        for (key_31, value_32) in var_29 {
            {
                object_30.key(key_31).string(value_32);
            }
        }
        object_30.finish();
    }
    if let Some(var_33) = &input.workload_name {
        object.key("WorkloadName").string(var_33);
    }
}

pub fn serialize_structure_create_workload_share_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateWorkloadShareInput,
) {
    if let Some(var_34) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_34);
    }
    if let Some(var_35) = &input.permission_type {
        object.key("PermissionType").string(var_35.as_str());
    }
    if let Some(var_36) = &input.shared_with {
        object.key("SharedWith").string(var_36);
    }
}

pub fn serialize_structure_disassociate_lenses_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DisassociateLensesInput,
) {
    if let Some(var_37) = &input.lens_aliases {
        let mut array_38 = object.key("LensAliases").start_array();
        for item_39 in var_37 {
            {
                array_38.value().string(item_39);
            }
        }
        array_38.finish();
    }
}

pub fn serialize_structure_list_milestones_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListMilestonesInput,
) {
    if input.max_results != 0 {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_40) = &input.next_token {
        object.key("NextToken").string(var_40);
    }
}

pub fn serialize_structure_list_notifications_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListNotificationsInput,
) {
    if input.max_results != 0 {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_41) = &input.next_token {
        object.key("NextToken").string(var_41);
    }
    if let Some(var_42) = &input.workload_id {
        object.key("WorkloadId").string(var_42);
    }
}

pub fn serialize_structure_list_workloads_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListWorkloadsInput,
) {
    if input.max_results != 0 {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_43) = &input.next_token {
        object.key("NextToken").string(var_43);
    }
    if let Some(var_44) = &input.workload_name_prefix {
        object.key("WorkloadNamePrefix").string(var_44);
    }
}

pub fn serialize_structure_tag_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) {
    if let Some(var_45) = &input.tags {
        let mut object_46 = object.key("Tags").start_object();
        for (key_47, value_48) in var_45 {
            {
                object_46.key(key_47).string(value_48);
            }
        }
        object_46.finish();
    }
}

pub fn serialize_structure_update_answer_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateAnswerInput,
) {
    if let Some(var_49) = &input.choice_updates {
        let mut object_50 = object.key("ChoiceUpdates").start_object();
        for (key_51, value_52) in var_49 {
            {
                let mut object_53 = object_50.key(key_51).start_object();
                crate::json_ser::serialize_structure_choice_update(&mut object_53, value_52);
                object_53.finish();
            }
        }
        object_50.finish();
    }
    if input.is_applicable {
        object.key("IsApplicable").boolean(input.is_applicable);
    }
    if let Some(var_54) = &input.notes {
        object.key("Notes").string(var_54);
    }
    if let Some(var_55) = &input.reason {
        object.key("Reason").string(var_55.as_str());
    }
    if let Some(var_56) = &input.selected_choices {
        let mut array_57 = object.key("SelectedChoices").start_array();
        for item_58 in var_56 {
            {
                array_57.value().string(item_58);
            }
        }
        array_57.finish();
    }
}

pub fn serialize_structure_update_lens_review_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateLensReviewInput,
) {
    if let Some(var_59) = &input.lens_notes {
        object.key("LensNotes").string(var_59);
    }
    if let Some(var_60) = &input.pillar_notes {
        let mut object_61 = object.key("PillarNotes").start_object();
        for (key_62, value_63) in var_60 {
            {
                object_61.key(key_62).string(value_63);
            }
        }
        object_61.finish();
    }
}

pub fn serialize_structure_update_share_invitation_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateShareInvitationInput,
) {
    if let Some(var_64) = &input.share_invitation_action {
        object.key("ShareInvitationAction").string(var_64.as_str());
    }
}

pub fn serialize_structure_update_workload_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateWorkloadInput,
) {
    if let Some(var_65) = &input.account_ids {
        let mut array_66 = object.key("AccountIds").start_array();
        for item_67 in var_65 {
            {
                array_66.value().string(item_67);
            }
        }
        array_66.finish();
    }
    if let Some(var_68) = &input.architectural_design {
        object.key("ArchitecturalDesign").string(var_68);
    }
    if let Some(var_69) = &input.aws_regions {
        let mut array_70 = object.key("AwsRegions").start_array();
        for item_71 in var_69 {
            {
                array_70.value().string(item_71);
            }
        }
        array_70.finish();
    }
    if let Some(var_72) = &input.description {
        object.key("Description").string(var_72);
    }
    if let Some(var_73) = &input.environment {
        object.key("Environment").string(var_73.as_str());
    }
    if let Some(var_74) = &input.improvement_status {
        object.key("ImprovementStatus").string(var_74.as_str());
    }
    if let Some(var_75) = &input.industry {
        object.key("Industry").string(var_75);
    }
    if let Some(var_76) = &input.industry_type {
        object.key("IndustryType").string(var_76);
    }
    if input.is_review_owner_update_acknowledged {
        object
            .key("IsReviewOwnerUpdateAcknowledged")
            .boolean(input.is_review_owner_update_acknowledged);
    }
    if let Some(var_77) = &input.non_aws_regions {
        let mut array_78 = object.key("NonAwsRegions").start_array();
        for item_79 in var_77 {
            {
                array_78.value().string(item_79);
            }
        }
        array_78.finish();
    }
    if let Some(var_80) = &input.notes {
        object.key("Notes").string(var_80);
    }
    if let Some(var_81) = &input.pillar_priorities {
        let mut array_82 = object.key("PillarPriorities").start_array();
        for item_83 in var_81 {
            {
                array_82.value().string(item_83);
            }
        }
        array_82.finish();
    }
    if let Some(var_84) = &input.review_owner {
        object.key("ReviewOwner").string(var_84);
    }
    if let Some(var_85) = &input.workload_name {
        object.key("WorkloadName").string(var_85);
    }
}

pub fn serialize_structure_update_workload_share_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateWorkloadShareInput,
) {
    if let Some(var_86) = &input.permission_type {
        object.key("PermissionType").string(var_86.as_str());
    }
}

pub fn serialize_structure_upgrade_lens_review_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpgradeLensReviewInput,
) {
    if let Some(var_87) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_87);
    }
    if let Some(var_88) = &input.milestone_name {
        object.key("MilestoneName").string(var_88);
    }
}

pub fn serialize_structure_choice_update(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ChoiceUpdate,
) {
    if let Some(var_89) = &input.status {
        object.key("Status").string(var_89.as_str());
    }
    if let Some(var_90) = &input.reason {
        object.key("Reason").string(var_90.as_str());
    }
    if let Some(var_91) = &input.notes {
        object.key("Notes").string(var_91);
    }
}
