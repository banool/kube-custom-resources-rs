// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/grafana/loki/loki.grafana.com/v1/recordingrules.yaml --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// RecordingRuleSpec defines the desired state of RecordingRule
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "loki.grafana.com", version = "v1", kind = "RecordingRule", plural = "recordingrules")]
#[kube(namespaced)]
#[kube(status = "RecordingRuleStatus")]
#[kube(schema = "disabled")]
pub struct RecordingRuleSpec {
    /// List of groups for recording rules.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<RecordingRuleGroups>>,
    /// TenantID of tenant where the recording rules are evaluated in.
    #[serde(rename = "tenantID")]
    pub tenant_id: String,
}

/// RecordingRuleGroup defines a group of Loki  recording rules.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RecordingRuleGroups {
    /// Interval defines the time interval between evaluation of the given recoding rule.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// Limit defines the number of series a recording rule can produce. 0 is no limit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Name of the recording rule group. Must be unique within all recording rules.
    pub name: String,
    /// Rules defines a list of recording rules
    pub rules: Vec<RecordingRuleGroupsRules>,
}

/// RecordingRuleGroupSpec defines the spec for a Loki recording rule.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RecordingRuleGroupsRules {
    /// The LogQL expression to evaluate. Every evaluation cycle this is evaluated at the current time, and all resultant time series become pending/firing alerts.
    pub expr: String,
    /// Labels to add to each recording rule.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// The name of the time series to output to. Must be a valid metric name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record: Option<String>,
}

/// RecordingRuleStatus defines the observed state of RecordingRule
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RecordingRuleStatus {
    /// Conditions of the RecordingRule generation health.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<RecordingRuleStatusConditions>>,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RecordingRuleStatusConditions {
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
    pub status: RecordingRuleStatusConditionsStatus,
    /// type of condition in CamelCase or in foo.example.com/CamelCase. --- Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be useful (see .node.status.conditions), the ability to deconflict is important. The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RecordingRuleStatusConditionsStatus {
    True,
    False,
    Unknown,
}

