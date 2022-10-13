// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `AddTagsToStream`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`add_tags_to_stream`](crate::client::Client::add_tags_to_stream).
///
/// See [`crate::client::fluent_builders::AddTagsToStream`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct AddTagsToStream {
    _private: (),
}
impl AddTagsToStream {
    /// Creates a new builder-style object to manufacture [`AddTagsToStreamInput`](crate::input::AddTagsToStreamInput).
    pub fn builder() -> crate::input::add_tags_to_stream_input::Builder {
        crate::input::add_tags_to_stream_input::Builder::default()
    }
    /// Creates a new `AddTagsToStream` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for AddTagsToStream {
    type Output = std::result::Result<
        crate::output::AddTagsToStreamOutput,
        crate::error::AddTagsToStreamError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_add_tags_to_stream_error(response)
        } else {
            crate::operation_deser::parse_add_tags_to_stream_response(response)
        }
    }
}

/// Operation shape for `CreateStream`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_stream`](crate::client::Client::create_stream).
///
/// See [`crate::client::fluent_builders::CreateStream`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateStream {
    _private: (),
}
impl CreateStream {
    /// Creates a new builder-style object to manufacture [`CreateStreamInput`](crate::input::CreateStreamInput).
    pub fn builder() -> crate::input::create_stream_input::Builder {
        crate::input::create_stream_input::Builder::default()
    }
    /// Creates a new `CreateStream` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateStream {
    type Output =
        std::result::Result<crate::output::CreateStreamOutput, crate::error::CreateStreamError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_stream_error(response)
        } else {
            crate::operation_deser::parse_create_stream_response(response)
        }
    }
}

/// Operation shape for `DecreaseStreamRetentionPeriod`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`decrease_stream_retention_period`](crate::client::Client::decrease_stream_retention_period).
///
/// See [`crate::client::fluent_builders::DecreaseStreamRetentionPeriod`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DecreaseStreamRetentionPeriod {
    _private: (),
}
impl DecreaseStreamRetentionPeriod {
    /// Creates a new builder-style object to manufacture [`DecreaseStreamRetentionPeriodInput`](crate::input::DecreaseStreamRetentionPeriodInput).
    pub fn builder() -> crate::input::decrease_stream_retention_period_input::Builder {
        crate::input::decrease_stream_retention_period_input::Builder::default()
    }
    /// Creates a new `DecreaseStreamRetentionPeriod` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DecreaseStreamRetentionPeriod {
    type Output = std::result::Result<
        crate::output::DecreaseStreamRetentionPeriodOutput,
        crate::error::DecreaseStreamRetentionPeriodError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_decrease_stream_retention_period_error(response)
        } else {
            crate::operation_deser::parse_decrease_stream_retention_period_response(response)
        }
    }
}

/// Operation shape for `DeleteStream`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_stream`](crate::client::Client::delete_stream).
///
/// See [`crate::client::fluent_builders::DeleteStream`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteStream {
    _private: (),
}
impl DeleteStream {
    /// Creates a new builder-style object to manufacture [`DeleteStreamInput`](crate::input::DeleteStreamInput).
    pub fn builder() -> crate::input::delete_stream_input::Builder {
        crate::input::delete_stream_input::Builder::default()
    }
    /// Creates a new `DeleteStream` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteStream {
    type Output =
        std::result::Result<crate::output::DeleteStreamOutput, crate::error::DeleteStreamError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_stream_error(response)
        } else {
            crate::operation_deser::parse_delete_stream_response(response)
        }
    }
}

/// Operation shape for `DeregisterStreamConsumer`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`deregister_stream_consumer`](crate::client::Client::deregister_stream_consumer).
///
/// See [`crate::client::fluent_builders::DeregisterStreamConsumer`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeregisterStreamConsumer {
    _private: (),
}
impl DeregisterStreamConsumer {
    /// Creates a new builder-style object to manufacture [`DeregisterStreamConsumerInput`](crate::input::DeregisterStreamConsumerInput).
    pub fn builder() -> crate::input::deregister_stream_consumer_input::Builder {
        crate::input::deregister_stream_consumer_input::Builder::default()
    }
    /// Creates a new `DeregisterStreamConsumer` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeregisterStreamConsumer {
    type Output = std::result::Result<
        crate::output::DeregisterStreamConsumerOutput,
        crate::error::DeregisterStreamConsumerError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_deregister_stream_consumer_error(response)
        } else {
            crate::operation_deser::parse_deregister_stream_consumer_response(response)
        }
    }
}

/// Operation shape for `DescribeLimits`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_limits`](crate::client::Client::describe_limits).
///
/// See [`crate::client::fluent_builders::DescribeLimits`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeLimits {
    _private: (),
}
impl DescribeLimits {
    /// Creates a new builder-style object to manufacture [`DescribeLimitsInput`](crate::input::DescribeLimitsInput).
    pub fn builder() -> crate::input::describe_limits_input::Builder {
        crate::input::describe_limits_input::Builder::default()
    }
    /// Creates a new `DescribeLimits` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeLimits {
    type Output =
        std::result::Result<crate::output::DescribeLimitsOutput, crate::error::DescribeLimitsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_limits_error(response)
        } else {
            crate::operation_deser::parse_describe_limits_response(response)
        }
    }
}

/// Operation shape for `DescribeStream`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_stream`](crate::client::Client::describe_stream).
///
/// See [`crate::client::fluent_builders::DescribeStream`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeStream {
    _private: (),
}
impl DescribeStream {
    /// Creates a new builder-style object to manufacture [`DescribeStreamInput`](crate::input::DescribeStreamInput).
    pub fn builder() -> crate::input::describe_stream_input::Builder {
        crate::input::describe_stream_input::Builder::default()
    }
    /// Creates a new `DescribeStream` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeStream {
    type Output =
        std::result::Result<crate::output::DescribeStreamOutput, crate::error::DescribeStreamError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_stream_error(response)
        } else {
            crate::operation_deser::parse_describe_stream_response(response)
        }
    }
}

/// Operation shape for `DescribeStreamConsumer`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_stream_consumer`](crate::client::Client::describe_stream_consumer).
///
/// See [`crate::client::fluent_builders::DescribeStreamConsumer`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeStreamConsumer {
    _private: (),
}
impl DescribeStreamConsumer {
    /// Creates a new builder-style object to manufacture [`DescribeStreamConsumerInput`](crate::input::DescribeStreamConsumerInput).
    pub fn builder() -> crate::input::describe_stream_consumer_input::Builder {
        crate::input::describe_stream_consumer_input::Builder::default()
    }
    /// Creates a new `DescribeStreamConsumer` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeStreamConsumer {
    type Output = std::result::Result<
        crate::output::DescribeStreamConsumerOutput,
        crate::error::DescribeStreamConsumerError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_stream_consumer_error(response)
        } else {
            crate::operation_deser::parse_describe_stream_consumer_response(response)
        }
    }
}

/// Operation shape for `DescribeStreamSummary`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_stream_summary`](crate::client::Client::describe_stream_summary).
///
/// See [`crate::client::fluent_builders::DescribeStreamSummary`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeStreamSummary {
    _private: (),
}
impl DescribeStreamSummary {
    /// Creates a new builder-style object to manufacture [`DescribeStreamSummaryInput`](crate::input::DescribeStreamSummaryInput).
    pub fn builder() -> crate::input::describe_stream_summary_input::Builder {
        crate::input::describe_stream_summary_input::Builder::default()
    }
    /// Creates a new `DescribeStreamSummary` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeStreamSummary {
    type Output = std::result::Result<
        crate::output::DescribeStreamSummaryOutput,
        crate::error::DescribeStreamSummaryError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_stream_summary_error(response)
        } else {
            crate::operation_deser::parse_describe_stream_summary_response(response)
        }
    }
}

/// Operation shape for `DisableEnhancedMonitoring`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`disable_enhanced_monitoring`](crate::client::Client::disable_enhanced_monitoring).
///
/// See [`crate::client::fluent_builders::DisableEnhancedMonitoring`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DisableEnhancedMonitoring {
    _private: (),
}
impl DisableEnhancedMonitoring {
    /// Creates a new builder-style object to manufacture [`DisableEnhancedMonitoringInput`](crate::input::DisableEnhancedMonitoringInput).
    pub fn builder() -> crate::input::disable_enhanced_monitoring_input::Builder {
        crate::input::disable_enhanced_monitoring_input::Builder::default()
    }
    /// Creates a new `DisableEnhancedMonitoring` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DisableEnhancedMonitoring {
    type Output = std::result::Result<
        crate::output::DisableEnhancedMonitoringOutput,
        crate::error::DisableEnhancedMonitoringError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_disable_enhanced_monitoring_error(response)
        } else {
            crate::operation_deser::parse_disable_enhanced_monitoring_response(response)
        }
    }
}

/// Operation shape for `EnableEnhancedMonitoring`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`enable_enhanced_monitoring`](crate::client::Client::enable_enhanced_monitoring).
///
/// See [`crate::client::fluent_builders::EnableEnhancedMonitoring`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct EnableEnhancedMonitoring {
    _private: (),
}
impl EnableEnhancedMonitoring {
    /// Creates a new builder-style object to manufacture [`EnableEnhancedMonitoringInput`](crate::input::EnableEnhancedMonitoringInput).
    pub fn builder() -> crate::input::enable_enhanced_monitoring_input::Builder {
        crate::input::enable_enhanced_monitoring_input::Builder::default()
    }
    /// Creates a new `EnableEnhancedMonitoring` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for EnableEnhancedMonitoring {
    type Output = std::result::Result<
        crate::output::EnableEnhancedMonitoringOutput,
        crate::error::EnableEnhancedMonitoringError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_enable_enhanced_monitoring_error(response)
        } else {
            crate::operation_deser::parse_enable_enhanced_monitoring_response(response)
        }
    }
}

/// Operation shape for `GetRecords`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_records`](crate::client::Client::get_records).
///
/// See [`crate::client::fluent_builders::GetRecords`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetRecords {
    _private: (),
}
impl GetRecords {
    /// Creates a new builder-style object to manufacture [`GetRecordsInput`](crate::input::GetRecordsInput).
    pub fn builder() -> crate::input::get_records_input::Builder {
        crate::input::get_records_input::Builder::default()
    }
    /// Creates a new `GetRecords` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetRecords {
    type Output =
        std::result::Result<crate::output::GetRecordsOutput, crate::error::GetRecordsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_records_error(response)
        } else {
            crate::operation_deser::parse_get_records_response(response)
        }
    }
}

/// Operation shape for `GetShardIterator`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_shard_iterator`](crate::client::Client::get_shard_iterator).
///
/// See [`crate::client::fluent_builders::GetShardIterator`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetShardIterator {
    _private: (),
}
impl GetShardIterator {
    /// Creates a new builder-style object to manufacture [`GetShardIteratorInput`](crate::input::GetShardIteratorInput).
    pub fn builder() -> crate::input::get_shard_iterator_input::Builder {
        crate::input::get_shard_iterator_input::Builder::default()
    }
    /// Creates a new `GetShardIterator` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetShardIterator {
    type Output = std::result::Result<
        crate::output::GetShardIteratorOutput,
        crate::error::GetShardIteratorError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_shard_iterator_error(response)
        } else {
            crate::operation_deser::parse_get_shard_iterator_response(response)
        }
    }
}

/// Operation shape for `IncreaseStreamRetentionPeriod`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`increase_stream_retention_period`](crate::client::Client::increase_stream_retention_period).
///
/// See [`crate::client::fluent_builders::IncreaseStreamRetentionPeriod`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct IncreaseStreamRetentionPeriod {
    _private: (),
}
impl IncreaseStreamRetentionPeriod {
    /// Creates a new builder-style object to manufacture [`IncreaseStreamRetentionPeriodInput`](crate::input::IncreaseStreamRetentionPeriodInput).
    pub fn builder() -> crate::input::increase_stream_retention_period_input::Builder {
        crate::input::increase_stream_retention_period_input::Builder::default()
    }
    /// Creates a new `IncreaseStreamRetentionPeriod` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for IncreaseStreamRetentionPeriod {
    type Output = std::result::Result<
        crate::output::IncreaseStreamRetentionPeriodOutput,
        crate::error::IncreaseStreamRetentionPeriodError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_increase_stream_retention_period_error(response)
        } else {
            crate::operation_deser::parse_increase_stream_retention_period_response(response)
        }
    }
}

/// Operation shape for `ListShards`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_shards`](crate::client::Client::list_shards).
///
/// See [`crate::client::fluent_builders::ListShards`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListShards {
    _private: (),
}
impl ListShards {
    /// Creates a new builder-style object to manufacture [`ListShardsInput`](crate::input::ListShardsInput).
    pub fn builder() -> crate::input::list_shards_input::Builder {
        crate::input::list_shards_input::Builder::default()
    }
    /// Creates a new `ListShards` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListShards {
    type Output =
        std::result::Result<crate::output::ListShardsOutput, crate::error::ListShardsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_shards_error(response)
        } else {
            crate::operation_deser::parse_list_shards_response(response)
        }
    }
}

/// Operation shape for `ListStreamConsumers`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_stream_consumers`](crate::client::Client::list_stream_consumers).
///
/// See [`crate::client::fluent_builders::ListStreamConsumers`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListStreamConsumers {
    _private: (),
}
impl ListStreamConsumers {
    /// Creates a new builder-style object to manufacture [`ListStreamConsumersInput`](crate::input::ListStreamConsumersInput).
    pub fn builder() -> crate::input::list_stream_consumers_input::Builder {
        crate::input::list_stream_consumers_input::Builder::default()
    }
    /// Creates a new `ListStreamConsumers` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListStreamConsumers {
    type Output = std::result::Result<
        crate::output::ListStreamConsumersOutput,
        crate::error::ListStreamConsumersError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_stream_consumers_error(response)
        } else {
            crate::operation_deser::parse_list_stream_consumers_response(response)
        }
    }
}

/// Operation shape for `ListStreams`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_streams`](crate::client::Client::list_streams).
///
/// See [`crate::client::fluent_builders::ListStreams`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListStreams {
    _private: (),
}
impl ListStreams {
    /// Creates a new builder-style object to manufacture [`ListStreamsInput`](crate::input::ListStreamsInput).
    pub fn builder() -> crate::input::list_streams_input::Builder {
        crate::input::list_streams_input::Builder::default()
    }
    /// Creates a new `ListStreams` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListStreams {
    type Output =
        std::result::Result<crate::output::ListStreamsOutput, crate::error::ListStreamsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_streams_error(response)
        } else {
            crate::operation_deser::parse_list_streams_response(response)
        }
    }
}

/// Operation shape for `ListTagsForStream`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_tags_for_stream`](crate::client::Client::list_tags_for_stream).
///
/// See [`crate::client::fluent_builders::ListTagsForStream`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListTagsForStream {
    _private: (),
}
impl ListTagsForStream {
    /// Creates a new builder-style object to manufacture [`ListTagsForStreamInput`](crate::input::ListTagsForStreamInput).
    pub fn builder() -> crate::input::list_tags_for_stream_input::Builder {
        crate::input::list_tags_for_stream_input::Builder::default()
    }
    /// Creates a new `ListTagsForStream` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListTagsForStream {
    type Output = std::result::Result<
        crate::output::ListTagsForStreamOutput,
        crate::error::ListTagsForStreamError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_tags_for_stream_error(response)
        } else {
            crate::operation_deser::parse_list_tags_for_stream_response(response)
        }
    }
}

/// Operation shape for `MergeShards`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`merge_shards`](crate::client::Client::merge_shards).
///
/// See [`crate::client::fluent_builders::MergeShards`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct MergeShards {
    _private: (),
}
impl MergeShards {
    /// Creates a new builder-style object to manufacture [`MergeShardsInput`](crate::input::MergeShardsInput).
    pub fn builder() -> crate::input::merge_shards_input::Builder {
        crate::input::merge_shards_input::Builder::default()
    }
    /// Creates a new `MergeShards` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for MergeShards {
    type Output =
        std::result::Result<crate::output::MergeShardsOutput, crate::error::MergeShardsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_merge_shards_error(response)
        } else {
            crate::operation_deser::parse_merge_shards_response(response)
        }
    }
}

/// Operation shape for `PutRecord`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`put_record`](crate::client::Client::put_record).
///
/// See [`crate::client::fluent_builders::PutRecord`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct PutRecord {
    _private: (),
}
impl PutRecord {
    /// Creates a new builder-style object to manufacture [`PutRecordInput`](crate::input::PutRecordInput).
    pub fn builder() -> crate::input::put_record_input::Builder {
        crate::input::put_record_input::Builder::default()
    }
    /// Creates a new `PutRecord` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutRecord {
    type Output = std::result::Result<crate::output::PutRecordOutput, crate::error::PutRecordError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_put_record_error(response)
        } else {
            crate::operation_deser::parse_put_record_response(response)
        }
    }
}

/// Operation shape for `PutRecords`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`put_records`](crate::client::Client::put_records).
///
/// See [`crate::client::fluent_builders::PutRecords`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct PutRecords {
    _private: (),
}
impl PutRecords {
    /// Creates a new builder-style object to manufacture [`PutRecordsInput`](crate::input::PutRecordsInput).
    pub fn builder() -> crate::input::put_records_input::Builder {
        crate::input::put_records_input::Builder::default()
    }
    /// Creates a new `PutRecords` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutRecords {
    type Output =
        std::result::Result<crate::output::PutRecordsOutput, crate::error::PutRecordsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_put_records_error(response)
        } else {
            crate::operation_deser::parse_put_records_response(response)
        }
    }
}

/// Operation shape for `RegisterStreamConsumer`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`register_stream_consumer`](crate::client::Client::register_stream_consumer).
///
/// See [`crate::client::fluent_builders::RegisterStreamConsumer`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct RegisterStreamConsumer {
    _private: (),
}
impl RegisterStreamConsumer {
    /// Creates a new builder-style object to manufacture [`RegisterStreamConsumerInput`](crate::input::RegisterStreamConsumerInput).
    pub fn builder() -> crate::input::register_stream_consumer_input::Builder {
        crate::input::register_stream_consumer_input::Builder::default()
    }
    /// Creates a new `RegisterStreamConsumer` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for RegisterStreamConsumer {
    type Output = std::result::Result<
        crate::output::RegisterStreamConsumerOutput,
        crate::error::RegisterStreamConsumerError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_register_stream_consumer_error(response)
        } else {
            crate::operation_deser::parse_register_stream_consumer_response(response)
        }
    }
}

/// Operation shape for `RemoveTagsFromStream`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`remove_tags_from_stream`](crate::client::Client::remove_tags_from_stream).
///
/// See [`crate::client::fluent_builders::RemoveTagsFromStream`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct RemoveTagsFromStream {
    _private: (),
}
impl RemoveTagsFromStream {
    /// Creates a new builder-style object to manufacture [`RemoveTagsFromStreamInput`](crate::input::RemoveTagsFromStreamInput).
    pub fn builder() -> crate::input::remove_tags_from_stream_input::Builder {
        crate::input::remove_tags_from_stream_input::Builder::default()
    }
    /// Creates a new `RemoveTagsFromStream` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for RemoveTagsFromStream {
    type Output = std::result::Result<
        crate::output::RemoveTagsFromStreamOutput,
        crate::error::RemoveTagsFromStreamError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_remove_tags_from_stream_error(response)
        } else {
            crate::operation_deser::parse_remove_tags_from_stream_response(response)
        }
    }
}

/// Operation shape for `SplitShard`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`split_shard`](crate::client::Client::split_shard).
///
/// See [`crate::client::fluent_builders::SplitShard`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct SplitShard {
    _private: (),
}
impl SplitShard {
    /// Creates a new builder-style object to manufacture [`SplitShardInput`](crate::input::SplitShardInput).
    pub fn builder() -> crate::input::split_shard_input::Builder {
        crate::input::split_shard_input::Builder::default()
    }
    /// Creates a new `SplitShard` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for SplitShard {
    type Output =
        std::result::Result<crate::output::SplitShardOutput, crate::error::SplitShardError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_split_shard_error(response)
        } else {
            crate::operation_deser::parse_split_shard_response(response)
        }
    }
}

/// Operation shape for `StartStreamEncryption`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`start_stream_encryption`](crate::client::Client::start_stream_encryption).
///
/// See [`crate::client::fluent_builders::StartStreamEncryption`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StartStreamEncryption {
    _private: (),
}
impl StartStreamEncryption {
    /// Creates a new builder-style object to manufacture [`StartStreamEncryptionInput`](crate::input::StartStreamEncryptionInput).
    pub fn builder() -> crate::input::start_stream_encryption_input::Builder {
        crate::input::start_stream_encryption_input::Builder::default()
    }
    /// Creates a new `StartStreamEncryption` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StartStreamEncryption {
    type Output = std::result::Result<
        crate::output::StartStreamEncryptionOutput,
        crate::error::StartStreamEncryptionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_start_stream_encryption_error(response)
        } else {
            crate::operation_deser::parse_start_stream_encryption_response(response)
        }
    }
}

/// Operation shape for `StopStreamEncryption`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`stop_stream_encryption`](crate::client::Client::stop_stream_encryption).
///
/// See [`crate::client::fluent_builders::StopStreamEncryption`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StopStreamEncryption {
    _private: (),
}
impl StopStreamEncryption {
    /// Creates a new builder-style object to manufacture [`StopStreamEncryptionInput`](crate::input::StopStreamEncryptionInput).
    pub fn builder() -> crate::input::stop_stream_encryption_input::Builder {
        crate::input::stop_stream_encryption_input::Builder::default()
    }
    /// Creates a new `StopStreamEncryption` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StopStreamEncryption {
    type Output = std::result::Result<
        crate::output::StopStreamEncryptionOutput,
        crate::error::StopStreamEncryptionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_stop_stream_encryption_error(response)
        } else {
            crate::operation_deser::parse_stop_stream_encryption_response(response)
        }
    }
}

/// Operation shape for `UpdateShardCount`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_shard_count`](crate::client::Client::update_shard_count).
///
/// See [`crate::client::fluent_builders::UpdateShardCount`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateShardCount {
    _private: (),
}
impl UpdateShardCount {
    /// Creates a new builder-style object to manufacture [`UpdateShardCountInput`](crate::input::UpdateShardCountInput).
    pub fn builder() -> crate::input::update_shard_count_input::Builder {
        crate::input::update_shard_count_input::Builder::default()
    }
    /// Creates a new `UpdateShardCount` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateShardCount {
    type Output = std::result::Result<
        crate::output::UpdateShardCountOutput,
        crate::error::UpdateShardCountError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_shard_count_error(response)
        } else {
            crate::operation_deser::parse_update_shard_count_response(response)
        }
    }
}

/// Operation shape for `UpdateStreamMode`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_stream_mode`](crate::client::Client::update_stream_mode).
///
/// See [`crate::client::fluent_builders::UpdateStreamMode`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateStreamMode {
    _private: (),
}
impl UpdateStreamMode {
    /// Creates a new builder-style object to manufacture [`UpdateStreamModeInput`](crate::input::UpdateStreamModeInput).
    pub fn builder() -> crate::input::update_stream_mode_input::Builder {
        crate::input::update_stream_mode_input::Builder::default()
    }
    /// Creates a new `UpdateStreamMode` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateStreamMode {
    type Output = std::result::Result<
        crate::output::UpdateStreamModeOutput,
        crate::error::UpdateStreamModeError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_stream_mode_error(response)
        } else {
            crate::operation_deser::parse_update_stream_mode_response(response)
        }
    }
}

/// Operation customization and supporting types
pub mod customize;
