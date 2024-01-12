// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kiegroup/kogito-operator/app.kiegroup.org/v1beta1/kogitoinfras.yaml --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;

/// KogitoInfraSpec defines the desired state of KogitoInfra.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "app.kiegroup.org", version = "v1beta1", kind = "KogitoInfra", plural = "kogitoinfras")]
#[kube(namespaced)]
#[kube(status = "KogitoInfraStatus")]
#[kube(schema = "disabled")]
pub struct KogitoInfraSpec {
    /// List of secret that should be mounted to the services as envs
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configMapEnvFromReferences")]
    pub config_map_env_from_references: Option<Vec<String>>,
    /// List of configmap that should be added to the services bound to this infra instance
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configMapVolumeReferences")]
    pub config_map_volume_references: Option<Vec<KogitoInfraConfigMapVolumeReferences>>,
    /// Environment variables to be added to the runtime container. Keys must be a C_IDENTIFIER.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub envs: Option<Vec<KogitoInfraEnvs>>,
    /// Optional properties which would be needed to setup correct runtime/service configuration, based on the resource type. 
    ///  For example, MongoDB will require `username` and `database` as properties for a correct setup, else it will fail
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "infraProperties")]
    pub infra_properties: Option<BTreeMap<String, String>>,
    /// Resource for the service. Example: Infinispan/Kafka/Keycloak.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<KogitoInfraResource>,
    /// List of secret that should be mounted to the services as envs
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretEnvFromReferences")]
    pub secret_env_from_references: Option<Vec<String>>,
    /// List of secret that should be munted to the services bound to this infra instance
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretVolumeReferences")]
    pub secret_volume_references: Option<Vec<KogitoInfraSecretVolumeReferences>>,
}

/// VolumeReference represents the source of a volume to mount.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct KogitoInfraConfigMapVolumeReferences {
    /// Permission on the file mounted as volume on deployment. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. Defaults to 0644.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fileMode")]
    pub file_mode: Option<i32>,
    /// Path within the container at which the volume should be mounted.  Must not contain ':'. Default mount path is /home/kogito/config
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mountPath")]
    pub mount_path: Option<String>,
    /// This must match the Name of a ConfigMap.
    pub name: String,
    /// Specify whether the Secret or its keys must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// EnvVar represents an environment variable present in a Container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct KogitoInfraEnvs {
    /// Name of the environment variable. Must be a C_IDENTIFIER.
    pub name: String,
    /// Variable references $(VAR_NAME) are expanded using the previously defined environment variables in the container and any service environment variables. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Defaults to "".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Source for the environment variable's value. Cannot be used if value is not empty.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "valueFrom")]
    pub value_from: Option<KogitoInfraEnvsValueFrom>,
}

/// Source for the environment variable's value. Cannot be used if value is not empty.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct KogitoInfraEnvsValueFrom {
    /// Selects a key of a ConfigMap.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configMapKeyRef")]
    pub config_map_key_ref: Option<KogitoInfraEnvsValueFromConfigMapKeyRef>,
    /// Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels['<KEY>']`, `metadata.annotations['<KEY>']`, spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldRef")]
    pub field_ref: Option<KogitoInfraEnvsValueFromFieldRef>,
    /// Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceFieldRef")]
    pub resource_field_ref: Option<KogitoInfraEnvsValueFromResourceFieldRef>,
    /// Selects a key of a secret in the pod's namespace
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretKeyRef")]
    pub secret_key_ref: Option<KogitoInfraEnvsValueFromSecretKeyRef>,
}

/// Selects a key of a ConfigMap.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct KogitoInfraEnvsValueFromConfigMapKeyRef {
    /// The key to select.
    pub key: String,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the ConfigMap or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels['<KEY>']`, `metadata.annotations['<KEY>']`, spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct KogitoInfraEnvsValueFromFieldRef {
    /// Version of the schema the FieldPath is written in terms of, defaults to "v1".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// Path of the field to select in the specified API version.
    #[serde(rename = "fieldPath")]
    pub field_path: String,
}

/// Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct KogitoInfraEnvsValueFromResourceFieldRef {
    /// Container name: required for volumes, optional for env vars
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerName")]
    pub container_name: Option<String>,
    /// Specifies the output format of the exposed resources, defaults to "1"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub divisor: Option<IntOrString>,
    /// Required: resource to select
    pub resource: String,
}

/// Selects a key of a secret in the pod's namespace
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct KogitoInfraEnvsValueFromSecretKeyRef {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// Resource for the service. Example: Infinispan/Kafka/Keycloak.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct KogitoInfraResource {
    /// APIVersion describes the API Version of referred Kubernetes resource for example, infinispan.org/v1
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    /// Kind describes the kind of referred Kubernetes resource for example, Infinispan
    pub kind: String,
    /// Name of referred resource.
    pub name: String,
    /// Namespace where referred resource exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// VolumeReference represents the source of a volume to mount.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct KogitoInfraSecretVolumeReferences {
    /// Permission on the file mounted as volume on deployment. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. Defaults to 0644.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fileMode")]
    pub file_mode: Option<i32>,
    /// Path within the container at which the volume should be mounted.  Must not contain ':'. Default mount path is /home/kogito/config
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mountPath")]
    pub mount_path: Option<String>,
    /// This must match the Name of a ConfigMap.
    pub name: String,
    /// Specify whether the Secret or its keys must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// KogitoInfraStatus defines the observed state of KogitoInfra.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct KogitoInfraStatus {
    /// History of conditions for the resource
    pub conditions: Vec<KogitoInfraStatusConditions>,
    /// List of Configmap that should be mounted to the services as envs
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configMapEnvFromReferences")]
    pub config_map_env_from_references: Option<Vec<String>>,
    /// List of configmap that should be added as volume mount to this infra instance
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configMapVolumeReferences")]
    pub config_map_volume_references: Option<Vec<KogitoInfraStatusConfigMapVolumeReferences>>,
    /// Environment variables to be added to the runtime container. Keys must be a C_IDENTIFIER.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<KogitoInfraStatusEnv>>,
    /// List of secret that should be mounted to the services as envs
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretEnvFromReferences")]
    pub secret_env_from_references: Option<Vec<String>>,
    /// List of secret that should be added as volume mount to this infra instance
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretVolumeReferences")]
    pub secret_volume_references: Option<Vec<KogitoInfraStatusSecretVolumeReferences>>,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct KogitoInfraStatusConditions {
    /// lastTransitionTime is the last time the condition transitioned from one status to another. This should be when the underlying condition changed.  If that is not known, then using the time when the API field changed is acceptable.
    #[serde(rename = "lastTransitionTime")]
    pub last_transition_time: String,
    /// message is a human readable message indicating details about the transition. This may be an empty string.
    pub message: String,
    /// observedGeneration represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date with respect to the current state of the instance.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// reason contains a programmatic identifier indicating the reason for the condition's last transition. Producers of specific condition types may define expected values and meanings for this field, and whether the values are considered a guaranteed API. The value should be a CamelCase string. This field may not be empty.
    pub reason: String,
    /// status of the condition, one of True, False, Unknown.
    pub status: KogitoInfraStatusConditionsStatus,
    /// type of condition in CamelCase or in foo.example.com/CamelCase. --- Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be useful (see .node.status.conditions), the ability to deconflict is important. The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum KogitoInfraStatusConditionsStatus {
    True,
    False,
    Unknown,
}

/// VolumeReference represents the source of a volume to mount.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct KogitoInfraStatusConfigMapVolumeReferences {
    /// Permission on the file mounted as volume on deployment. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. Defaults to 0644.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fileMode")]
    pub file_mode: Option<i32>,
    /// Path within the container at which the volume should be mounted.  Must not contain ':'. Default mount path is /home/kogito/config
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mountPath")]
    pub mount_path: Option<String>,
    /// This must match the Name of a ConfigMap.
    pub name: String,
    /// Specify whether the Secret or its keys must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// EnvVar represents an environment variable present in a Container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct KogitoInfraStatusEnv {
    /// Name of the environment variable. Must be a C_IDENTIFIER.
    pub name: String,
    /// Variable references $(VAR_NAME) are expanded using the previously defined environment variables in the container and any service environment variables. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Defaults to "".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Source for the environment variable's value. Cannot be used if value is not empty.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "valueFrom")]
    pub value_from: Option<KogitoInfraStatusEnvValueFrom>,
}

/// Source for the environment variable's value. Cannot be used if value is not empty.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct KogitoInfraStatusEnvValueFrom {
    /// Selects a key of a ConfigMap.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configMapKeyRef")]
    pub config_map_key_ref: Option<KogitoInfraStatusEnvValueFromConfigMapKeyRef>,
    /// Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels['<KEY>']`, `metadata.annotations['<KEY>']`, spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldRef")]
    pub field_ref: Option<KogitoInfraStatusEnvValueFromFieldRef>,
    /// Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceFieldRef")]
    pub resource_field_ref: Option<KogitoInfraStatusEnvValueFromResourceFieldRef>,
    /// Selects a key of a secret in the pod's namespace
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretKeyRef")]
    pub secret_key_ref: Option<KogitoInfraStatusEnvValueFromSecretKeyRef>,
}

/// Selects a key of a ConfigMap.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct KogitoInfraStatusEnvValueFromConfigMapKeyRef {
    /// The key to select.
    pub key: String,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the ConfigMap or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels['<KEY>']`, `metadata.annotations['<KEY>']`, spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct KogitoInfraStatusEnvValueFromFieldRef {
    /// Version of the schema the FieldPath is written in terms of, defaults to "v1".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// Path of the field to select in the specified API version.
    #[serde(rename = "fieldPath")]
    pub field_path: String,
}

/// Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct KogitoInfraStatusEnvValueFromResourceFieldRef {
    /// Container name: required for volumes, optional for env vars
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerName")]
    pub container_name: Option<String>,
    /// Specifies the output format of the exposed resources, defaults to "1"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub divisor: Option<IntOrString>,
    /// Required: resource to select
    pub resource: String,
}

/// Selects a key of a secret in the pod's namespace
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct KogitoInfraStatusEnvValueFromSecretKeyRef {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// VolumeReference represents the source of a volume to mount.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct KogitoInfraStatusSecretVolumeReferences {
    /// Permission on the file mounted as volume on deployment. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. Defaults to 0644.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fileMode")]
    pub file_mode: Option<i32>,
    /// Path within the container at which the volume should be mounted.  Must not contain ':'. Default mount path is /home/kogito/config
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mountPath")]
    pub mount_path: Option<String>,
    /// This must match the Name of a ConfigMap.
    pub name: String,
    /// Specify whether the Secret or its keys must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

