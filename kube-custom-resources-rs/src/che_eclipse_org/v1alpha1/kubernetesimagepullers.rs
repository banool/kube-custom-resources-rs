// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/che-incubator/kubernetes-image-puller-operator/che.eclipse.org/v1alpha1/kubernetesimagepullers.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// KubernetesImagePullerSpec defines the desired state of KubernetesImagePuller
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "che.eclipse.org", version = "v1alpha1", kind = "KubernetesImagePuller", plural = "kubernetesimagepullers")]
#[kube(namespaced)]
#[kube(status = "KubernetesImagePullerStatus")]
#[kube(schema = "disabled")]
pub struct KubernetesImagePullerSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub affinity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cachingCPULimit")]
    pub caching_cpu_limit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cachingCPURequest")]
    pub caching_cpu_request: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cachingIntervalHours")]
    pub caching_interval_hours: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cachingMemoryLimit")]
    pub caching_memory_limit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cachingMemoryRequest")]
    pub caching_memory_request: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configMapName")]
    pub config_map_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "daemonsetName")]
    pub daemonset_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deploymentName")]
    pub deployment_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imagePullSecrets")]
    pub image_pull_secrets: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imagePullerImage")]
    pub image_puller_image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub images: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeSelector")]
    pub node_selector: Option<String>,
}

/// KubernetesImagePullerStatus defines the observed state of KubernetesImagePuller
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubernetesImagePullerStatus {
    /// KubernetesImagePuller image in use.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imagePullerImage")]
    pub image_puller_image: Option<String>,
}

