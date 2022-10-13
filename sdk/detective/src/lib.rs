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
//! <p>Detective uses machine learning and purpose-built visualizations to help you to
//! analyze and investigate security issues across your Amazon Web Services (Amazon Web Services) workloads. Detective automatically extracts time-based events such
//! as login attempts, API calls, and network traffic from CloudTrail and Amazon Virtual Private Cloud (Amazon VPC) flow logs. It also extracts findings detected by
//! Amazon GuardDuty.</p>
//! <p>The Detective API primarily supports the creation and management of behavior
//! graphs. A behavior graph contains the extracted data from a set of member accounts, and is
//! created and managed by an administrator account.</p>
//! <p>To add a member account to the behavior graph, the administrator account sends an
//! invitation to the account. When the account accepts the invitation, it becomes a member
//! account in the behavior graph.</p>
//! <p>Detective is also integrated with Organizations. The organization
//! management account designates the Detective administrator account for the
//! organization. That account becomes the administrator account for the organization behavior
//! graph. The Detective administrator account is also the delegated administrator
//! account for Detective in Organizations.</p>
//! <p>The Detective administrator account can enable any organization account as a
//! member account in the organization behavior graph. The organization accounts do not receive
//! invitations. The Detective administrator account can also invite other accounts to
//! the organization behavior graph.</p>
//! <p>Every behavior graph is specific to a Region. You can only use the API to manage
//! behavior graphs that belong to the Region that is associated with the currently selected
//! endpoint.</p>
//! <p>The administrator account for a behavior graph can use the Detective API to do
//! the following:</p>
//! <ul>
//! <li>
//! <p>Enable and disable Detective. Enabling Detective creates a new
//! behavior graph.</p>
//! </li>
//! <li>
//! <p>View the list of member accounts in a behavior graph.</p>
//! </li>
//! <li>
//! <p>Add member accounts to a behavior graph.</p>
//! </li>
//! <li>
//! <p>Remove member accounts from a behavior graph.</p>
//! </li>
//! <li>
//! <p>Apply tags to a behavior graph.</p>
//! </li>
//! </ul>
//! <p>The organization management account can use the Detective API to select the
//! delegated administrator for Detective.</p>
//! <p>The Detective administrator account for an organization can use the Detective API to do the following:</p>
//! <ul>
//! <li>
//! <p>Perform all of the functions of an administrator account.</p>
//! </li>
//! <li>
//! <p>Determine whether to automatically enable new organization accounts as member
//! accounts in the organization behavior graph.</p>
//! </li>
//! </ul>
//! <p>An invited member account can use the Detective API to do the following:</p>
//! <ul>
//! <li>
//! <p>View the list of behavior graphs that they are invited to.</p>
//! </li>
//! <li>
//! <p>Accept an invitation to contribute to a behavior graph.</p>
//! </li>
//! <li>
//! <p>Decline an invitation to contribute to a behavior graph.</p>
//! </li>
//! <li>
//! <p>Remove their account from a behavior graph.</p>
//! </li>
//! </ul>
//! <p>All API actions are logged as CloudTrail events. See <a href="https://docs.aws.amazon.com/detective/latest/adminguide/logging-using-cloudtrail.html">Logging Detective API Calls with CloudTrail</a>.</p>
//! <note>
//! <p>We replaced the term "master account" with the term "administrator account." An
//! administrator account is used to centrally manage multiple accounts. In the case of
//! Detective, the administrator account manages the accounts in their behavior
//! graph.</p>
//! </note>
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
    pub use aws_smithy_types::DateTime;
}
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("detective", PKG_VERSION);
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_types::app_name::AppName;
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[doc(inline)]
pub use client::Client;
