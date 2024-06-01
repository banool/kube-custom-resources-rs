// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/redhat-cop/keepalived-operator/redhatcop.redhat.io/v1alpha1/keepalivedgroups.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// KeepalivedGroupSpec defines the desired state of KeepalivedGroup
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "redhatcop.redhat.io", version = "v1alpha1", kind = "KeepalivedGroup", plural = "keepalivedgroups")]
#[kube(namespaced)]
#[kube(status = "KeepalivedGroupStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct KeepalivedGroupSpec {
    /// // +kubebuilder:validation:UniqueItems=true
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "blacklistRouterIDs")]
    pub blacklist_router_i_ds: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "daemonsetPodAnnotations")]
    pub daemonset_pod_annotations: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "daemonsetPodPriorityClassName")]
    pub daemonset_pod_priority_class_name: Option<String>,
    /// //+kubebuilder:validation:Optional
    pub image: String,
    pub interface: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "interfaceFromIP")]
    pub interface_from_ip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeSelector")]
    pub node_selector: Option<BTreeMap<String, String>>,
    /// PasswordAuth references a Kubernetes secret to extract the password for VRRP authentication
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "passwordAuth")]
    pub password_auth: Option<KeepalivedGroupPasswordAuth>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "unicastEnabled")]
    pub unicast_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "verbatimConfig")]
    pub verbatim_config: Option<BTreeMap<String, String>>,
}

/// PasswordAuth references a Kubernetes secret to extract the password for VRRP authentication
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeepalivedGroupPasswordAuth {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretKey")]
    pub secret_key: Option<String>,
    /// LocalObjectReference contains enough information to let you locate the referenced object inside the same namespace.
    #[serde(rename = "secretRef")]
    pub secret_ref: KeepalivedGroupPasswordAuthSecretRef,
}

/// LocalObjectReference contains enough information to let you locate the referenced object inside the same namespace.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeepalivedGroupPasswordAuthSecretRef {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// KeepalivedGroupStatus defines the observed state of KeepalivedGroup
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeepalivedGroupStatus {
    /// INSERT ADDITIONAL STATUS FIELD - define observed state of cluster
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "routerIDs")]
    pub router_i_ds: Option<BTreeMap<String, i64>>,
}

