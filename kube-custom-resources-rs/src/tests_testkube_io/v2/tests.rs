// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubeshop/testkube-operator/tests.testkube.io/v2/tests.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;

/// TestSpec defines the desired state of Test
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "tests.testkube.io", version = "v2", kind = "Test", plural = "tests")]
#[kube(namespaced)]
#[kube(status = "TestStatus")]
#[kube(schema = "disabled")]
pub struct TestSpec {
    /// test content object
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<TestContent>,
    /// additional executor binary arguments
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "executorArgs")]
    pub executor_args: Option<Vec<String>>,
    /// test execution custom name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// DEPRECATED execution params passed to executor
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<BTreeMap<String, String>>,
    /// schedule in cron job format for scheduled test execution
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    /// test type
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    /// Variables are new params with secrets attached
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<BTreeMap<String, TestVariables>>,
}

/// test content object
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TestContent {
    /// test content body
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// repository of test content
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<TestContentRepository>,
    /// test type
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    /// uri of test content
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

/// repository of test content
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TestContentRepository {
    /// branch/tag name for checkout
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    /// commit id (sha) for checkout
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit: Option<String>,
    /// if needed we can checkout particular path (dir or file) in case of BIG/mono repositories
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// git auth token for private repositories
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// VCS repository type
    #[serde(rename = "type")]
    pub r#type: String,
    /// uri of content file or git directory
    pub uri: String,
    /// git auth username for private repositories
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// Variables are new params with secrets attached
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TestVariables {
    /// variable name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// variable type
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    /// variable string value
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// or load it from var source
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "valueFrom")]
    pub value_from: Option<TestVariablesValueFrom>,
}

/// or load it from var source
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TestVariablesValueFrom {
    /// Selects a key of a ConfigMap.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configMapKeyRef")]
    pub config_map_key_ref: Option<TestVariablesValueFromConfigMapKeyRef>,
    /// Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels['<KEY>']`, `metadata.annotations['<KEY>']`, spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldRef")]
    pub field_ref: Option<TestVariablesValueFromFieldRef>,
    /// Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceFieldRef")]
    pub resource_field_ref: Option<TestVariablesValueFromResourceFieldRef>,
    /// Selects a key of a secret in the pod's namespace
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretKeyRef")]
    pub secret_key_ref: Option<TestVariablesValueFromSecretKeyRef>,
}

/// Selects a key of a ConfigMap.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TestVariablesValueFromConfigMapKeyRef {
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
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TestVariablesValueFromFieldRef {
    /// Version of the schema the FieldPath is written in terms of, defaults to "v1".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// Path of the field to select in the specified API version.
    #[serde(rename = "fieldPath")]
    pub field_path: String,
}

/// Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TestVariablesValueFromResourceFieldRef {
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
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TestVariablesValueFromSecretKeyRef {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// TestStatus defines the observed state of Test
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TestStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub executions_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_execution: Option<String>,
}

