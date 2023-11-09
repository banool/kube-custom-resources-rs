// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --derive PartialEq --docs --filename ./crd-catalog/karmada-io/karmada/networking.karmada.io/v1alpha1/multiclusterservices.yaml
// kopium version: 0.16.1

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// Spec is the desired state of the MultiClusterService.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "networking.karmada.io", version = "v1alpha1", kind = "MultiClusterService", plural = "multiclusterservices")]
#[kube(namespaced)]
#[kube(status = "MultiClusterServiceStatus")]
#[kube(schema = "disabled")]
pub struct MultiClusterServiceSpec {
    /// Ports is the list of ports that are exposed by this MultiClusterService. No specified port will be filtered out during the service exposure and discovery process. All ports in the referencing service will be exposed by default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<MultiClusterServicePorts>>,
    /// Range specifies the ranges where the referencing service should be exposed. Only valid and optional in case of Types contains CrossCluster. If not set and Types contains CrossCluster, all clusters will be selected, that means the referencing service will be exposed across all registered clusters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<MultiClusterServiceRange>,
    /// Types specifies how to expose the service referencing by this MultiClusterService.
    pub types: Vec<String>,
}

/// ExposurePort describes which port will be exposed.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MultiClusterServicePorts {
    /// Name is the name of the port that needs to be exposed within the service. The port name must be the same as that defined in the service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Port specifies the exposed service port.
    pub port: i32,
}

/// Range specifies the ranges where the referencing service should be exposed. Only valid and optional in case of Types contains CrossCluster. If not set and Types contains CrossCluster, all clusters will be selected, that means the referencing service will be exposed across all registered clusters.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MultiClusterServiceRange {
    /// ClusterNames is the list of clusters to be selected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterNames")]
    pub cluster_names: Option<Vec<String>>,
}

/// Status is the current state of the MultiClusterService.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MultiClusterServiceStatus {
    /// Current service state
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<MultiClusterServiceStatusConditions>>,
    /// LoadBalancer contains the current status of the load-balancer, if one is present.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "loadBalancer")]
    pub load_balancer: Option<MultiClusterServiceStatusLoadBalancer>,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MultiClusterServiceStatusConditions {
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
    pub status: MultiClusterServiceStatusConditionsStatus,
    /// type of condition in CamelCase or in foo.example.com/CamelCase. --- Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be useful (see .node.status.conditions), the ability to deconflict is important. The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MultiClusterServiceStatusConditionsStatus {
    True,
    False,
    Unknown,
}

/// LoadBalancer contains the current status of the load-balancer, if one is present.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MultiClusterServiceStatusLoadBalancer {
    /// Ingress is a list containing ingress points for the load-balancer. Traffic intended for the service should be sent to these ingress points.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ingress: Option<Vec<MultiClusterServiceStatusLoadBalancerIngress>>,
}

/// LoadBalancerIngress represents the status of a load-balancer ingress point: traffic intended for the service should be sent to an ingress point.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MultiClusterServiceStatusLoadBalancerIngress {
    /// Hostname is set for load-balancer ingress points that are DNS based (typically AWS load-balancers)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// IP is set for load-balancer ingress points that are IP based (typically GCE or OpenStack load-balancers)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// Ports is a list of records of service ports If used, every port defined in the service should have an entry in it
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<MultiClusterServiceStatusLoadBalancerIngressPorts>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MultiClusterServiceStatusLoadBalancerIngressPorts {
    /// Error is to record the problem with the service port The format of the error shall comply with the following rules: - built-in error values shall be specified in this file and those shall use CamelCase names - cloud provider specific error values must have names that comply with the format foo.example.com/CamelCase. --- The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Port is the port number of the service port of which status is recorded here
    pub port: i32,
    /// Protocol is the protocol of the service port of which status is recorded here The supported values are: "TCP", "UDP", "SCTP"
    pub protocol: String,
}

