// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/openshift/api/config.openshift.io/v1/clusteroperators.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// spec holds configuration that could apply to any operator.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "config.openshift.io", version = "v1", kind = "ClusterOperator", plural = "clusteroperators")]
#[kube(status = "ClusterOperatorStatus")]
#[kube(schema = "disabled")]
pub struct ClusterOperatorSpec {
}

/// status holds the information about the state of an operator.  It is consistent with status information across the Kubernetes ecosystem.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterOperatorStatus {
    /// conditions describes the state of the operator's managed and monitored components.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<ClusterOperatorStatusConditions>>,
    /// extension contains any additional status information specific to the operator which owns this status object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extension: Option<BTreeMap<String, serde_json::Value>>,
    /// relatedObjects is a list of objects that are "interesting" or related to this operator.  Common uses are: 1. the detailed resource driving the operator 2. operator namespaces 3. operand namespaces
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "relatedObjects")]
    pub related_objects: Option<Vec<ClusterOperatorStatusRelatedObjects>>,
    /// versions is a slice of operator and operand version tuples.  Operators which manage multiple operands will have multiple operand entries in the array.  Available operators must report the version of the operator itself with the name "operator". An operator reports a new "operator" version when it has rolled out the new version to all of its operands.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<ClusterOperatorStatusVersions>>,
}

/// ClusterOperatorStatusCondition represents the state of the operator's managed and monitored components.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterOperatorStatusConditions {
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

/// ObjectReference contains enough information to let you inspect or modify the referred object.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterOperatorStatusRelatedObjects {
    /// group of the referent.
    pub group: String,
    /// name of the referent.
    pub name: String,
    /// namespace of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// resource of the referent.
    pub resource: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterOperatorStatusVersions {
    /// name is the name of the particular operand this version is for.  It usually matches container images, not operators.
    pub name: String,
    /// version indicates which version of a particular operand is currently being managed.  It must always match the Available operand.  If 1.0.0 is Available, then this must indicate 1.0.0 even if the operator is trying to rollout 1.1.0
    pub version: String,
}

