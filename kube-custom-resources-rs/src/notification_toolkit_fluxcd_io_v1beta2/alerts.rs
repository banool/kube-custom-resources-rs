// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --derive PartialEq --docs --filename ./crd-catalog/fluxcd/notification-controller/notification.toolkit.fluxcd.io/v1beta2/alerts.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// AlertSpec defines an alerting rule for events involving a list of objects.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "notification.toolkit.fluxcd.io", version = "v1beta2", kind = "Alert", plural = "alerts")]
#[kube(namespaced)]
#[kube(status = "AlertStatus")]
#[kube(schema = "disabled")]
pub struct AlertSpec {
    /// EventMetadata is an optional field for adding metadata to events dispatched by the controller. This can be used for enhancing the context of the event. If a field would override one already present on the original event as generated by the emitter, then the override doesn't happen, i.e. the original value is preserved, and an info log is printed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "eventMetadata")]
    pub event_metadata: Option<BTreeMap<String, String>>,
    /// EventSeverity specifies how to filter events based on severity. If set to 'info' no events will be filtered.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "eventSeverity")]
    pub event_severity: Option<AlertEventSeverity>,
    /// EventSources specifies how to filter events based on the involved object kind, name and namespace.
    #[serde(rename = "eventSources")]
    pub event_sources: Vec<AlertEventSources>,
    /// ExclusionList specifies a list of Golang regular expressions to be used for excluding messages.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "exclusionList")]
    pub exclusion_list: Option<Vec<String>>,
    /// InclusionList specifies a list of Golang regular expressions to be used for including messages.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "inclusionList")]
    pub inclusion_list: Option<Vec<String>>,
    /// ProviderRef specifies which Provider this Alert should use.
    #[serde(rename = "providerRef")]
    pub provider_ref: AlertProviderRef,
    /// Summary holds a short description of the impact and affected cluster.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// Suspend tells the controller to suspend subsequent events handling for this Alert.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspend: Option<bool>,
}

/// AlertSpec defines an alerting rule for events involving a list of objects.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AlertEventSeverity {
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "error")]
    Error,
}

/// CrossNamespaceObjectReference contains enough information to let you locate the typed referenced object at cluster level
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AlertEventSources {
    /// API version of the referent
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// Kind of the referent
    pub kind: AlertEventSourcesKind,
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
pub enum AlertEventSourcesKind {
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

/// ProviderRef specifies which Provider this Alert should use.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AlertProviderRef {
    /// Name of the referent.
    pub name: String,
}

/// AlertStatus defines the observed state of the Alert.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AlertStatus {
    /// Conditions holds the conditions for the Alert.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<AlertStatusConditions>>,
    /// LastHandledReconcileAt holds the value of the most recent reconcile request value, so a change of the annotation value can be detected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastHandledReconcileAt")]
    pub last_handled_reconcile_at: Option<String>,
    /// ObservedGeneration is the last observed generation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AlertStatusConditions {
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
    pub status: AlertStatusConditionsStatus,
    /// type of condition in CamelCase or in foo.example.com/CamelCase. --- Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be useful (see .node.status.conditions), the ability to deconflict is important. The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AlertStatusConditionsStatus {
    True,
    False,
    Unknown,
}

