// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_associate_tracker_consumer_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AssociateTrackerConsumerInput,
) {
    if let Some(var_1) = &input.consumer_arn {
        object.key("ConsumerArn").string(var_1);
    }
}

pub fn serialize_structure_batch_delete_device_position_history_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchDeleteDevicePositionHistoryInput,
) {
    if let Some(var_2) = &input.device_ids {
        let mut array_3 = object.key("DeviceIds").start_array();
        for item_4 in var_2 {
            {
                array_3.value().string(item_4);
            }
        }
        array_3.finish();
    }
}

pub fn serialize_structure_batch_delete_geofence_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchDeleteGeofenceInput,
) {
    if let Some(var_5) = &input.geofence_ids {
        let mut array_6 = object.key("GeofenceIds").start_array();
        for item_7 in var_5 {
            {
                array_6.value().string(item_7);
            }
        }
        array_6.finish();
    }
}

pub fn serialize_structure_batch_evaluate_geofences_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchEvaluateGeofencesInput,
) {
    if let Some(var_8) = &input.device_position_updates {
        let mut array_9 = object.key("DevicePositionUpdates").start_array();
        for item_10 in var_8 {
            {
                let mut object_11 = array_9.value().start_object();
                crate::json_ser::serialize_structure_device_position_update(
                    &mut object_11,
                    item_10,
                );
                object_11.finish();
            }
        }
        array_9.finish();
    }
}

pub fn serialize_structure_batch_get_device_position_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchGetDevicePositionInput,
) {
    if let Some(var_12) = &input.device_ids {
        let mut array_13 = object.key("DeviceIds").start_array();
        for item_14 in var_12 {
            {
                array_13.value().string(item_14);
            }
        }
        array_13.finish();
    }
}

pub fn serialize_structure_batch_put_geofence_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchPutGeofenceInput,
) {
    if let Some(var_15) = &input.entries {
        let mut array_16 = object.key("Entries").start_array();
        for item_17 in var_15 {
            {
                let mut object_18 = array_16.value().start_object();
                crate::json_ser::serialize_structure_batch_put_geofence_request_entry(
                    &mut object_18,
                    item_17,
                );
                object_18.finish();
            }
        }
        array_16.finish();
    }
}

pub fn serialize_structure_batch_update_device_position_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchUpdateDevicePositionInput,
) {
    if let Some(var_19) = &input.updates {
        let mut array_20 = object.key("Updates").start_array();
        for item_21 in var_19 {
            {
                let mut object_22 = array_20.value().start_object();
                crate::json_ser::serialize_structure_device_position_update(
                    &mut object_22,
                    item_21,
                );
                object_22.finish();
            }
        }
        array_20.finish();
    }
}

pub fn serialize_structure_calculate_route_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CalculateRouteInput,
) {
    if let Some(var_23) = &input.car_mode_options {
        let mut object_24 = object.key("CarModeOptions").start_object();
        crate::json_ser::serialize_structure_calculate_route_car_mode_options(
            &mut object_24,
            var_23,
        );
        object_24.finish();
    }
    if let Some(var_25) = &input.depart_now {
        object.key("DepartNow").boolean(*var_25);
    }
    if let Some(var_26) = &input.departure_position {
        let mut array_27 = object.key("DeparturePosition").start_array();
        for item_28 in var_26 {
            {
                array_27.value().number(
                    #[allow(clippy::useless_conversion)]
                    smithy_types::Number::Float((*item_28).into()),
                );
            }
        }
        array_27.finish();
    }
    if let Some(var_29) = &input.departure_time {
        object
            .key("DepartureTime")
            .instant(var_29, smithy_types::instant::Format::DateTime);
    }
    if let Some(var_30) = &input.destination_position {
        let mut array_31 = object.key("DestinationPosition").start_array();
        for item_32 in var_30 {
            {
                array_31.value().number(
                    #[allow(clippy::useless_conversion)]
                    smithy_types::Number::Float((*item_32).into()),
                );
            }
        }
        array_31.finish();
    }
    if let Some(var_33) = &input.distance_unit {
        object.key("DistanceUnit").string(var_33.as_str());
    }
    if let Some(var_34) = &input.include_leg_geometry {
        object.key("IncludeLegGeometry").boolean(*var_34);
    }
    if let Some(var_35) = &input.travel_mode {
        object.key("TravelMode").string(var_35.as_str());
    }
    if let Some(var_36) = &input.truck_mode_options {
        let mut object_37 = object.key("TruckModeOptions").start_object();
        crate::json_ser::serialize_structure_calculate_route_truck_mode_options(
            &mut object_37,
            var_36,
        );
        object_37.finish();
    }
    if let Some(var_38) = &input.waypoint_positions {
        let mut array_39 = object.key("WaypointPositions").start_array();
        for item_40 in var_38 {
            {
                let mut array_41 = array_39.value().start_array();
                for item_42 in item_40 {
                    {
                        array_41.value().number(
                            #[allow(clippy::useless_conversion)]
                            smithy_types::Number::Float((*item_42).into()),
                        );
                    }
                }
                array_41.finish();
            }
        }
        array_39.finish();
    }
}

pub fn serialize_structure_create_geofence_collection_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateGeofenceCollectionInput,
) {
    if let Some(var_43) = &input.collection_name {
        object.key("CollectionName").string(var_43);
    }
    if let Some(var_44) = &input.description {
        object.key("Description").string(var_44);
    }
    if let Some(var_45) = &input.kms_key_id {
        object.key("KmsKeyId").string(var_45);
    }
    if let Some(var_46) = &input.pricing_plan {
        object.key("PricingPlan").string(var_46.as_str());
    }
    if let Some(var_47) = &input.pricing_plan_data_source {
        object.key("PricingPlanDataSource").string(var_47);
    }
    if let Some(var_48) = &input.tags {
        let mut object_49 = object.key("Tags").start_object();
        for (key_50, value_51) in var_48 {
            {
                object_49.key(key_50).string(value_51);
            }
        }
        object_49.finish();
    }
}

pub fn serialize_structure_create_map_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateMapInput,
) {
    if let Some(var_52) = &input.configuration {
        let mut object_53 = object.key("Configuration").start_object();
        crate::json_ser::serialize_structure_map_configuration(&mut object_53, var_52);
        object_53.finish();
    }
    if let Some(var_54) = &input.description {
        object.key("Description").string(var_54);
    }
    if let Some(var_55) = &input.map_name {
        object.key("MapName").string(var_55);
    }
    if let Some(var_56) = &input.pricing_plan {
        object.key("PricingPlan").string(var_56.as_str());
    }
    if let Some(var_57) = &input.tags {
        let mut object_58 = object.key("Tags").start_object();
        for (key_59, value_60) in var_57 {
            {
                object_58.key(key_59).string(value_60);
            }
        }
        object_58.finish();
    }
}

pub fn serialize_structure_create_place_index_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreatePlaceIndexInput,
) {
    if let Some(var_61) = &input.data_source {
        object.key("DataSource").string(var_61);
    }
    if let Some(var_62) = &input.data_source_configuration {
        let mut object_63 = object.key("DataSourceConfiguration").start_object();
        crate::json_ser::serialize_structure_data_source_configuration(&mut object_63, var_62);
        object_63.finish();
    }
    if let Some(var_64) = &input.description {
        object.key("Description").string(var_64);
    }
    if let Some(var_65) = &input.index_name {
        object.key("IndexName").string(var_65);
    }
    if let Some(var_66) = &input.pricing_plan {
        object.key("PricingPlan").string(var_66.as_str());
    }
    if let Some(var_67) = &input.tags {
        let mut object_68 = object.key("Tags").start_object();
        for (key_69, value_70) in var_67 {
            {
                object_68.key(key_69).string(value_70);
            }
        }
        object_68.finish();
    }
}

pub fn serialize_structure_create_route_calculator_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateRouteCalculatorInput,
) {
    if let Some(var_71) = &input.calculator_name {
        object.key("CalculatorName").string(var_71);
    }
    if let Some(var_72) = &input.data_source {
        object.key("DataSource").string(var_72);
    }
    if let Some(var_73) = &input.description {
        object.key("Description").string(var_73);
    }
    if let Some(var_74) = &input.pricing_plan {
        object.key("PricingPlan").string(var_74.as_str());
    }
    if let Some(var_75) = &input.tags {
        let mut object_76 = object.key("Tags").start_object();
        for (key_77, value_78) in var_75 {
            {
                object_76.key(key_77).string(value_78);
            }
        }
        object_76.finish();
    }
}

pub fn serialize_structure_create_tracker_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateTrackerInput,
) {
    if let Some(var_79) = &input.description {
        object.key("Description").string(var_79);
    }
    if let Some(var_80) = &input.kms_key_id {
        object.key("KmsKeyId").string(var_80);
    }
    if let Some(var_81) = &input.pricing_plan {
        object.key("PricingPlan").string(var_81.as_str());
    }
    if let Some(var_82) = &input.pricing_plan_data_source {
        object.key("PricingPlanDataSource").string(var_82);
    }
    if let Some(var_83) = &input.tags {
        let mut object_84 = object.key("Tags").start_object();
        for (key_85, value_86) in var_83 {
            {
                object_84.key(key_85).string(value_86);
            }
        }
        object_84.finish();
    }
    if let Some(var_87) = &input.tracker_name {
        object.key("TrackerName").string(var_87);
    }
}

pub fn serialize_structure_get_device_position_history_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetDevicePositionHistoryInput,
) {
    if let Some(var_88) = &input.end_time_exclusive {
        object
            .key("EndTimeExclusive")
            .instant(var_88, smithy_types::instant::Format::DateTime);
    }
    if let Some(var_89) = &input.next_token {
        object.key("NextToken").string(var_89);
    }
    if let Some(var_90) = &input.start_time_inclusive {
        object
            .key("StartTimeInclusive")
            .instant(var_90, smithy_types::instant::Format::DateTime);
    }
}

pub fn serialize_structure_list_device_positions_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListDevicePositionsInput,
) {
    if let Some(var_91) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_91).into()),
        );
    }
    if let Some(var_92) = &input.next_token {
        object.key("NextToken").string(var_92);
    }
}

pub fn serialize_structure_list_geofence_collections_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListGeofenceCollectionsInput,
) {
    if let Some(var_93) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_93).into()),
        );
    }
    if let Some(var_94) = &input.next_token {
        object.key("NextToken").string(var_94);
    }
}

pub fn serialize_structure_list_geofences_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListGeofencesInput,
) {
    if let Some(var_95) = &input.next_token {
        object.key("NextToken").string(var_95);
    }
}

pub fn serialize_structure_list_maps_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListMapsInput,
) {
    if let Some(var_96) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_96).into()),
        );
    }
    if let Some(var_97) = &input.next_token {
        object.key("NextToken").string(var_97);
    }
}

pub fn serialize_structure_list_place_indexes_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListPlaceIndexesInput,
) {
    if let Some(var_98) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_98).into()),
        );
    }
    if let Some(var_99) = &input.next_token {
        object.key("NextToken").string(var_99);
    }
}

pub fn serialize_structure_list_route_calculators_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListRouteCalculatorsInput,
) {
    if let Some(var_100) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_100).into()),
        );
    }
    if let Some(var_101) = &input.next_token {
        object.key("NextToken").string(var_101);
    }
}

pub fn serialize_structure_list_tracker_consumers_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTrackerConsumersInput,
) {
    if let Some(var_102) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_102).into()),
        );
    }
    if let Some(var_103) = &input.next_token {
        object.key("NextToken").string(var_103);
    }
}

pub fn serialize_structure_list_trackers_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTrackersInput,
) {
    if let Some(var_104) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_104).into()),
        );
    }
    if let Some(var_105) = &input.next_token {
        object.key("NextToken").string(var_105);
    }
}

pub fn serialize_structure_put_geofence_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutGeofenceInput,
) {
    if let Some(var_106) = &input.geometry {
        let mut object_107 = object.key("Geometry").start_object();
        crate::json_ser::serialize_structure_geofence_geometry(&mut object_107, var_106);
        object_107.finish();
    }
}

pub fn serialize_structure_search_place_index_for_position_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::SearchPlaceIndexForPositionInput,
) {
    if input.max_results != 0 {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_108) = &input.position {
        let mut array_109 = object.key("Position").start_array();
        for item_110 in var_108 {
            {
                array_109.value().number(
                    #[allow(clippy::useless_conversion)]
                    smithy_types::Number::Float((*item_110).into()),
                );
            }
        }
        array_109.finish();
    }
}

pub fn serialize_structure_search_place_index_for_text_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::SearchPlaceIndexForTextInput,
) {
    if let Some(var_111) = &input.bias_position {
        let mut array_112 = object.key("BiasPosition").start_array();
        for item_113 in var_111 {
            {
                array_112.value().number(
                    #[allow(clippy::useless_conversion)]
                    smithy_types::Number::Float((*item_113).into()),
                );
            }
        }
        array_112.finish();
    }
    if let Some(var_114) = &input.filter_b_box {
        let mut array_115 = object.key("FilterBBox").start_array();
        for item_116 in var_114 {
            {
                array_115.value().number(
                    #[allow(clippy::useless_conversion)]
                    smithy_types::Number::Float((*item_116).into()),
                );
            }
        }
        array_115.finish();
    }
    if let Some(var_117) = &input.filter_countries {
        let mut array_118 = object.key("FilterCountries").start_array();
        for item_119 in var_117 {
            {
                array_118.value().string(item_119);
            }
        }
        array_118.finish();
    }
    if input.max_results != 0 {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_120) = &input.text {
        object.key("Text").string(var_120);
    }
}

pub fn serialize_structure_tag_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) {
    if let Some(var_121) = &input.tags {
        let mut object_122 = object.key("Tags").start_object();
        for (key_123, value_124) in var_121 {
            {
                object_122.key(key_123).string(value_124);
            }
        }
        object_122.finish();
    }
}

pub fn serialize_structure_update_geofence_collection_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateGeofenceCollectionInput,
) {
    if let Some(var_125) = &input.description {
        object.key("Description").string(var_125);
    }
    if let Some(var_126) = &input.pricing_plan {
        object.key("PricingPlan").string(var_126.as_str());
    }
    if let Some(var_127) = &input.pricing_plan_data_source {
        object.key("PricingPlanDataSource").string(var_127);
    }
}

pub fn serialize_structure_update_map_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateMapInput,
) {
    if let Some(var_128) = &input.description {
        object.key("Description").string(var_128);
    }
    if let Some(var_129) = &input.pricing_plan {
        object.key("PricingPlan").string(var_129.as_str());
    }
}

pub fn serialize_structure_update_place_index_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdatePlaceIndexInput,
) {
    if let Some(var_130) = &input.data_source_configuration {
        let mut object_131 = object.key("DataSourceConfiguration").start_object();
        crate::json_ser::serialize_structure_data_source_configuration(&mut object_131, var_130);
        object_131.finish();
    }
    if let Some(var_132) = &input.description {
        object.key("Description").string(var_132);
    }
    if let Some(var_133) = &input.pricing_plan {
        object.key("PricingPlan").string(var_133.as_str());
    }
}

pub fn serialize_structure_update_route_calculator_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateRouteCalculatorInput,
) {
    if let Some(var_134) = &input.description {
        object.key("Description").string(var_134);
    }
    if let Some(var_135) = &input.pricing_plan {
        object.key("PricingPlan").string(var_135.as_str());
    }
}

pub fn serialize_structure_update_tracker_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateTrackerInput,
) {
    if let Some(var_136) = &input.description {
        object.key("Description").string(var_136);
    }
    if let Some(var_137) = &input.pricing_plan {
        object.key("PricingPlan").string(var_137.as_str());
    }
    if let Some(var_138) = &input.pricing_plan_data_source {
        object.key("PricingPlanDataSource").string(var_138);
    }
}

pub fn serialize_structure_device_position_update(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DevicePositionUpdate,
) {
    if let Some(var_139) = &input.device_id {
        object.key("DeviceId").string(var_139);
    }
    if let Some(var_140) = &input.sample_time {
        object
            .key("SampleTime")
            .instant(var_140, smithy_types::instant::Format::DateTime);
    }
    if let Some(var_141) = &input.position {
        let mut array_142 = object.key("Position").start_array();
        for item_143 in var_141 {
            {
                array_142.value().number(
                    #[allow(clippy::useless_conversion)]
                    smithy_types::Number::Float((*item_143).into()),
                );
            }
        }
        array_142.finish();
    }
}

pub fn serialize_structure_batch_put_geofence_request_entry(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::BatchPutGeofenceRequestEntry,
) {
    if let Some(var_144) = &input.geofence_id {
        object.key("GeofenceId").string(var_144);
    }
    if let Some(var_145) = &input.geometry {
        let mut object_146 = object.key("Geometry").start_object();
        crate::json_ser::serialize_structure_geofence_geometry(&mut object_146, var_145);
        object_146.finish();
    }
}

pub fn serialize_structure_calculate_route_car_mode_options(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CalculateRouteCarModeOptions,
) {
    if let Some(var_147) = &input.avoid_ferries {
        object.key("AvoidFerries").boolean(*var_147);
    }
    if let Some(var_148) = &input.avoid_tolls {
        object.key("AvoidTolls").boolean(*var_148);
    }
}

pub fn serialize_structure_calculate_route_truck_mode_options(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CalculateRouteTruckModeOptions,
) {
    if let Some(var_149) = &input.avoid_ferries {
        object.key("AvoidFerries").boolean(*var_149);
    }
    if let Some(var_150) = &input.avoid_tolls {
        object.key("AvoidTolls").boolean(*var_150);
    }
    if let Some(var_151) = &input.dimensions {
        let mut object_152 = object.key("Dimensions").start_object();
        crate::json_ser::serialize_structure_truck_dimensions(&mut object_152, var_151);
        object_152.finish();
    }
    if let Some(var_153) = &input.weight {
        let mut object_154 = object.key("Weight").start_object();
        crate::json_ser::serialize_structure_truck_weight(&mut object_154, var_153);
        object_154.finish();
    }
}

pub fn serialize_structure_map_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::MapConfiguration,
) {
    if let Some(var_155) = &input.style {
        object.key("Style").string(var_155);
    }
}

pub fn serialize_structure_data_source_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DataSourceConfiguration,
) {
    if let Some(var_156) = &input.intended_use {
        object.key("IntendedUse").string(var_156.as_str());
    }
}

pub fn serialize_structure_geofence_geometry(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::GeofenceGeometry,
) {
    if let Some(var_157) = &input.polygon {
        let mut array_158 = object.key("Polygon").start_array();
        for item_159 in var_157 {
            {
                let mut array_160 = array_158.value().start_array();
                for item_161 in item_159 {
                    {
                        let mut array_162 = array_160.value().start_array();
                        for item_163 in item_161 {
                            {
                                array_162.value().number(
                                    #[allow(clippy::useless_conversion)]
                                    smithy_types::Number::Float((*item_163).into()),
                                );
                            }
                        }
                        array_162.finish();
                    }
                }
                array_160.finish();
            }
        }
        array_158.finish();
    }
}

pub fn serialize_structure_truck_dimensions(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::TruckDimensions,
) {
    if let Some(var_164) = &input.length {
        object.key("Length").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::Float((*var_164).into()),
        );
    }
    if let Some(var_165) = &input.height {
        object.key("Height").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::Float((*var_165).into()),
        );
    }
    if let Some(var_166) = &input.width {
        object.key("Width").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::Float((*var_166).into()),
        );
    }
    if let Some(var_167) = &input.unit {
        object.key("Unit").string(var_167.as_str());
    }
}

pub fn serialize_structure_truck_weight(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::TruckWeight,
) {
    if let Some(var_168) = &input.total {
        object.key("Total").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::Float((*var_168).into()),
        );
    }
    if let Some(var_169) = &input.unit {
        object.key("Unit").string(var_169.as_str());
    }
}
