// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/chaos-mesh/chaos-mesh/chaos-mesh.org/v1alpha1/remoteclusters.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// RemoteClusterSpec defines the specification of a remote cluster
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "chaos-mesh.org", version = "v1alpha1", kind = "RemoteCluster", plural = "remoteclusters")]
#[kube(status = "RemoteClusterStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct RemoteClusterSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configOverride")]
    pub config_override: Option<BTreeMap<String, serde_json::Value>>,
    /// RemoteClusterKubeConfig refers to a secret by which we'll use to connect remote cluster
    #[serde(rename = "kubeConfig")]
    pub kube_config: RemoteClusterKubeConfig,
    pub namespace: String,
    pub version: String,
}

/// RemoteClusterKubeConfig refers to a secret by which we'll use to connect remote cluster
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RemoteClusterKubeConfig {
    /// RemoteClusterSecretRef refers to a secret in any namespaces
    #[serde(rename = "secretRef")]
    pub secret_ref: RemoteClusterKubeConfigSecretRef,
}

/// RemoteClusterSecretRef refers to a secret in any namespaces
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RemoteClusterKubeConfigSecretRef {
    pub key: String,
    pub name: String,
    pub namespace: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RemoteClusterStatus {
    /// Conditions represents the current condition of the remote cluster
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<RemoteClusterStatusConditions>>,
    #[serde(rename = "currentVersion")]
    pub current_version: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RemoteClusterStatusConditions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub status: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

