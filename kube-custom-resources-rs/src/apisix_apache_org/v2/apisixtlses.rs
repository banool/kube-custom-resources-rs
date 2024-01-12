// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/apache/apisix-ingress-controller/apisix.apache.org/v2/apisixtlses.yaml --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// ApisixTlsSpec is the specification of ApisixSSL.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "apisix.apache.org", version = "v2", kind = "ApisixTls", plural = "apisixtlses")]
#[kube(namespaced)]
#[kube(status = "ApisixTlsStatus")]
#[kube(schema = "disabled")]
pub struct ApisixTlsSpec {
    /// ApisixMutualTlsClientConfig describes the mutual TLS CA and verify depth
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client: Option<ApisixTlsClient>,
    pub hosts: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ingressClassName")]
    pub ingress_class_name: Option<String>,
    /// ApisixSecret describes the Kubernetes Secret name and namespace.
    pub secret: ApisixTlsSecret,
}

/// ApisixMutualTlsClientConfig describes the mutual TLS CA and verify depth
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ApisixTlsClient {
    /// ApisixSecret describes the Kubernetes Secret name and namespace.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "caSecret")]
    pub ca_secret: Option<ApisixTlsClientCaSecret>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub depth: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skip_mtls_uri_regex: Option<Vec<String>>,
}

/// ApisixSecret describes the Kubernetes Secret name and namespace.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ApisixTlsClientCaSecret {
    pub name: String,
    pub namespace: String,
}

/// ApisixSecret describes the Kubernetes Secret name and namespace.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ApisixTlsSecret {
    pub name: String,
    pub namespace: String,
}

/// ApisixStatus is the status report for Apisix ingress Resources
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ApisixTlsStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<ApisixTlsStatusConditions>>,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, type FooStatus struct{     // Represents the observations of a foo's current state.     // Known .status.conditions.type are: "Available", "Progressing", and "Degraded"     // +patchMergeKey=type     // +patchStrategy=merge     // +listType=map     // +listMapKey=type     Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///      // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ApisixTlsStatusConditions {
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
    pub status: ApisixTlsStatusConditionsStatus,
    /// type of condition in CamelCase or in foo.example.com/CamelCase. --- Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be useful (see .node.status.conditions), the ability to deconflict is important. The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, type FooStatus struct{     // Represents the observations of a foo's current state.     // Known .status.conditions.type are: "Available", "Progressing", and "Degraded"     // +patchMergeKey=type     // +patchStrategy=merge     // +listType=map     // +listMapKey=type     Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///      // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ApisixTlsStatusConditionsStatus {
    True,
    False,
    Unknown,
}

