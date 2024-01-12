// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/mariadb-operator/mariadb-operator/mariadb.mmontes.io/v1alpha1/grants.yaml --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// GrantSpec defines the desired state of Grant
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "mariadb.mmontes.io", version = "v1alpha1", kind = "Grant", plural = "grants")]
#[kube(namespaced)]
#[kube(status = "GrantStatus")]
#[kube(schema = "disabled")]
pub struct GrantSpec {
    /// Database to use in the Grant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    /// GrantOption to use in the Grant.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "grantOption")]
    pub grant_option: Option<bool>,
    /// Host to use in the Grant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// MariaDBRef is a reference to a MariaDB object.
    #[serde(rename = "mariaDbRef")]
    pub maria_db_ref: GrantMariaDbRef,
    /// Privileges to use in the Grant.
    pub privileges: Vec<String>,
    /// RequeueInterval is used to perform requeue reconcilizations.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requeueInterval")]
    pub requeue_interval: Option<String>,
    /// RetryInterval is the interval used to perform retries.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "retryInterval")]
    pub retry_interval: Option<String>,
    /// Table to use in the Grant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    /// Username to use in the Grant.
    pub username: String,
}

/// MariaDBRef is a reference to a MariaDB object.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GrantMariaDbRef {
    /// API version of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// If referring to a piece of an object instead of an entire object, this string should contain a valid JSON/Go field access statement, such as desiredState.manifest.containers[2]. For example, if the object reference is to a container within a pod, this would take on a value like: "spec.containers{name}" (where "name" refers to the name of the container that triggered the event) or if no container name is specified "spec.containers[2]" (container with index 2 in this pod). This syntax is chosen only to have some well-defined way of referencing a part of an object. TODO: this design is not final and this field is subject to change in the future.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldPath")]
    pub field_path: Option<String>,
    /// Kind of the referent. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Specific resourceVersion to which this reference is made, if any. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceVersion")]
    pub resource_version: Option<String>,
    /// UID of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#uids
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    /// WaitForIt indicates whether the controller using this reference should wait for MariaDB to be ready.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "waitForIt")]
    pub wait_for_it: Option<bool>,
}

/// GrantStatus defines the observed state of Grant
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GrantStatus {
    /// Conditions for the Grant object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<GrantStatusConditions>>,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GrantStatusConditions {
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
    pub status: GrantStatusConditionsStatus,
    /// type of condition in CamelCase or in foo.example.com/CamelCase. --- Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be useful (see .node.status.conditions), the ability to deconflict is important. The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum GrantStatusConditionsStatus {
    True,
    False,
    Unknown,
}

