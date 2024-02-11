// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubernetes-sigs/secrets-store-csi-driver/secrets-store.csi.x-k8s.io/v1/secretproviderclasspodstatuses.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.5


use serde::{Serialize, Deserialize};

/// SecretProviderClassPodStatusStatus defines the observed state of SecretProviderClassPodStatus
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecretProviderClassPodStatusStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mounted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub objects: Option<Vec<SecretProviderClassPodStatusStatusObjects>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podName")]
    pub pod_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretProviderClassName")]
    pub secret_provider_class_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetPath")]
    pub target_path: Option<String>,
}

/// SecretProviderClassObject defines the object fetched from external secrets store
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecretProviderClassPodStatusStatusObjects {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

