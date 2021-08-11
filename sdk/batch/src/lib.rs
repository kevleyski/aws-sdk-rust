#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
//! <fullname>Batch</fullname>
//! <p>Using Batch, you can run batch computing workloads on the Cloud. Batch computing is a common means for
//! developers, scientists, and engineers to access large amounts of compute resources. Batch uses the advantages of
//! this computing workload to remove the undifferentiated heavy lifting of configuring and managing required
//! infrastructure. At the same time, it also adopts a familiar batch computing software approach. Given these
//! advantages, Batch can help you to efficiently provision resources in response to jobs submitted, thus effectively
//! helping you to eliminate capacity constraints, reduce compute costs, and deliver your results more quickly.</p>
//! <p>As a fully managed service, Batch can run batch computing workloads of any scale. Batch automatically
//! provisions compute resources and optimizes workload distribution based on the quantity and scale of your specific
//! workloads. With Batch, there's no need to install or manage batch computing software. This means that you can focus
//! your time and energy on analyzing results and solving your specific problems. </p>

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

pub use config::Config;

mod aws_endpoint;
#[cfg(feature = "client")]
pub mod client;
pub mod config;
pub mod error;
mod error_meta;
pub mod input;
mod json_deser;
mod json_errors;
mod json_ser;
pub mod model;
pub mod operation;
mod operation_deser;
mod operation_ser;
pub mod output;
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub use smithy_http::byte_stream::ByteStream;
pub use smithy_http::result::SdkError;
pub use smithy_types::Blob;
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("batch", PKG_VERSION);
pub use aws_auth::Credentials;
pub use aws_types::region::Region;
#[cfg(feature = "client")]
pub use client::Client;
pub use smithy_http::endpoint::Endpoint;
