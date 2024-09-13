// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/antrea-io/antrea/multicluster.crd.antrea.io/v1alpha2/clustersets.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// ClusterSetSpec defines the desired state of ClusterSet.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "multicluster.crd.antrea.io", version = "v1alpha2", kind = "ClusterSet", plural = "clustersets")]
#[kube(namespaced)]
#[kube(status = "ClusterSetStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ClusterSetSpec {
    /// ClusterID identifies the local cluster.
    #[serde(rename = "clusterID")]
    pub cluster_id: String,
    /// Leaders include leader clusters known to the member clusters.
    pub leaders: Vec<ClusterSetLeaders>,
    /// The leader cluster Namespace in which the ClusterSet is defined.
    /// Used in a member cluster.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// LeaderClusterInfo specifies information of a leader cluster.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterSetLeaders {
    /// Identify a leader cluster in the ClusterSet.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterID")]
    pub cluster_id: Option<String>,
    /// Name of the Secret resource in the member cluster, which stores
    /// the token to access the leader cluster's API server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// API server endpoint of the leader cluster.
    /// E.g. "https://172.18.0.1:6443", "https://example.com:6443".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
}

/// ClusterSetStatus defines the observed state of ClusterSet.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterSetStatus {
    /// The status of individual member clusters.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterStatuses")]
    pub cluster_statuses: Option<Vec<ClusterSetStatusClusterStatuses>>,
    /// The overall condition of the cluster set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// The generation observed by the controller.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// Total number of clusters ready and connected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "readyClusters")]
    pub ready_clusters: Option<i32>,
    /// Total number of member clusters configured in the ClusterSet.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "totalClusters")]
    pub total_clusters: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterSetStatusClusterStatuses {
    /// ClusterID is the unique identifier of this cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterID")]
    pub cluster_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
}

