// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --derive PartialEq --docs --filename ./crd-catalog/karmada-io/karmada/work.karmada.io/v1alpha1/resourcebindings.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;

/// Spec represents the desired behavior.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "work.karmada.io", version = "v1alpha1", kind = "ResourceBinding", plural = "resourcebindings")]
#[kube(namespaced)]
#[kube(status = "ResourceBindingStatus")]
#[kube(schema = "disabled")]
pub struct ResourceBindingSpec {
    /// Clusters represents target member clusters where the resource to be deployed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<ResourceBindingClusters>>,
    /// Resource represents the Kubernetes resource to be propagated.
    pub resource: ResourceBindingResource,
}

/// TargetCluster represents the identifier of a member cluster.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ResourceBindingClusters {
    /// Name of target cluster.
    pub name: String,
    /// Replicas in target cluster
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
}

/// Resource represents the Kubernetes resource to be propagated.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ResourceBindingResource {
    /// APIVersion represents the API version of the referent.
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    /// Kind represents the Kind of the referent.
    pub kind: String,
    /// Name represents the name of the referent.
    pub name: String,
    /// Namespace represents the namespace for the referent. For non-namespace scoped resources(e.g. 'ClusterRole')，do not need specify Namespace, and for namespace scoped resources, Namespace is required. If Namespace is not specified, means the resource is non-namespace scoped.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Replicas represents the replica number of the referencing resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    /// ReplicaResourceRequirements represents the resources required by each replica.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourcePerReplicas")]
    pub resource_per_replicas: Option<BTreeMap<String, IntOrString>>,
    /// ResourceVersion represents the internal version of the referenced object, that can be used by clients to determine when object has changed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceVersion")]
    pub resource_version: Option<String>,
}

/// Status represents the most recently observed status of the ResourceBinding.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ResourceBindingStatus {
    /// AggregatedStatus represents status list of the resource running in each member cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "aggregatedStatus")]
    pub aggregated_status: Option<Vec<ResourceBindingStatusAggregatedStatus>>,
    /// Conditions contain the different condition statuses.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<ResourceBindingStatusConditions>>,
}

/// AggregatedStatusItem represents status of the resource running in a member cluster.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ResourceBindingStatusAggregatedStatus {
    /// Applied represents if the resource referencing by ResourceBinding or ClusterResourceBinding is successfully applied on the cluster.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applied: Option<bool>,
    /// AppliedMessage is a human readable message indicating details about the applied status. This is usually holds the error message in case of apply failed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appliedMessage")]
    pub applied_message: Option<String>,
    /// ClusterName represents the member cluster name which the resource deployed on.
    #[serde(rename = "clusterName")]
    pub cluster_name: String,
    /// Status reflects running status of current manifest.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<BTreeMap<String, serde_json::Value>>,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ResourceBindingStatusConditions {
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
    pub status: ResourceBindingStatusConditionsStatus,
    /// type of condition in CamelCase or in foo.example.com/CamelCase. --- Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be useful (see .node.status.conditions), the ability to deconflict is important. The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ResourceBindingStatusConditionsStatus {
    True,
    False,
    Unknown,
}

