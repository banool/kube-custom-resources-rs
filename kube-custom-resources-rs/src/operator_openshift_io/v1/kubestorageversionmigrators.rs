// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/openshift/api/operator.openshift.io/v1/kubestorageversionmigrators.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "operator.openshift.io", version = "v1", kind = "KubeStorageVersionMigrator", plural = "kubestorageversionmigrators")]
#[kube(status = "KubeStorageVersionMigratorStatus")]
#[kube(schema = "disabled")]
pub struct KubeStorageVersionMigratorSpec {
    /// logLevel is an intent based logging for an overall component.  It does not give fine grained control, but it is a simple way to manage coarse grained logging choices that operators have to interpret for their operands. 
    ///  Valid values are: "Normal", "Debug", "Trace", "TraceAll". Defaults to "Normal".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "logLevel")]
    pub log_level: Option<KubeStorageVersionMigratorLogLevel>,
    /// managementState indicates whether and how the operator should manage the component
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "managementState")]
    pub management_state: Option<String>,
    /// observedConfig holds a sparse config that controller has observed from the cluster state.  It exists in spec because it is an input to the level for the operator
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedConfig")]
    pub observed_config: Option<BTreeMap<String, serde_json::Value>>,
    /// operatorLogLevel is an intent based logging for the operator itself.  It does not give fine grained control, but it is a simple way to manage coarse grained logging choices that operators have to interpret for themselves. 
    ///  Valid values are: "Normal", "Debug", "Trace", "TraceAll". Defaults to "Normal".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "operatorLogLevel")]
    pub operator_log_level: Option<KubeStorageVersionMigratorOperatorLogLevel>,
    /// unsupportedConfigOverrides overrides the final configuration that was computed by the operator. Red Hat does not support the use of this field. Misuse of this field could lead to unexpected behavior or conflict with other configuration options. Seek guidance from the Red Hat support before using this field. Use of this property blocks cluster upgrades, it must be removed before upgrading your cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "unsupportedConfigOverrides")]
    pub unsupported_config_overrides: Option<BTreeMap<String, serde_json::Value>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum KubeStorageVersionMigratorLogLevel {
    #[serde(rename = "")]
    KopiumEmpty,
    Normal,
    Debug,
    Trace,
    TraceAll,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum KubeStorageVersionMigratorOperatorLogLevel {
    #[serde(rename = "")]
    KopiumEmpty,
    Normal,
    Debug,
    Trace,
    TraceAll,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubeStorageVersionMigratorStatus {
    /// conditions is a list of conditions and their status
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<KubeStorageVersionMigratorStatusConditions>>,
    /// generations are used to determine when an item needs to be reconciled or has changed in a way that needs a reaction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generations: Option<Vec<KubeStorageVersionMigratorStatusGenerations>>,
    /// observedGeneration is the last generation change you've dealt with
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// readyReplicas indicates how many replicas are ready and at the desired state
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "readyReplicas")]
    pub ready_replicas: Option<i32>,
    /// version is the level this availability applies to
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// OperatorCondition is just the standard condition fields.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubeStorageVersionMigratorStatusConditions {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastTransitionTime")]
    pub last_transition_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// GenerationStatus keeps track of the generation for a given resource so that decisions about forced updates can be made.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubeStorageVersionMigratorStatusGenerations {
    /// group is the group of the thing you're tracking
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// hash is an optional field set for resources without generation that are content sensitive like secrets and configmaps
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    /// lastGeneration is the last generation of the workload controller involved
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastGeneration")]
    pub last_generation: Option<i64>,
    /// name is the name of the thing you're tracking
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// namespace is where the thing you're tracking is
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// resource is the resource type of the thing you're tracking
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}

