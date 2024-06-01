// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws-controllers-k8s/rds-controller/rds.services.k8s.aws/v1alpha1/globalclusters.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// GlobalClusterSpec defines the desired state of GlobalCluster.
/// 
/// 
/// A data type representing an Aurora global database.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "rds.services.k8s.aws", version = "v1alpha1", kind = "GlobalCluster", plural = "globalclusters")]
#[kube(namespaced)]
#[kube(status = "GlobalClusterStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct GlobalClusterSpec {
    /// The name for your database of up to 64 alphanumeric characters. If you do
    /// not provide a name, Amazon Aurora will not create a database in the global
    /// database cluster you are creating.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "databaseName")]
    pub database_name: Option<String>,
    /// The deletion protection setting for the new global database. The global database
    /// can't be deleted when deletion protection is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deletionProtection")]
    pub deletion_protection: Option<bool>,
    /// The name of the database engine to be used for this DB cluster.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    /// The engine version of the Aurora global database.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "engineVersion")]
    pub engine_version: Option<String>,
    /// The cluster identifier of the new global database cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "globalClusterIdentifier")]
    pub global_cluster_identifier: Option<String>,
    /// The Amazon Resource Name (ARN) to use as the primary cluster of the global
    /// database. This parameter is optional.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceDBClusterIdentifier")]
    pub source_db_cluster_identifier: Option<String>,
    /// The storage encryption setting for the new global database cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageEncrypted")]
    pub storage_encrypted: Option<bool>,
}

/// GlobalClusterStatus defines the observed state of GlobalCluster
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GlobalClusterStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<GlobalClusterStatusAckResourceMetadata>,
    /// All CRS managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// A data object containing all properties for the current state of an in-process
    /// or pending failover process for this Aurora global database. This object
    /// is empty unless the FailoverGlobalCluster API operation has been called on
    /// this Aurora global database (GlobalCluster).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failoverState")]
    pub failover_state: Option<GlobalClusterStatusFailoverState>,
    /// The list of primary and secondary clusters within the global database cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "globalClusterMembers")]
    pub global_cluster_members: Option<Vec<GlobalClusterStatusGlobalClusterMembers>>,
    /// The Amazon Web Services Region-unique, immutable identifier for the global
    /// database cluster. This identifier is found in Amazon Web Services CloudTrail
    /// log entries whenever the Amazon Web Services KMS key for the DB cluster is
    /// accessed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "globalClusterResourceID")]
    pub global_cluster_resource_id: Option<String>,
    /// Specifies the current state of this global database cluster.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GlobalClusterStatusAckResourceMetadata {
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

/// A data object containing all properties for the current state of an in-process
/// or pending failover process for this Aurora global database. This object
/// is empty unless the FailoverGlobalCluster API operation has been called on
/// this Aurora global database (GlobalCluster).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GlobalClusterStatusFailoverState {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fromDBClusterARN")]
    pub from_db_cluster_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "toDBClusterARN")]
    pub to_db_cluster_arn: Option<String>,
}

/// A data structure with information about any primary and secondary clusters
/// associated with an Aurora global database.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GlobalClusterStatusGlobalClusterMembers {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dbClusterARN")]
    pub db_cluster_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "globalWriteForwardingStatus")]
    pub global_write_forwarding_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isWriter")]
    pub is_writer: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readers: Option<Vec<String>>,
}

