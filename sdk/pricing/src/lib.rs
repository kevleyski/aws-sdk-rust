#![allow(deprecated)]
#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(clippy::type_complexity)]
#![allow(rustdoc::bare_urls)]
#![warn(missing_docs)]
//! <p>Amazon Web Services Price List API is a centralized and convenient way to
//! programmatically query Amazon Web Services for services, products, and pricing information. The Amazon Web Services Price List
//! uses standardized product attributes such as <code>Location</code>, <code>Storage
//! Class</code>, and <code>Operating System</code>, and provides prices at the SKU
//! level. You can use the Amazon Web Services Price List to build cost control and scenario planning tools, reconcile
//! billing data, forecast future spend for budgeting purposes, and provide cost benefit
//! analysis that compare your internal workloads with Amazon Web Services.</p>
//! <p>Use <code>GetServices</code> without a service code to retrieve the service codes for all AWS services, then
//! <code>GetServices</code> with a service code to retrieve the attribute names for
//! that service. After you have the service code and attribute names, you can use <code>GetAttributeValues</code>
//! to see what values are available for an attribute. With the service code and an attribute name and value,
//! you can use <code>GetProducts</code> to find specific products that you're interested in, such as
//! an <code>AmazonEC2</code> instance, with a <code>Provisioned IOPS</code>
//! <code>volumeType</code>.</p>
//! <p>Service Endpoint</p>
//! <p>Amazon Web Services Price List service API provides the following two endpoints:</p>
//! <ul>
//! <li>
//! <p>https://api.pricing.us-east-1.amazonaws.com</p>
//! </li>
//! <li>
//! <p>https://api.pricing.ap-south-1.amazonaws.com</p>
//! </li>
//! </ul>
//!
//! # Crate Organization
//!
//! The entry point for most customers will be [`Client`]. [`Client`] exposes one method for each API offered
//! by the service.
//!
//! Some APIs require complex or nested arguments. These exist in [`model`](crate::model).
//!
//! Lastly, errors that can be returned by the service are contained within [`error`]. [`Error`] defines a meta
//! error encompassing all possible errors that can be returned by the service.
//!
//! The other modules within this crate are not required for normal usage.

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

#[doc(inline)]
pub use config::Config;

mod aws_endpoint;
/// Client and fluent builders for calling the service.
pub mod client;
/// Configuration for the service.
pub mod config;
/// All error types that operations can return.
pub mod error;
mod error_meta;
/// Input structures for operations.
pub mod input;
mod json_deser;
mod json_errors;
mod json_ser;
/// Generated accessors for nested fields
pub mod lens;
pub mod middleware;
/// Data structures used by operation inputs/outputs.
pub mod model;
mod no_credentials;
/// All operations that this crate can perform.
pub mod operation;
mod operation_deser;
mod operation_ser;
/// Output structures for operations.
pub mod output;
/// Paginators for the service
pub mod paginator;
/// Crate version number.
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
/// Re-exported types from supporting crates.
pub mod types {
    pub use aws_smithy_http::result::SdkError;
}
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("pricing", PKG_VERSION);
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_types::app_name::AppName;
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[doc(inline)]
pub use client::Client;
