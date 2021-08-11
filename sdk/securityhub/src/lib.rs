#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
//! <p>Security Hub provides you with a comprehensive view of the security state of your Amazon Web Services environment and resources. It also provides you with the readiness status
//! of your environment based on controls from supported security standards. Security Hub collects
//! security data from Amazon Web Services accounts, services, and integrated third-party products and helps
//! you analyze security trends in your environment to identify the highest priority security
//! issues. For more information about Security Hub, see the <i>Security Hub<a href="https://docs.aws.amazon.com/securityhub/latest/userguide/what-is-securityhub.html">User
//! Guide</a>
//! </i>.</p>
//! <p>When you use operations in the Security Hub API, the requests are executed only in the Amazon Web Services
//! Region that is currently active or in the specific Amazon Web Services Region that you specify in your
//! request. Any configuration or settings change that results from the operation is applied
//! only to that Region. To make the same change in other Regions, execute the same command for
//! each Region to apply the change to.</p>
//! <p>For example, if your Region is set to <code>us-west-2</code>, when you use <code>CreateMembers</code> to add a member account to Security Hub, the association of
//! the member account with the administrator account is created only in the <code>us-west-2</code>
//! Region. Security Hub must be enabled for the member account in the same Region that the invitation
//! was sent from.</p>
//! <p>The following throttling limits apply to using Security Hub API operations.</p>
//! <ul>
//! <li>
//! <p>
//! <code>BatchEnableStandards</code> - <code>RateLimit</code> of 1
//! request per second, <code>BurstLimit</code> of 1 request per second.</p>
//! </li>
//! <li>
//! <p>
//! <code>GetFindings</code> - <code>RateLimit</code> of 3 requests per second.
//! <code>BurstLimit</code> of 6 requests per second.</p>
//! </li>
//! <li>
//! <p>
//! <code>UpdateFindings</code> - <code>RateLimit</code> of 1 request per
//! second. <code>BurstLimit</code> of 5 requests per second.</p>
//! </li>
//! <li>
//! <p>
//! <code>UpdateStandardsControl</code> - <code>RateLimit</code> of
//! 1 request per second, <code>BurstLimit</code> of 5 requests per second.</p>
//! </li>
//! <li>
//! <p>All other operations - <code>RateLimit</code> of 10 requests per second.
//! <code>BurstLimit</code> of 30 requests per second.</p>
//! </li>
//! </ul>

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
    aws_http::user_agent::ApiMetadata::new("securityhub", PKG_VERSION);
pub use aws_auth::Credentials;
pub use aws_types::region::Region;
#[cfg(feature = "client")]
pub use client::Client;
pub use smithy_http::endpoint::Endpoint;
