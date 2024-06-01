// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/devfile/registry-operator/registry.devfile.io/v1alpha1/devfileregistries.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// DevfileRegistrySpec defines the desired state of DevfileRegistry
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "registry.devfile.io", version = "v1alpha1", kind = "DevfileRegistry", plural = "devfileregistries")]
#[kube(namespaced)]
#[kube(status = "DevfileRegistryStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct DevfileRegistrySpec {
    /// Sets the devfile index container spec to be deployed on the Devfile Registry
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "devfileIndex")]
    pub devfile_index: Option<DevfileRegistryDevfileIndex>,
    /// Sets the container image containing devfile stacks to be deployed on the Devfile Registry
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "devfileIndexImage")]
    pub devfile_index_image: Option<String>,
    /// Overrides the fully qualified app name of the devfile registry
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fullnameOverride")]
    pub fullname_override: Option<String>,
    /// Sets the registry server deployment to run under headless mode
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headless: Option<bool>,
    /// Overrides the entire hostname and domain of the devfile registry ingress
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hostnameOverride")]
    pub hostname_override: Option<String>,
    /// DevfileRegistrySpecK8sOnly defines the desired state of the kubernetes-only fields of the DevfileRegistry
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub k8s: Option<DevfileRegistryK8s>,
    /// Overrides the app name of the devfile registry
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nameOverride")]
    pub name_override: Option<String>,
    /// Sets the OCI registry container spec to be deployed on the Devfile Registry
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ociRegistry")]
    pub oci_registry: Option<DevfileRegistryOciRegistry>,
    /// Overrides the container image used for the OCI registry. Recommended to leave blank and default to the image specified by the operator.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ociRegistryImage")]
    pub oci_registry_image: Option<String>,
    /// Sets the registry viewer container spec to be deployed on the Devfile Registry
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "registryViewer")]
    pub registry_viewer: Option<DevfileRegistryRegistryViewer>,
    /// Overrides the container image used for the registry viewer.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "registryViewerImage")]
    pub registry_viewer_image: Option<String>,
    /// DevfileRegistrySpecStorage defines the desired state of the storage for the DevfileRegistry
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage: Option<DevfileRegistryStorage>,
    /// Telemetry defines the desired state for telemetry in the DevfileRegistry
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub telemetry: Option<DevfileRegistryTelemetry>,
    /// DevfileRegistrySpecTLS defines the desired state for TLS in the DevfileRegistry
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<DevfileRegistryTls>,
}

/// Sets the devfile index container spec to be deployed on the Devfile Registry
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DevfileRegistryDevfileIndex {
    /// Sets the container image
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Sets the image pull policy for the container
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imagePullPolicy")]
    pub image_pull_policy: Option<String>,
    /// Sets the memory limit for the container
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "memoryLimit")]
    pub memory_limit: Option<String>,
}

/// DevfileRegistrySpecK8sOnly defines the desired state of the kubernetes-only fields of the DevfileRegistry
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DevfileRegistryK8s {
    /// Ingress class for a Kubernetes cluster. Defaults to nginx.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ingressClass")]
    pub ingress_class: Option<String>,
    /// Ingress domain for a Kubernetes cluster. This MUST be explicitly specified on Kubernetes. There are no defaults
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ingressDomain")]
    pub ingress_domain: Option<String>,
}

/// Sets the OCI registry container spec to be deployed on the Devfile Registry
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DevfileRegistryOciRegistry {
    /// Sets the container image
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Sets the image pull policy for the container
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imagePullPolicy")]
    pub image_pull_policy: Option<String>,
    /// Sets the memory limit for the container
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "memoryLimit")]
    pub memory_limit: Option<String>,
}

/// Sets the registry viewer container spec to be deployed on the Devfile Registry
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DevfileRegistryRegistryViewer {
    /// Sets the container image
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Sets the image pull policy for the container
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imagePullPolicy")]
    pub image_pull_policy: Option<String>,
    /// Sets the memory limit for the container
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "memoryLimit")]
    pub memory_limit: Option<String>,
}

/// DevfileRegistrySpecStorage defines the desired state of the storage for the DevfileRegistry
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DevfileRegistryStorage {
    /// Instructs the operator to deploy the DevfileRegistry with persistent storage Disabled by default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Configures the size of the devfile registry's persistent volume, if enabled. Defaults to 1Gi.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "registryVolumeSize")]
    pub registry_volume_size: Option<String>,
}

/// Telemetry defines the desired state for telemetry in the DevfileRegistry
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DevfileRegistryTelemetry {
    /// Specify a telemetry key to allow devfile specific data to be sent to a client's own Segment analytics source. If the write key is specified then telemetry will be enabled
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The registry name (can be any string) that is used as identifier for devfile telemetry.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "registryName")]
    pub registry_name: Option<String>,
    /// Specify a telemetry write key for the registry viewer component to allow data to be sent to a client's own Segment analytics source. If the write key is specified then telemetry for the registry viewer component will be enabled
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "registryViewerWriteKey")]
    pub registry_viewer_write_key: Option<String>,
}

/// DevfileRegistrySpecTLS defines the desired state for TLS in the DevfileRegistry
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DevfileRegistryTls {
    /// Instructs the operator to deploy the DevfileRegistry with TLS enabled. Enabled by default. Disabling is only recommended for development or test.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Name of an optional, pre-existing TLS secret to use for TLS termination on ingress/route resources.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretName")]
    pub secret_name: Option<String>,
}

/// DevfileRegistryStatus defines the observed state of DevfileRegistry
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DevfileRegistryStatus {
    /// Conditions shows the state devfile registries.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// URL is the exposed URL for the Devfile Registry, and is set in the status after the registry has become available.
    pub url: String,
}

