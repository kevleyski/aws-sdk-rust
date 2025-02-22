// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_profiling_group_payload(
    body: &[u8],
) -> std::result::Result<
    std::option::Option<crate::types::ProfilingGroupDescription>,
    crate::operation::describe_profiling_group::DescribeProfilingGroupError,
> {
    (!body.is_empty()).then(||{
        crate::protocol_serde::shape_profiling_group_description::de_profiling_group_description_payload(body).map_err(crate::operation::describe_profiling_group::DescribeProfilingGroupError::unhandled)
    }).transpose()
}
