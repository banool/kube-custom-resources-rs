// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws/aws-application-networking-k8s/application-networking.k8s.aws/v1alpha1/targetgrouppolicies.yaml --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// TargetGroupPolicySpec defines the desired state of TargetGroupPolicy.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "application-networking.k8s.aws", version = "v1alpha1", kind = "TargetGroupPolicy", plural = "targetgrouppolicies")]
#[kube(namespaced)]
#[kube(status = "TargetGroupPolicyStatus")]
#[kube(schema = "disabled")]
pub struct TargetGroupPolicySpec {
    /// The health check configuration. 
    ///  Changes to this value will update VPC Lattice resource in place.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "healthCheck")]
    pub health_check: Option<TargetGroupPolicyHealthCheck>,
    /// The protocol to use for routing traffic to the targets. Supported values are HTTP (default) and HTTPS. 
    ///  Changes to this value results in a replacement of VPC Lattice target group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// The protocol version to use. Supported values are HTTP1 (default) and HTTP2. When a policy is behind GRPCRoute, this field value will be ignored as GRPC is only supported through HTTP/2. 
    ///  Changes to this value results in a replacement of VPC Lattice target group.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "protocolVersion")]
    pub protocol_version: Option<String>,
    /// TargetRef points to the kubernetes Service resource that will have this policy attached. 
    ///  This field is following the guidelines of Kubernetes Gateway API policy attachment.
    #[serde(rename = "targetRef")]
    pub target_ref: TargetGroupPolicyTargetRef,
}

/// The health check configuration. 
///  Changes to this value will update VPC Lattice resource in place.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TargetGroupPolicyHealthCheck {
    /// Indicates whether health checking is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The number of consecutive successful health checks required before considering an unhealthy target healthy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "healthyThresholdCount")]
    pub healthy_threshold_count: Option<i64>,
    /// The approximate amount of time, in seconds, between health checks of an individual target.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "intervalSeconds")]
    pub interval_seconds: Option<i64>,
    /// The destination for health checks on the targets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// The port used when performing health checks on targets. If not specified, health check defaults to the port that a target receives traffic on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// The protocol used when performing health checks on targets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<TargetGroupPolicyHealthCheckProtocol>,
    /// The protocol version used when performing health checks on targets. Defaults to HTTP/1.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "protocolVersion")]
    pub protocol_version: Option<TargetGroupPolicyHealthCheckProtocolVersion>,
    /// A regular expression to match HTTP status codes when checking for successful response from a target.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statusMatch")]
    pub status_match: Option<String>,
    /// The amount of time, in seconds, to wait before reporting a target as unhealthy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeoutSeconds")]
    pub timeout_seconds: Option<i64>,
    /// The number of consecutive failed health checks required before considering a target unhealthy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "unhealthyThresholdCount")]
    pub unhealthy_threshold_count: Option<i64>,
}

/// The health check configuration. 
///  Changes to this value will update VPC Lattice resource in place.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TargetGroupPolicyHealthCheckProtocol {
    #[serde(rename = "HTTP")]
    Http,
    #[serde(rename = "HTTPS")]
    Https,
}

/// The health check configuration. 
///  Changes to this value will update VPC Lattice resource in place.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TargetGroupPolicyHealthCheckProtocolVersion {
    #[serde(rename = "HTTP1")]
    Http1,
    #[serde(rename = "HTTP2")]
    Http2,
}

/// TargetRef points to the kubernetes Service resource that will have this policy attached. 
///  This field is following the guidelines of Kubernetes Gateway API policy attachment.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TargetGroupPolicyTargetRef {
    /// Group is the group of the target resource.
    pub group: String,
    /// Kind is kind of the target resource.
    pub kind: String,
    /// Name is the name of the target resource.
    pub name: String,
    /// Namespace is the namespace of the referent. When unspecified, the local namespace is inferred. Even when policy targets a resource in a different namespace, it MUST only apply to traffic originating from the same namespace as the policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// Status defines the current state of TargetGroupPolicy.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TargetGroupPolicyStatus {
    /// Conditions describe the current conditions of the TargetGroup. 
    ///  Implementations should prefer to express Policy conditions using the `PolicyConditionType` and `PolicyConditionReason` constants so that operators and tools can converge on a common vocabulary to describe TargetGroup state. 
    ///  Known condition types are: 
    ///  * "Accepted" * "Ready"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<TargetGroupPolicyStatusConditions>>,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TargetGroupPolicyStatusConditions {
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
    pub status: TargetGroupPolicyStatusConditionsStatus,
    /// type of condition in CamelCase or in foo.example.com/CamelCase. --- Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be useful (see .node.status.conditions), the ability to deconflict is important. The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TargetGroupPolicyStatusConditionsStatus {
    True,
    False,
    Unknown,
}

