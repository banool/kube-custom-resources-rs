// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/openshift/hive/hive.openshift.io/v1/clusterrelocates.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// ClusterRelocateSpec defines the relocation of clusters from one Hive instance to another.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "hive.openshift.io", version = "v1", kind = "ClusterRelocate", plural = "clusterrelocates")]
#[kube(namespaced)]
#[kube(status = "ClusterRelocateStatus")]
#[kube(schema = "disabled")]
pub struct ClusterRelocateSpec {
    /// ClusterDeploymentSelector is a LabelSelector indicating which clusters will be relocated.
    #[serde(rename = "clusterDeploymentSelector")]
    pub cluster_deployment_selector: ClusterRelocateClusterDeploymentSelector,
    /// KubeconfigSecretRef is a reference to the secret containing the kubeconfig for the destination Hive instance. The kubeconfig must be in a data field where the key is "kubeconfig".
    #[serde(rename = "kubeconfigSecretRef")]
    pub kubeconfig_secret_ref: ClusterRelocateKubeconfigSecretRef,
}

/// ClusterDeploymentSelector is a LabelSelector indicating which clusters will be relocated.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterRelocateClusterDeploymentSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<ClusterRelocateClusterDeploymentSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterRelocateClusterDeploymentSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// KubeconfigSecretRef is a reference to the secret containing the kubeconfig for the destination Hive instance. The kubeconfig must be in a data field where the key is "kubeconfig".
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterRelocateKubeconfigSecretRef {
    /// Name is the name of the secret.
    pub name: String,
    /// Namespace is the namespace where the secret lives.
    pub namespace: String,
}

/// ClusterRelocateStatus defines the observed state of ClusterRelocate.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterRelocateStatus {
}

