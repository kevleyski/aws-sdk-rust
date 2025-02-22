// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_configuration_payload(
    body: &[u8],
) -> std::result::Result<
    std::option::Option<crate::types::AgentConfiguration>,
    crate::operation::configure_agent::ConfigureAgentError,
> {
    (!body.is_empty())
        .then(|| {
            crate::protocol_serde::shape_agent_configuration::de_agent_configuration_payload(body)
                .map_err(crate::operation::configure_agent::ConfigureAgentError::unhandled)
        })
        .transpose()
}
