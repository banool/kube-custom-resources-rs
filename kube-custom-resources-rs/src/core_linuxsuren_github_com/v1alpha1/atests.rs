// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/LinuxSuRen/api-testing/core.linuxsuren.github.com/v1alpha1/atests.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// ATestSpec defines the desired state of ATest
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "core.linuxsuren.github.com", version = "v1alpha1", kind = "ATest", plural = "atests")]
#[kube(namespaced)]
#[kube(status = "ATestStatus")]
#[kube(schema = "disabled")]
pub struct ATestSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Persistent defines the persistent volume claim
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub persistent: Option<ATestPersistent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    /// Service Type string describes ingress methods for a service
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceType")]
    pub service_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// Persistent defines the persistent volume claim
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ATestPersistent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageClass")]
    pub storage_class: Option<String>,
}

/// ATestStatus defines the observed state of ATest
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ATestStatus {
}

