// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) mod shape_create_cluster;

pub fn parse_http_error_metadata(
    response: &http::Response<bytes::Bytes>,
) -> Result<
    aws_smithy_types::error::metadata::Builder,
    aws_smithy_json::deserialize::error::DeserializeError,
> {
    crate::json_errors::parse_error_metadata(response.body(), response.headers())
}

pub(crate) mod shape_create_control_panel;

pub(crate) mod shape_create_routing_control;

pub(crate) mod shape_create_safety_rule;

pub(crate) mod shape_delete_cluster;

pub(crate) mod shape_delete_control_panel;

pub(crate) mod shape_delete_routing_control;

pub(crate) mod shape_delete_safety_rule;

pub(crate) mod shape_describe_cluster;

pub(crate) mod shape_describe_control_panel;

pub(crate) mod shape_describe_routing_control;

pub(crate) mod shape_describe_safety_rule;

pub(crate) mod shape_list_associated_route53_health_checks;

pub(crate) mod shape_list_clusters;

pub(crate) mod shape_list_control_panels;

pub(crate) mod shape_list_routing_controls;

pub(crate) mod shape_list_safety_rules;

pub(crate) mod shape_list_tags_for_resource;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_untag_resource;

pub(crate) mod shape_update_control_panel;

pub(crate) mod shape_update_routing_control;

pub(crate) mod shape_update_safety_rule;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub(crate) mod shape_access_denied_exception;

pub(crate) mod shape_conflict_exception;

pub(crate) mod shape_create_cluster_input;

pub(crate) mod shape_create_control_panel_input;

pub(crate) mod shape_create_routing_control_input;

pub(crate) mod shape_create_safety_rule_input;

pub(crate) mod shape_internal_server_exception;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_service_quota_exceeded_exception;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_throttling_exception;

pub(crate) mod shape_update_control_panel_input;

pub(crate) mod shape_update_routing_control_input;

pub(crate) mod shape_update_safety_rule_input;

pub(crate) mod shape_validation_exception;

#[allow(non_snake_case)]
pub(crate) mod shape___list_of__string_max36_pattern_s;

#[allow(non_snake_case)]
pub(crate) mod shape___list_of_cluster;

#[allow(non_snake_case)]
pub(crate) mod shape___list_of_control_panel;

#[allow(non_snake_case)]
pub(crate) mod shape___list_of_routing_control;

#[allow(non_snake_case)]
pub(crate) mod shape___list_of_rule;

#[allow(non_snake_case)]
pub(crate) mod shape___map_of__string_min0_max256_pattern_s;

pub(crate) mod shape_assertion_rule;

pub(crate) mod shape_assertion_rule_update;

pub(crate) mod shape_cluster;

pub(crate) mod shape_control_panel;

pub(crate) mod shape_gating_rule;

pub(crate) mod shape_gating_rule_update;

pub(crate) mod shape_new_assertion_rule;

pub(crate) mod shape_new_gating_rule;

pub(crate) mod shape_routing_control;

#[allow(non_snake_case)]
pub(crate) mod shape___list_of__string_min1_max256_pattern_a_za_z09;

#[allow(non_snake_case)]
pub(crate) mod shape___list_of_cluster_endpoint;

pub(crate) mod shape_rule;

pub(crate) mod shape_rule_config;

pub(crate) mod shape_cluster_endpoint;
