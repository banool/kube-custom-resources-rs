// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --derive PartialEq --docs --filename ./crd-catalog/cilium/cilium/cilium.io/v2alpha1/ciliuml2announcementpolicies.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// Spec is a human readable description of a L2 announcement policy
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "cilium.io", version = "v2alpha1", kind = "CiliumL2AnnouncementPolicy", plural = "ciliuml2announcementpolicies")]
#[kube(status = "CiliumL2AnnouncementPolicyStatus")]
#[kube(schema = "disabled")]
pub struct CiliumL2AnnouncementPolicySpec {
    /// If true, the external IPs of the services are announced
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "externalIPs")]
    pub external_i_ps: Option<bool>,
    /// A list of regular expressions that express which network interface(s) should be used to announce the services over. If nil, all network interfaces are used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interfaces: Option<Vec<String>>,
    /// If true, the loadbalancer IPs of the services are announced 
    ///  If nil this policy applies to all services.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "loadBalancerIPs")]
    pub load_balancer_i_ps: Option<bool>,
    /// NodeSelector selects a group of nodes which will announce the IPs for the services selected by the service selector. 
    ///  If nil this policy applies to all nodes.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeSelector")]
    pub node_selector: Option<CiliumL2AnnouncementPolicyNodeSelector>,
    /// ServiceSelector selects a set of services which will be announced over L2 networks. The loadBalancerClass for a service must be nil or specify a supported class, e.g. "io.cilium/l2-announcer". Refer to the following document for additional details regarding load balancer classes: 
    ///  https://kubernetes.io/docs/concepts/services-networking/service/#load-balancer-class 
    ///  If nil this policy applies to all services.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceSelector")]
    pub service_selector: Option<CiliumL2AnnouncementPolicyServiceSelector>,
}

/// NodeSelector selects a group of nodes which will announce the IPs for the services selected by the service selector. 
///  If nil this policy applies to all nodes.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CiliumL2AnnouncementPolicyNodeSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<CiliumL2AnnouncementPolicyNodeSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CiliumL2AnnouncementPolicyNodeSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: CiliumL2AnnouncementPolicyNodeSelectorMatchExpressionsOperator,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CiliumL2AnnouncementPolicyNodeSelectorMatchExpressionsOperator {
    In,
    NotIn,
    Exists,
    DoesNotExist,
}

/// ServiceSelector selects a set of services which will be announced over L2 networks. The loadBalancerClass for a service must be nil or specify a supported class, e.g. "io.cilium/l2-announcer". Refer to the following document for additional details regarding load balancer classes: 
///  https://kubernetes.io/docs/concepts/services-networking/service/#load-balancer-class 
///  If nil this policy applies to all services.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CiliumL2AnnouncementPolicyServiceSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<CiliumL2AnnouncementPolicyServiceSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CiliumL2AnnouncementPolicyServiceSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: CiliumL2AnnouncementPolicyServiceSelectorMatchExpressionsOperator,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CiliumL2AnnouncementPolicyServiceSelectorMatchExpressionsOperator {
    In,
    NotIn,
    Exists,
    DoesNotExist,
}

/// Status is the status of the policy.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CiliumL2AnnouncementPolicyStatus {
    /// Current service state
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<CiliumL2AnnouncementPolicyStatusConditions>>,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CiliumL2AnnouncementPolicyStatusConditions {
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
    pub status: CiliumL2AnnouncementPolicyStatusConditionsStatus,
    /// type of condition in CamelCase or in foo.example.com/CamelCase. --- Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be useful (see .node.status.conditions), the ability to deconflict is important. The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CiliumL2AnnouncementPolicyStatusConditionsStatus {
    True,
    False,
    Unknown,
}

