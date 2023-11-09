// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --derive PartialEq --docs --filename ./crd-catalog/fluxcd/notification-controller/notification.toolkit.fluxcd.io/v1beta2/receivers.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// ReceiverSpec defines the desired state of the Receiver.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "notification.toolkit.fluxcd.io", version = "v1beta2", kind = "Receiver", plural = "receivers")]
#[kube(namespaced)]
#[kube(status = "ReceiverStatus")]
#[kube(schema = "disabled")]
pub struct ReceiverSpec {
    /// Events specifies the list of event types to handle, e.g. 'push' for GitHub or 'Push Hook' for GitLab.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
    /// Interval at which to reconcile the Receiver with its Secret references.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// A list of resources to be notified about changes.
    pub resources: Vec<ReceiverResources>,
    /// SecretRef specifies the Secret containing the token used to validate the payload authenticity.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretRef")]
    pub secret_ref: Option<ReceiverSecretRef>,
    /// Suspend tells the controller to suspend subsequent events handling for this receiver.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspend: Option<bool>,
    /// Type of webhook sender, used to determine the validation procedure and payload deserialization.
    #[serde(rename = "type")]
    pub r#type: ReceiverType,
}

/// CrossNamespaceObjectReference contains enough information to let you locate the typed referenced object at cluster level
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ReceiverResources {
    /// API version of the referent
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// Kind of the referent
    pub kind: ReceiverResourcesKind,
    /// MatchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed. MatchLabels requires the name to be set to `*`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
    /// Name of the referent If multiple resources are targeted `*` may be set.
    pub name: String,
    /// Namespace of the referent
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// CrossNamespaceObjectReference contains enough information to let you locate the typed referenced object at cluster level
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ReceiverResourcesKind {
    Bucket,
    GitRepository,
    Kustomization,
    HelmRelease,
    HelmChart,
    HelmRepository,
    ImageRepository,
    ImagePolicy,
    ImageUpdateAutomation,
    #[serde(rename = "OCIRepository")]
    OciRepository,
}

/// SecretRef specifies the Secret containing the token used to validate the payload authenticity.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ReceiverSecretRef {
    /// Name of the referent.
    pub name: String,
}

/// ReceiverSpec defines the desired state of the Receiver.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ReceiverType {
    #[serde(rename = "generic")]
    Generic,
    #[serde(rename = "generic-hmac")]
    GenericHmac,
    #[serde(rename = "github")]
    Github,
    #[serde(rename = "gitlab")]
    Gitlab,
    #[serde(rename = "bitbucket")]
    Bitbucket,
    #[serde(rename = "harbor")]
    Harbor,
    #[serde(rename = "dockerhub")]
    Dockerhub,
    #[serde(rename = "quay")]
    Quay,
    #[serde(rename = "gcr")]
    Gcr,
    #[serde(rename = "nexus")]
    Nexus,
    #[serde(rename = "acr")]
    Acr,
}

/// ReceiverStatus defines the observed state of the Receiver.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ReceiverStatus {
    /// Conditions holds the conditions for the Receiver.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<ReceiverStatusConditions>>,
    /// LastHandledReconcileAt holds the value of the most recent reconcile request value, so a change of the annotation value can be detected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastHandledReconcileAt")]
    pub last_handled_reconcile_at: Option<String>,
    /// ObservedGeneration is the last observed generation of the Receiver object.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// URL is the generated incoming webhook address in the format of '/hook/sha256sum(token+name+namespace)'. Deprecated: Replaced by WebhookPath.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// WebhookPath is the generated incoming webhook address in the format of '/hook/sha256sum(token+name+namespace)'.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "webhookPath")]
    pub webhook_path: Option<String>,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ReceiverStatusConditions {
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
    pub status: ReceiverStatusConditionsStatus,
    /// type of condition in CamelCase or in foo.example.com/CamelCase. --- Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be useful (see .node.status.conditions), the ability to deconflict is important. The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ReceiverStatusConditionsStatus {
    True,
    False,
    Unknown,
}

