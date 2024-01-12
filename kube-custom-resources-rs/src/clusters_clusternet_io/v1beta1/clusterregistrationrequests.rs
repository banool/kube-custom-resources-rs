// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/clusternet/clusternet/clusters.clusternet.io/v1beta1/clusterregistrationrequests.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// ClusterRegistrationRequestSpec defines the desired state of ClusterRegistrationRequest
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "clusters.clusternet.io", version = "v1beta1", kind = "ClusterRegistrationRequest", plural = "clusterregistrationrequests")]
#[kube(status = "ClusterRegistrationRequestStatus")]
#[kube(schema = "disabled")]
pub struct ClusterRegistrationRequestSpec {
    /// ClusterID, a Random (Version 4) UUID, is a unique value in time and space value representing for child cluster. It is typically generated by the clusternet agent on the successful creation of a "clusternet-agent" Lease in the child cluster. Also it is not allowed to change on PUT operations.
    #[serde(rename = "clusterId")]
    pub cluster_id: String,
    /// ClusterLabels is the labels of the child cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterLabels")]
    pub cluster_labels: Option<BTreeMap<String, String>>,
    /// ClusterName is the cluster name. a lower case alphanumeric characters or '-', and must start and end with an alphanumeric character
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterName")]
    pub cluster_name: Option<String>,
    /// ClusterNamespace is the dedicated namespace of the cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterNamespace")]
    pub cluster_namespace: Option<String>,
    /// ClusterType denotes the type of the child cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterType")]
    pub cluster_type: Option<String>,
    /// SyncMode decides how to sync resources from parent cluster to child cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "syncMode")]
    pub sync_mode: Option<ClusterRegistrationRequestSyncMode>,
}

/// ClusterRegistrationRequestSpec defines the desired state of ClusterRegistrationRequest
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterRegistrationRequestSyncMode {
    Push,
    Pull,
    Dual,
}

/// ClusterRegistrationRequestStatus defines the observed state of ClusterRegistrationRequest
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterRegistrationRequestStatus {
    /// CACertificate is the public certificate that is the root of trust for parent cluster The certificate is encoded in PEM format.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "caCertificate")]
    pub ca_certificate: Option<String>,
    /// DedicatedNamespace is a dedicated namespace for the child cluster, which is created in the parent cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dedicatedNamespace")]
    pub dedicated_namespace: Option<String>,
    /// ErrorMessage tells the reason why the request is not approved successfully.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "errorMessage")]
    pub error_message: Option<String>,
    /// ManagedClusterName is the name of ManagedCluster object in the parent cluster corresponding to the child cluster
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "managedClusterName")]
    pub managed_cluster_name: Option<String>,
    /// Result indicates whether this request has been approved. When all necessary objects have been created and ready for child cluster registration, this field will be set to "Approved". If any illegal updates on this object, "Illegal" will be set to this filed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    /// DedicatedToken is populated by clusternet-hub when Result is RequestApproved. With this token, the client could have full access on the resources created in DedicatedNamespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

