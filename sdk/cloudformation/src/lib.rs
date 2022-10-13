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
//! <fullname>CloudFormation</fullname>
//! <p>CloudFormation allows you to create and manage Amazon Web Services infrastructure
//! deployments predictably and repeatedly. You can use CloudFormation to leverage
//! Amazon Web Services products, such as Amazon Elastic Compute Cloud, Amazon Elastic Block Store,
//! Amazon Simple Notification Service, Elastic Load Balancing, and Auto Scaling to build highly
//! reliable, highly scalable, cost-effective applications without creating or configuring the
//! underlying Amazon Web Services infrastructure.</p>
//! <p>With CloudFormation, you declare all your resources and dependencies in a template
//! file. The template defines a collection of resources as a single unit called a stack.
//! CloudFormation creates and deletes all member resources of the stack together and
//! manages all dependencies between the resources for you.</p>
//! <p>For more information about CloudFormation, see the <a href="http://aws.amazon.com/cloudformation/">CloudFormation product page</a>.</p>
//! <p>CloudFormation makes use of other Amazon Web Services products. If you need
//! additional technical information about a specific Amazon Web Services product, you can find
//! the product's technical documentation at <a href="https://docs.aws.amazon.com/">
//! <code>docs.aws.amazon.com</code>
//! </a>.</p>
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
//!
//! # Examples
//! Examples can be found [here](https://github.com/awslabs/aws-sdk-rust/tree/main/examples/cloudformation).

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
mod idempotency_token;
/// Input structures for operations.
pub mod input;
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
mod query_ser;
mod rest_xml_wrapped_errors;
mod xml_deser;
/// Crate version number.
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
/// Re-exported types from supporting crates.
pub mod types {
    pub use aws_smithy_http::result::SdkError;
    pub use aws_smithy_types::DateTime;
}
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("cloudformation", PKG_VERSION);
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_types::app_name::AppName;
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[doc(inline)]
pub use client::Client;
