// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws-controllers-k8s/dynamodb-controller/dynamodb.services.k8s.aws/v1alpha1/tables.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// TableSpec defines the desired state of Table.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "dynamodb.services.k8s.aws", version = "v1alpha1", kind = "Table", plural = "tables")]
#[kube(namespaced)]
#[kube(status = "TableStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct TableSpec {
    /// An array of attributes that describe the key schema for the table and indexes.
    #[serde(rename = "attributeDefinitions")]
    pub attribute_definitions: Vec<TableAttributeDefinitions>,
    /// Controls how you are charged for read and write throughput and how you manage
    /// capacity. This setting can be changed later.
    /// 
    /// 
    ///    * PROVISIONED - We recommend using PROVISIONED for predictable workloads.
    ///    PROVISIONED sets the billing mode to Provisioned Mode (https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.ReadWriteCapacityMode.html#HowItWorks.ProvisionedThroughput.Manual).
    /// 
    /// 
    ///    * PAY_PER_REQUEST - We recommend using PAY_PER_REQUEST for unpredictable
    ///    workloads. PAY_PER_REQUEST sets the billing mode to On-Demand Mode (https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.ReadWriteCapacityMode.html#HowItWorks.OnDemand).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "billingMode")]
    pub billing_mode: Option<String>,
    /// Represents the settings used to enable point in time recovery.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "continuousBackups")]
    pub continuous_backups: Option<TableContinuousBackups>,
    /// Indicates whether deletion protection is to be enabled (true) or disabled
    /// (false) on the table.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deletionProtectionEnabled")]
    pub deletion_protection_enabled: Option<bool>,
    /// One or more global secondary indexes (the maximum is 20) to be created on
    /// the table. Each global secondary index in the array includes the following:
    /// 
    /// 
    ///    * IndexName - The name of the global secondary index. Must be unique only
    ///    for this table.
    /// 
    /// 
    ///    * KeySchema - Specifies the key schema for the global secondary index.
    /// 
    /// 
    ///    * Projection - Specifies attributes that are copied (projected) from the
    ///    table into the index. These are in addition to the primary key attributes
    ///    and index key attributes, which are automatically projected. Each attribute
    ///    specification is composed of: ProjectionType - One of the following: KEYS_ONLY
    ///    - Only the index and primary keys are projected into the index. INCLUDE
    ///    - Only the specified table attributes are projected into the index. The
    ///    list of projected attributes is in NonKeyAttributes. ALL - All of the
    ///    table attributes are projected into the index. NonKeyAttributes - A list
    ///    of one or more non-key attribute names that are projected into the secondary
    ///    index. The total count of attributes provided in NonKeyAttributes, summed
    ///    across all of the secondary indexes, must not exceed 100. If you project
    ///    the same attribute into two different indexes, this counts as two distinct
    ///    attributes when determining the total.
    /// 
    /// 
    ///    * ProvisionedThroughput - The provisioned throughput settings for the
    ///    global secondary index, consisting of read and write capacity units.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "globalSecondaryIndexes")]
    pub global_secondary_indexes: Option<Vec<TableGlobalSecondaryIndexes>>,
    /// Specifies the attributes that make up the primary key for a table or an index.
    /// The attributes in KeySchema must also be defined in the AttributeDefinitions
    /// array. For more information, see Data Model (https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/DataModel.html)
    /// in the Amazon DynamoDB Developer Guide.
    /// 
    /// 
    /// Each KeySchemaElement in the array is composed of:
    /// 
    /// 
    ///    * AttributeName - The name of this key attribute.
    /// 
    /// 
    ///    * KeyType - The role that the key attribute will assume: HASH - partition
    ///    key RANGE - sort key
    /// 
    /// 
    /// The partition key of an item is also known as its hash attribute. The term
    /// "hash attribute" derives from the DynamoDB usage of an internal hash function
    /// to evenly distribute data items across partitions, based on their partition
    /// key values.
    /// 
    /// 
    /// The sort key of an item is also known as its range attribute. The term "range
    /// attribute" derives from the way DynamoDB stores items with the same partition
    /// key physically close together, in sorted order by the sort key value.
    /// 
    /// 
    /// For a simple primary key (partition key), you must provide exactly one element
    /// with a KeyType of HASH.
    /// 
    /// 
    /// For a composite primary key (partition key and sort key), you must provide
    /// exactly two elements, in this order: The first element must have a KeyType
    /// of HASH, and the second element must have a KeyType of RANGE.
    /// 
    /// 
    /// For more information, see Working with Tables (https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/WorkingWithTables.html#WorkingWithTables.primary.key)
    /// in the Amazon DynamoDB Developer Guide.
    #[serde(rename = "keySchema")]
    pub key_schema: Vec<TableKeySchema>,
    /// One or more local secondary indexes (the maximum is 5) to be created on the
    /// table. Each index is scoped to a given partition key value. There is a 10
    /// GB size limit per partition key value; otherwise, the size of a local secondary
    /// index is unconstrained.
    /// 
    /// 
    /// Each local secondary index in the array includes the following:
    /// 
    /// 
    ///    * IndexName - The name of the local secondary index. Must be unique only
    ///    for this table.
    /// 
    /// 
    ///    * KeySchema - Specifies the key schema for the local secondary index.
    ///    The key schema must begin with the same partition key as the table.
    /// 
    /// 
    ///    * Projection - Specifies attributes that are copied (projected) from the
    ///    table into the index. These are in addition to the primary key attributes
    ///    and index key attributes, which are automatically projected. Each attribute
    ///    specification is composed of: ProjectionType - One of the following: KEYS_ONLY
    ///    - Only the index and primary keys are projected into the index. INCLUDE
    ///    - Only the specified table attributes are projected into the index. The
    ///    list of projected attributes is in NonKeyAttributes. ALL - All of the
    ///    table attributes are projected into the index. NonKeyAttributes - A list
    ///    of one or more non-key attribute names that are projected into the secondary
    ///    index. The total count of attributes provided in NonKeyAttributes, summed
    ///    across all of the secondary indexes, must not exceed 100. If you project
    ///    the same attribute into two different indexes, this counts as two distinct
    ///    attributes when determining the total.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "localSecondaryIndexes")]
    pub local_secondary_indexes: Option<Vec<TableLocalSecondaryIndexes>>,
    /// Represents the provisioned throughput settings for a specified table or index.
    /// The settings can be modified using the UpdateTable operation.
    /// 
    /// 
    /// If you set BillingMode as PROVISIONED, you must specify this property. If
    /// you set BillingMode as PAY_PER_REQUEST, you cannot specify this property.
    /// 
    /// 
    /// For current minimum and maximum provisioned throughput values, see Service,
    /// Account, and Table Quotas (https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Limits.html)
    /// in the Amazon DynamoDB Developer Guide.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "provisionedThroughput")]
    pub provisioned_throughput: Option<TableProvisionedThroughput>,
    /// Represents the settings used to enable server-side encryption.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sseSpecification")]
    pub sse_specification: Option<TableSseSpecification>,
    /// The settings for DynamoDB Streams on the table. These settings consist of:
    /// 
    /// 
    ///    * StreamEnabled - Indicates whether DynamoDB Streams is to be enabled
    ///    (true) or disabled (false).
    /// 
    /// 
    ///    * StreamViewType - When an item in the table is modified, StreamViewType
    ///    determines what information is written to the table's stream. Valid values
    ///    for StreamViewType are: KEYS_ONLY - Only the key attributes of the modified
    ///    item are written to the stream. NEW_IMAGE - The entire item, as it appears
    ///    after it was modified, is written to the stream. OLD_IMAGE - The entire
    ///    item, as it appeared before it was modified, is written to the stream.
    ///    NEW_AND_OLD_IMAGES - Both the new and the old item images of the item
    ///    are written to the stream.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "streamSpecification")]
    pub stream_specification: Option<TableStreamSpecification>,
    /// The table class of the new table. Valid values are STANDARD and STANDARD_INFREQUENT_ACCESS.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tableClass")]
    pub table_class: Option<String>,
    /// The name of the table to create.
    #[serde(rename = "tableName")]
    pub table_name: String,
    /// A list of key-value pairs to label the table. For more information, see Tagging
    /// for DynamoDB (https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Tagging.html).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TableTags>>,
    /// Represents the settings used to enable or disable Time to Live for the specified
    /// table.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeToLive")]
    pub time_to_live: Option<TableTimeToLive>,
}

/// Represents an attribute for describing the key schema for the table and indexes.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableAttributeDefinitions {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "attributeName")]
    pub attribute_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "attributeType")]
    pub attribute_type: Option<String>,
}

/// Represents the settings used to enable point in time recovery.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableContinuousBackups {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pointInTimeRecoveryEnabled")]
    pub point_in_time_recovery_enabled: Option<bool>,
}

/// Represents the properties of a global secondary index.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableGlobalSecondaryIndexes {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "indexName")]
    pub index_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keySchema")]
    pub key_schema: Option<Vec<TableGlobalSecondaryIndexesKeySchema>>,
    /// Represents attributes that are copied (projected) from the table into an
    /// index. These are in addition to the primary key attributes and index key
    /// attributes, which are automatically projected.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub projection: Option<TableGlobalSecondaryIndexesProjection>,
    /// Represents the provisioned throughput settings for a specified table or index.
    /// The settings can be modified using the UpdateTable operation.
    /// 
    /// 
    /// For current minimum and maximum provisioned throughput values, see Service,
    /// Account, and Table Quotas (https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Limits.html)
    /// in the Amazon DynamoDB Developer Guide.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "provisionedThroughput")]
    pub provisioned_throughput: Option<TableGlobalSecondaryIndexesProvisionedThroughput>,
}

/// Represents a single element of a key schema. A key schema specifies the attributes
/// that make up the primary key of a table, or the key attributes of an index.
/// 
/// 
/// A KeySchemaElement represents exactly one attribute of the primary key. For
/// example, a simple primary key would be represented by one KeySchemaElement
/// (for the partition key). A composite primary key would require one KeySchemaElement
/// for the partition key, and another KeySchemaElement for the sort key.
/// 
/// 
/// A KeySchemaElement must be a scalar, top-level attribute (not a nested attribute).
/// The data type must be one of String, Number, or Binary. The attribute cannot
/// be nested within a List or a Map.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableGlobalSecondaryIndexesKeySchema {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "attributeName")]
    pub attribute_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keyType")]
    pub key_type: Option<String>,
}

/// Represents attributes that are copied (projected) from the table into an
/// index. These are in addition to the primary key attributes and index key
/// attributes, which are automatically projected.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableGlobalSecondaryIndexesProjection {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nonKeyAttributes")]
    pub non_key_attributes: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "projectionType")]
    pub projection_type: Option<String>,
}

/// Represents the provisioned throughput settings for a specified table or index.
/// The settings can be modified using the UpdateTable operation.
/// 
/// 
/// For current minimum and maximum provisioned throughput values, see Service,
/// Account, and Table Quotas (https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Limits.html)
/// in the Amazon DynamoDB Developer Guide.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableGlobalSecondaryIndexesProvisionedThroughput {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "readCapacityUnits")]
    pub read_capacity_units: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "writeCapacityUnits")]
    pub write_capacity_units: Option<i64>,
}

/// Represents a single element of a key schema. A key schema specifies the attributes
/// that make up the primary key of a table, or the key attributes of an index.
/// 
/// 
/// A KeySchemaElement represents exactly one attribute of the primary key. For
/// example, a simple primary key would be represented by one KeySchemaElement
/// (for the partition key). A composite primary key would require one KeySchemaElement
/// for the partition key, and another KeySchemaElement for the sort key.
/// 
/// 
/// A KeySchemaElement must be a scalar, top-level attribute (not a nested attribute).
/// The data type must be one of String, Number, or Binary. The attribute cannot
/// be nested within a List or a Map.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableKeySchema {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "attributeName")]
    pub attribute_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keyType")]
    pub key_type: Option<String>,
}

/// Represents the properties of a local secondary index.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableLocalSecondaryIndexes {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "indexName")]
    pub index_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keySchema")]
    pub key_schema: Option<Vec<TableLocalSecondaryIndexesKeySchema>>,
    /// Represents attributes that are copied (projected) from the table into an
    /// index. These are in addition to the primary key attributes and index key
    /// attributes, which are automatically projected.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub projection: Option<TableLocalSecondaryIndexesProjection>,
}

/// Represents a single element of a key schema. A key schema specifies the attributes
/// that make up the primary key of a table, or the key attributes of an index.
/// 
/// 
/// A KeySchemaElement represents exactly one attribute of the primary key. For
/// example, a simple primary key would be represented by one KeySchemaElement
/// (for the partition key). A composite primary key would require one KeySchemaElement
/// for the partition key, and another KeySchemaElement for the sort key.
/// 
/// 
/// A KeySchemaElement must be a scalar, top-level attribute (not a nested attribute).
/// The data type must be one of String, Number, or Binary. The attribute cannot
/// be nested within a List or a Map.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableLocalSecondaryIndexesKeySchema {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "attributeName")]
    pub attribute_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keyType")]
    pub key_type: Option<String>,
}

/// Represents attributes that are copied (projected) from the table into an
/// index. These are in addition to the primary key attributes and index key
/// attributes, which are automatically projected.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableLocalSecondaryIndexesProjection {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nonKeyAttributes")]
    pub non_key_attributes: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "projectionType")]
    pub projection_type: Option<String>,
}

/// Represents the provisioned throughput settings for a specified table or index.
/// The settings can be modified using the UpdateTable operation.
/// 
/// 
/// If you set BillingMode as PROVISIONED, you must specify this property. If
/// you set BillingMode as PAY_PER_REQUEST, you cannot specify this property.
/// 
/// 
/// For current minimum and maximum provisioned throughput values, see Service,
/// Account, and Table Quotas (https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Limits.html)
/// in the Amazon DynamoDB Developer Guide.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableProvisionedThroughput {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "readCapacityUnits")]
    pub read_capacity_units: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "writeCapacityUnits")]
    pub write_capacity_units: Option<i64>,
}

/// Represents the settings used to enable server-side encryption.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableSseSpecification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kmsMasterKeyID")]
    pub kms_master_key_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sseType")]
    pub sse_type: Option<String>,
}

/// The settings for DynamoDB Streams on the table. These settings consist of:
/// 
/// 
///    * StreamEnabled - Indicates whether DynamoDB Streams is to be enabled
///    (true) or disabled (false).
/// 
/// 
///    * StreamViewType - When an item in the table is modified, StreamViewType
///    determines what information is written to the table's stream. Valid values
///    for StreamViewType are: KEYS_ONLY - Only the key attributes of the modified
///    item are written to the stream. NEW_IMAGE - The entire item, as it appears
///    after it was modified, is written to the stream. OLD_IMAGE - The entire
///    item, as it appeared before it was modified, is written to the stream.
///    NEW_AND_OLD_IMAGES - Both the new and the old item images of the item
///    are written to the stream.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableStreamSpecification {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "streamEnabled")]
    pub stream_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "streamViewType")]
    pub stream_view_type: Option<String>,
}

/// Describes a tag. A tag is a key-value pair. You can add up to 50 tags to
/// a single DynamoDB table.
/// 
/// 
/// Amazon Web Services-assigned tag names and values are automatically assigned
/// the aws: prefix, which the user cannot assign. Amazon Web Services-assigned
/// tag names do not count towards the tag limit of 50. User-assigned tag names
/// have the prefix user: in the Cost Allocation Report. You cannot backdate
/// the application of a tag.
/// 
/// 
/// For an overview on tagging DynamoDB resources, see Tagging for DynamoDB (https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Tagging.html)
/// in the Amazon DynamoDB Developer Guide.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Represents the settings used to enable or disable Time to Live for the specified
/// table.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableTimeToLive {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "attributeName")]
    pub attribute_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// TableStatus defines the observed state of Table
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<TableStatusAckResourceMetadata>,
    /// Contains information about the table archive.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "archivalSummary")]
    pub archival_summary: Option<TableStatusArchivalSummary>,
    /// All CRS managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// The date and time when the table was created, in UNIX epoch time (http://www.epochconverter.com/)
    /// format.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "creationDateTime")]
    pub creation_date_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "globalSecondaryIndexesDescriptions")]
    pub global_secondary_indexes_descriptions: Option<Vec<TableStatusGlobalSecondaryIndexesDescriptions>>,
    /// Represents the version of global tables (https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/GlobalTables.html)
    /// in use, if the table is replicated across Amazon Web Services Regions.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "globalTableVersion")]
    pub global_table_version: Option<String>,
    /// The number of items in the specified table. DynamoDB updates this value approximately
    /// every six hours. Recent changes might not be reflected in this value.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "itemCount")]
    pub item_count: Option<i64>,
    /// The Amazon Resource Name (ARN) that uniquely identifies the latest stream
    /// for this table.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "latestStreamARN")]
    pub latest_stream_arn: Option<String>,
    /// A timestamp, in ISO 8601 format, for this stream.
    /// 
    /// 
    /// Note that LatestStreamLabel is not a unique identifier for the stream, because
    /// it is possible that a stream from another table might have the same timestamp.
    /// However, the combination of the following three elements is guaranteed to
    /// be unique:
    /// 
    /// 
    ///    * Amazon Web Services customer ID
    /// 
    /// 
    ///    * Table name
    /// 
    /// 
    ///    * StreamLabel
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "latestStreamLabel")]
    pub latest_stream_label: Option<String>,
    /// Represents replicas of the table.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<Vec<TableStatusReplicas>>,
    /// Contains details for the restore.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "restoreSummary")]
    pub restore_summary: Option<TableStatusRestoreSummary>,
    /// Unique identifier for the table for which the backup was created.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tableID")]
    pub table_id: Option<String>,
    /// The total size of the specified table, in bytes. DynamoDB updates this value
    /// approximately every six hours. Recent changes might not be reflected in this
    /// value.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tableSizeBytes")]
    pub table_size_bytes: Option<i64>,
    /// The current state of the table:
    /// 
    /// 
    ///    * CREATING - The table is being created.
    /// 
    /// 
    ///    * UPDATING - The table/index configuration is being updated. The table/index
    ///    remains available for data operations when UPDATING.
    /// 
    /// 
    ///    * DELETING - The table is being deleted.
    /// 
    /// 
    ///    * ACTIVE - The table is ready for use.
    /// 
    /// 
    ///    * INACCESSIBLE_ENCRYPTION_CREDENTIALS - The KMS key used to encrypt the
    ///    table in inaccessible. Table operations may fail due to failure to use
    ///    the KMS key. DynamoDB will initiate the table archival process when a
    ///    table's KMS key remains inaccessible for more than seven days.
    /// 
    /// 
    ///    * ARCHIVING - The table is being archived. Operations are not allowed
    ///    until archival is complete.
    /// 
    /// 
    ///    * ARCHIVED - The table has been archived. See the ArchivalReason for more
    ///    information.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tableStatus")]
    pub table_status: Option<String>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableStatusAckResourceMetadata {
    /// ARN is the Amazon Resource Name for the resource. This is a
    /// globally-unique identifier and is set only by the ACK service controller
    /// once the controller has orchestrated the creation of the resource OR
    /// when it has verified that an "adopted" resource (a resource where the
    /// ARN annotation was set by the Kubernetes user on the CR) exists and
    /// matches the supplied CR's Spec field values.
    /// TODO(vijat@): Find a better strategy for resources that do not have ARN in CreateOutputResponse
    /// https://github.com/aws/aws-controllers-k8s/issues/270
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// OwnerAccountID is the AWS Account ID of the account that owns the
    /// backend AWS service API resource.
    #[serde(rename = "ownerAccountID")]
    pub owner_account_id: String,
    /// Region is the AWS region in which the resource exists or will exist.
    pub region: String,
}

/// Contains information about the table archive.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableStatusArchivalSummary {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "archivalBackupARN")]
    pub archival_backup_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "archivalDateTime")]
    pub archival_date_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "archivalReason")]
    pub archival_reason: Option<String>,
}

/// Represents the properties of a global secondary index.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableStatusGlobalSecondaryIndexesDescriptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backfilling: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "indexARN")]
    pub index_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "indexName")]
    pub index_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "indexSizeBytes")]
    pub index_size_bytes: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "indexStatus")]
    pub index_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "itemCount")]
    pub item_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keySchema")]
    pub key_schema: Option<Vec<TableStatusGlobalSecondaryIndexesDescriptionsKeySchema>>,
    /// Represents attributes that are copied (projected) from the table into an
    /// index. These are in addition to the primary key attributes and index key
    /// attributes, which are automatically projected.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub projection: Option<TableStatusGlobalSecondaryIndexesDescriptionsProjection>,
    /// Represents the provisioned throughput settings for the table, consisting
    /// of read and write capacity units, along with data about increases and decreases.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "provisionedThroughput")]
    pub provisioned_throughput: Option<TableStatusGlobalSecondaryIndexesDescriptionsProvisionedThroughput>,
}

/// Represents a single element of a key schema. A key schema specifies the attributes
/// that make up the primary key of a table, or the key attributes of an index.
/// 
/// 
/// A KeySchemaElement represents exactly one attribute of the primary key. For
/// example, a simple primary key would be represented by one KeySchemaElement
/// (for the partition key). A composite primary key would require one KeySchemaElement
/// for the partition key, and another KeySchemaElement for the sort key.
/// 
/// 
/// A KeySchemaElement must be a scalar, top-level attribute (not a nested attribute).
/// The data type must be one of String, Number, or Binary. The attribute cannot
/// be nested within a List or a Map.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableStatusGlobalSecondaryIndexesDescriptionsKeySchema {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "attributeName")]
    pub attribute_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keyType")]
    pub key_type: Option<String>,
}

/// Represents attributes that are copied (projected) from the table into an
/// index. These are in addition to the primary key attributes and index key
/// attributes, which are automatically projected.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableStatusGlobalSecondaryIndexesDescriptionsProjection {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nonKeyAttributes")]
    pub non_key_attributes: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "projectionType")]
    pub projection_type: Option<String>,
}

/// Represents the provisioned throughput settings for the table, consisting
/// of read and write capacity units, along with data about increases and decreases.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableStatusGlobalSecondaryIndexesDescriptionsProvisionedThroughput {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastDecreaseDateTime")]
    pub last_decrease_date_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastIncreaseDateTime")]
    pub last_increase_date_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "numberOfDecreasesToday")]
    pub number_of_decreases_today: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "readCapacityUnits")]
    pub read_capacity_units: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "writeCapacityUnits")]
    pub write_capacity_units: Option<i64>,
}

/// Contains the details of the replica.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableStatusReplicas {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "globalSecondaryIndexes")]
    pub global_secondary_indexes: Option<Vec<TableStatusReplicasGlobalSecondaryIndexes>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kmsMasterKeyID")]
    pub kms_master_key_id: Option<String>,
    /// Replica-specific provisioned throughput settings. If not specified, uses
    /// the source table's provisioned throughput settings.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "provisionedThroughputOverride")]
    pub provisioned_throughput_override: Option<TableStatusReplicasProvisionedThroughputOverride>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "regionName")]
    pub region_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replicaInaccessibleDateTime")]
    pub replica_inaccessible_date_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replicaStatus")]
    pub replica_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replicaStatusDescription")]
    pub replica_status_description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replicaStatusPercentProgress")]
    pub replica_status_percent_progress: Option<String>,
    /// Contains details of the table class.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replicaTableClassSummary")]
    pub replica_table_class_summary: Option<TableStatusReplicasReplicaTableClassSummary>,
}

/// Represents the properties of a replica global secondary index.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableStatusReplicasGlobalSecondaryIndexes {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "indexName")]
    pub index_name: Option<String>,
    /// Replica-specific provisioned throughput settings. If not specified, uses
    /// the source table's provisioned throughput settings.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "provisionedThroughputOverride")]
    pub provisioned_throughput_override: Option<TableStatusReplicasGlobalSecondaryIndexesProvisionedThroughputOverride>,
}

/// Replica-specific provisioned throughput settings. If not specified, uses
/// the source table's provisioned throughput settings.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableStatusReplicasGlobalSecondaryIndexesProvisionedThroughputOverride {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "readCapacityUnits")]
    pub read_capacity_units: Option<i64>,
}

/// Replica-specific provisioned throughput settings. If not specified, uses
/// the source table's provisioned throughput settings.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableStatusReplicasProvisionedThroughputOverride {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "readCapacityUnits")]
    pub read_capacity_units: Option<i64>,
}

/// Contains details of the table class.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableStatusReplicasReplicaTableClassSummary {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastUpdateDateTime")]
    pub last_update_date_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tableClass")]
    pub table_class: Option<String>,
}

/// Contains details for the restore.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TableStatusRestoreSummary {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "restoreDateTime")]
    pub restore_date_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "restoreInProgress")]
    pub restore_in_progress: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceBackupARN")]
    pub source_backup_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceTableARN")]
    pub source_table_arn: Option<String>,
}

