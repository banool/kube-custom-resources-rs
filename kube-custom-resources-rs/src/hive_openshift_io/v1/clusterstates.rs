// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/openshift/hive/hive.openshift.io/v1/clusterstates.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// ClusterStateSpec defines the desired state of ClusterState
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "hive.openshift.io", version = "v1", kind = "ClusterState", plural = "clusterstates")]
#[kube(namespaced)]
#[kube(status = "ClusterStateStatus")]
#[kube(schema = "disabled")]
pub struct ClusterStateSpec {
}

/// ClusterStateStatus defines the observed state of ClusterState
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterStateStatus {
    /// ClusterOperators contains the state for every cluster operator in the target cluster
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterOperators")]
    pub cluster_operators: Option<Vec<ClusterStateStatusClusterOperators>>,
    /// LastUpdated is the last time that operator state was updated
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastUpdated")]
    pub last_updated: Option<String>,
}

/// ClusterOperatorState summarizes the status of a single cluster operator
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterStateStatusClusterOperators {
    /// Conditions is the set of conditions in the status of the cluster operator on the target cluster
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<ClusterStateStatusClusterOperatorsConditions>>,
    /// Name is the name of the cluster operator
    pub name: String,
}

/// ClusterOperatorStatusCondition represents the state of the operator's managed and monitored components.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterStateStatusClusterOperatorsConditions {
    /// lastTransitionTime is the time of the last update to the current status property.
    #[serde(rename = "lastTransitionTime")]
    pub last_transition_time: String,
    /// message provides additional information about the current condition. This is only to be consumed by humans.  It may contain Line Feed characters (U+000A), which should be rendered as new lines.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// reason is the CamelCase reason for the condition's current status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// status of the condition, one of True, False, Unknown.
    pub status: String,
    /// type specifies the aspect reported by this condition.
    #[serde(rename = "type")]
    pub r#type: String,
}

