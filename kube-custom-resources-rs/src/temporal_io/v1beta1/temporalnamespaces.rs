// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/alexandrevilain/temporal-operator/temporal.io/v1beta1/temporalnamespaces.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// TemporalNamespaceSpec defines the desired state of Namespace.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "temporal.io", version = "v1beta1", kind = "TemporalNamespace", plural = "temporalnamespaces")]
#[kube(namespaced)]
#[kube(status = "TemporalNamespaceStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct TemporalNamespaceSpec {
    /// The name of active Temporal Cluster. Only applicable if the namespace is a global namespace.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "activeClusterName")]
    pub active_cluster_name: Option<String>,
    /// AllowDeletion makes the controller delete the Temporal namespace if the CRD is deleted.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowDeletion")]
    pub allow_deletion: Option<bool>,
    /// Archival is a per-namespace archival configuration. If not set, the default cluster configuration is used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archival: Option<TemporalNamespaceArchival>,
    /// Reference to the temporal cluster the namespace will be created.
    #[serde(rename = "clusterRef")]
    pub cluster_ref: TemporalNamespaceClusterRef,
    /// List of clusters names to which the namespace can fail over. Only applicable if the namespace is a global namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<String>>,
    /// Data is a key-value map for any customized purpose.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<BTreeMap<String, String>>,
    /// Namespace description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// IsGlobalNamespace defines whether the namespace is a global namespace.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isGlobalNamespace")]
    pub is_global_namespace: Option<bool>,
    /// Namespace owner email.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownerEmail")]
    pub owner_email: Option<String>,
    /// RetentionPeriod to apply on closed workflow executions.
    #[serde(rename = "retentionPeriod")]
    pub retention_period: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "securityToken")]
    pub security_token: Option<String>,
}

/// Archival is a per-namespace archival configuration. If not set, the default cluster configuration is used.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TemporalNamespaceArchival {
    /// History is the config for this namespace history archival.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub history: Option<TemporalNamespaceArchivalHistory>,
    /// Visibility is the config for this namespace visibility archival.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<TemporalNamespaceArchivalVisibility>,
}

/// History is the config for this namespace history archival.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TemporalNamespaceArchivalHistory {
    /// EnableRead allows temporal to read from the archived Event History.
    #[serde(rename = "enableRead")]
    pub enable_read: bool,
    /// Enabled defines if the archival is enabled by default for all namespaces or for a particular namespace (depends if it's for a TemporalCluster or a TemporalNamespace).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Path is ...
    pub path: String,
    /// Paused defines if the archival is paused.
    pub paused: bool,
}

/// Visibility is the config for this namespace visibility archival.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TemporalNamespaceArchivalVisibility {
    /// EnableRead allows temporal to read from the archived Event History.
    #[serde(rename = "enableRead")]
    pub enable_read: bool,
    /// Enabled defines if the archival is enabled by default for all namespaces or for a particular namespace (depends if it's for a TemporalCluster or a TemporalNamespace).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Path is ...
    pub path: String,
    /// Paused defines if the archival is paused.
    pub paused: bool,
}

/// Reference to the temporal cluster the namespace will be created.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TemporalNamespaceClusterRef {
    /// The name of the TemporalCluster to reference.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The namespace of the TemporalCluster to reference. Defaults to the namespace of the requested resource if omitted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// TemporalNamespaceStatus defines the observed state of Namespace.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TemporalNamespaceStatus {
    /// Conditions represent the latest available observations of the Namespace state.
    pub conditions: Vec<Condition>,
}

