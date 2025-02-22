// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_record::_put_record_output::PutRecordOutputBuilder;

pub use crate::operation::put_record::_put_record_input::PutRecordInputBuilder;

/// Fluent builder constructing a request to `PutRecord`.
///
/// <p>Writes a single data record into an Amazon Kinesis data stream. Call <code>PutRecord</code> to send data into the stream for real-time ingestion and subsequent processing, one record at a time. Each shard can support writes up to 1,000 records per second, up to a maximum data write total of 1 MiB per second.</p> <note>
/// <p>When invoking this API, it is recommended you use the <code>StreamARN</code> input parameter rather than the <code>StreamName</code> input parameter.</p>
/// </note>
/// <p>You must specify the name of the stream that captures, stores, and transports the data; a partition key; and the data blob itself.</p>
/// <p>The data blob can be any type of data; for example, a segment from a log file, geographic/location data, website clickstream data, and so on.</p>
/// <p>The partition key is used by Kinesis Data Streams to distribute data across shards. Kinesis Data Streams segregates the data records that belong to a stream into multiple shards, using the partition key associated with each data record to determine the shard to which a given data record belongs.</p>
/// <p>Partition keys are Unicode strings, with a maximum length limit of 256 characters for each key. An MD5 hash function is used to map partition keys to 128-bit integer values and to map associated data records to shards using the hash key ranges of the shards. You can override hashing the partition key to determine the shard by explicitly specifying a hash value using the <code>ExplicitHashKey</code> parameter. For more information, see <a href="https://docs.aws.amazon.com/kinesis/latest/dev/developing-producers-with-sdk.html#kinesis-using-sdk-java-add-data-to-stream">Adding Data to a Stream</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>.</p>
/// <p> <code>PutRecord</code> returns the shard ID of where the data record was placed and the sequence number that was assigned to the data record.</p>
/// <p>Sequence numbers increase over time and are specific to a shard within a stream, not across all shards within a stream. To guarantee strictly increasing ordering, write serially to a shard and use the <code>SequenceNumberForOrdering</code> parameter. For more information, see <a href="https://docs.aws.amazon.com/kinesis/latest/dev/developing-producers-with-sdk.html#kinesis-using-sdk-java-add-data-to-stream">Adding Data to a Stream</a> in the <i>Amazon Kinesis Data Streams Developer Guide</i>.</p> <important>
/// <p>After you write a record to a stream, you cannot modify that record or its order within the stream.</p>
/// </important>
/// <p>If a <code>PutRecord</code> request cannot be processed because of insufficient provisioned throughput on the shard involved in the request, <code>PutRecord</code> throws <code>ProvisionedThroughputExceededException</code>. </p>
/// <p>By default, data records are accessible for 24 hours from the time that they are added to a stream. You can use <code>IncreaseStreamRetentionPeriod</code> or <code>DecreaseStreamRetentionPeriod</code> to modify this retention period.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct PutRecordFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::put_record::builders::PutRecordInputBuilder,
}
impl PutRecordFluentBuilder {
    /// Creates a new `PutRecord`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::put_record::PutRecord,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::put_record::PutRecordError>,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        Ok(crate::client::customize::CustomizableOperation { handle, operation })
    }

    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> std::result::Result<
        crate::operation::put_record::PutRecordOutput,
        aws_smithy_http::result::SdkError<crate::operation::put_record::PutRecordError>,
    > {
        let op = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// <p>The name of the stream to put the data record into.</p>
    pub fn stream_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.stream_name(input.into());
        self
    }
    /// <p>The name of the stream to put the data record into.</p>
    pub fn set_stream_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_stream_name(input);
        self
    }
    /// <p>The data blob to put into the record, which is base64-encoded when the blob is serialized. When the data blob (the payload before base64-encoding) is added to the partition key size, the total size must not exceed the maximum record size (1 MiB).</p>
    pub fn data(mut self, input: aws_smithy_types::Blob) -> Self {
        self.inner = self.inner.data(input);
        self
    }
    /// <p>The data blob to put into the record, which is base64-encoded when the blob is serialized. When the data blob (the payload before base64-encoding) is added to the partition key size, the total size must not exceed the maximum record size (1 MiB).</p>
    pub fn set_data(mut self, input: std::option::Option<aws_smithy_types::Blob>) -> Self {
        self.inner = self.inner.set_data(input);
        self
    }
    /// <p>Determines which shard in the stream the data record is assigned to. Partition keys are Unicode strings with a maximum length limit of 256 characters for each key. Amazon Kinesis Data Streams uses the partition key as input to a hash function that maps the partition key and associated data to a specific shard. Specifically, an MD5 hash function is used to map partition keys to 128-bit integer values and to map associated data records to shards. As a result of this hashing mechanism, all data records with the same partition key map to the same shard within the stream.</p>
    pub fn partition_key(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.partition_key(input.into());
        self
    }
    /// <p>Determines which shard in the stream the data record is assigned to. Partition keys are Unicode strings with a maximum length limit of 256 characters for each key. Amazon Kinesis Data Streams uses the partition key as input to a hash function that maps the partition key and associated data to a specific shard. Specifically, an MD5 hash function is used to map partition keys to 128-bit integer values and to map associated data records to shards. As a result of this hashing mechanism, all data records with the same partition key map to the same shard within the stream.</p>
    pub fn set_partition_key(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_partition_key(input);
        self
    }
    /// <p>The hash value used to explicitly determine the shard the data record is assigned to by overriding the partition key hash.</p>
    pub fn explicit_hash_key(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.explicit_hash_key(input.into());
        self
    }
    /// <p>The hash value used to explicitly determine the shard the data record is assigned to by overriding the partition key hash.</p>
    pub fn set_explicit_hash_key(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_explicit_hash_key(input);
        self
    }
    /// <p>Guarantees strictly increasing sequence numbers, for puts from the same client and to the same partition key. Usage: set the <code>SequenceNumberForOrdering</code> of record <i>n</i> to the sequence number of record <i>n-1</i> (as returned in the result when putting record <i>n-1</i>). If this parameter is not set, records are coarsely ordered based on arrival time.</p>
    pub fn sequence_number_for_ordering(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.sequence_number_for_ordering(input.into());
        self
    }
    /// <p>Guarantees strictly increasing sequence numbers, for puts from the same client and to the same partition key. Usage: set the <code>SequenceNumberForOrdering</code> of record <i>n</i> to the sequence number of record <i>n-1</i> (as returned in the result when putting record <i>n-1</i>). If this parameter is not set, records are coarsely ordered based on arrival time.</p>
    pub fn set_sequence_number_for_ordering(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_sequence_number_for_ordering(input);
        self
    }
    /// <p>The ARN of the stream.</p>
    pub fn stream_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.stream_arn(input.into());
        self
    }
    /// <p>The ARN of the stream.</p>
    pub fn set_stream_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_stream_arn(input);
        self
    }
}
