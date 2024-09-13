// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws-controllers-k8s/keyspaces-controller/keyspaces.services.k8s.aws/v1alpha1/keyspaces.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// KeyspaceSpec defines the desired state of Keyspace.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "keyspaces.services.k8s.aws", version = "v1alpha1", kind = "Keyspace", plural = "keyspaces")]
#[kube(namespaced)]
#[kube(status = "KeyspaceStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct KeyspaceSpec {
    /// The name of the keyspace to be created.
    #[serde(rename = "keyspaceName")]
    pub keyspace_name: String,
    /// The replication specification of the keyspace includes:
    /// 
    /// 
    ///    * replicationStrategy - the required value is SINGLE_REGION or MULTI_REGION.
    /// 
    /// 
    ///    * regionList - if the replicationStrategy is MULTI_REGION, the regionList
    ///    requires the current Region and at least one additional Amazon Web Services
    ///    Region where the keyspace is going to be replicated in. The maximum number
    ///    of supported replication Regions including the current Region is six.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replicationSpecification")]
    pub replication_specification: Option<KeyspaceReplicationSpecification>,
    /// A list of key-value pair tags to be attached to the keyspace.
    /// 
    /// 
    /// For more information, see Adding tags and labels to Amazon Keyspaces resources
    /// (https://docs.aws.amazon.com/keyspaces/latest/devguide/tagging-keyspaces.html)
    /// in the Amazon Keyspaces Developer Guide.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<KeyspaceTags>>,
}

/// The replication specification of the keyspace includes:
/// 
/// 
///    * replicationStrategy - the required value is SINGLE_REGION or MULTI_REGION.
/// 
/// 
///    * regionList - if the replicationStrategy is MULTI_REGION, the regionList
///    requires the current Region and at least one additional Amazon Web Services
///    Region where the keyspace is going to be replicated in. The maximum number
///    of supported replication Regions including the current Region is six.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeyspaceReplicationSpecification {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "regionList")]
    pub region_list: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replicationStrategy")]
    pub replication_strategy: Option<String>,
}

/// Describes a tag. A tag is a key-value pair. You can add up to 50 tags to
/// a single Amazon Keyspaces resource.
/// 
/// 
/// Amazon Web Services-assigned tag names and values are automatically assigned
/// the aws: prefix, which the user cannot assign. Amazon Web Services-assigned
/// tag names do not count towards the tag limit of 50. User-assigned tag names
/// have the prefix user: in the Cost Allocation Report. You cannot backdate
/// the application of a tag.
/// 
/// 
/// For more information, see Adding tags and labels to Amazon Keyspaces resources
/// (https://docs.aws.amazon.com/keyspaces/latest/devguide/tagging-keyspaces.html)
/// in the Amazon Keyspaces Developer Guide.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeyspaceTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// KeyspaceStatus defines the observed state of Keyspace
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeyspaceStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<KeyspaceStatusAckResourceMetadata>,
    /// All CRS managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// The unique identifier of the keyspace in the format of an Amazon Resource
    /// Name (ARN).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceARN")]
    pub resource_arn: Option<String>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeyspaceStatusAckResourceMetadata {
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

