// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/karmada-io/karmada/networking.karmada.io/v1alpha1/multiclusterservices.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// Spec is the desired state of the MultiClusterService.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "networking.karmada.io", version = "v1alpha1", kind = "MultiClusterService", plural = "multiclusterservices")]
#[kube(namespaced)]
#[kube(status = "MultiClusterServiceStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct MultiClusterServiceSpec {
    /// ConsumerClusters specifies the clusters where the service will be exposed, for clients.
    /// If leave it empty, the service will be exposed to all clusters.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "consumerClusters")]
    pub consumer_clusters: Option<Vec<MultiClusterServiceConsumerClusters>>,
    /// Ports is the list of ports that are exposed by this MultiClusterService.
    /// No specified port will be filtered out during the service
    /// exposure and discovery process.
    /// All ports in the referencing service will be exposed by default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<MultiClusterServicePorts>>,
    /// ProviderClusters specifies the clusters which will provide the service backend.
    /// If leave it empty, we will collect the backend endpoints from all clusters and sync
    /// them to the ConsumerClusters.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "providerClusters")]
    pub provider_clusters: Option<Vec<MultiClusterServiceProviderClusters>>,
    /// Range specifies the ranges where the referencing service should
    /// be exposed.
    /// Only valid and optional in case of Types contains CrossCluster.
    /// If not set and Types contains CrossCluster, all clusters will
    /// be selected, that means the referencing service will be exposed
    /// across all registered clusters.
    /// Deprecated: in favor of ProviderClusters/ConsumerClusters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<MultiClusterServiceRange>,
    /// ServiceConsumptionClusters specifies the clusters where the service will be exposed, for clients.
    /// If leave it empty, the service will be exposed to all clusters.
    /// Deprecated: in favor of ProviderClusters/ConsumerClusters.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceConsumptionClusters")]
    pub service_consumption_clusters: Option<Vec<String>>,
    /// ServiceProvisionClusters specifies the clusters which will provision the service backend.
    /// If leave it empty, we will collect the backend endpoints from all clusters and sync
    /// them to the ServiceConsumptionClusters.
    /// Deprecated: in favor of ProviderClusters/ConsumerClusters.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceProvisionClusters")]
    pub service_provision_clusters: Option<Vec<String>>,
    /// Types specifies how to expose the service referencing by this
    /// MultiClusterService.
    pub types: Vec<String>,
}

/// ClusterSelector specifies the cluster to be selected.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MultiClusterServiceConsumerClusters {
    /// Name is the name of the cluster to be selected.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// ExposurePort describes which port will be exposed.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MultiClusterServicePorts {
    /// Name is the name of the port that needs to be exposed within the service.
    /// The port name must be the same as that defined in the service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Port specifies the exposed service port.
    pub port: i32,
}

/// ClusterSelector specifies the cluster to be selected.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MultiClusterServiceProviderClusters {
    /// Name is the name of the cluster to be selected.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Range specifies the ranges where the referencing service should
/// be exposed.
/// Only valid and optional in case of Types contains CrossCluster.
/// If not set and Types contains CrossCluster, all clusters will
/// be selected, that means the referencing service will be exposed
/// across all registered clusters.
/// Deprecated: in favor of ProviderClusters/ConsumerClusters.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MultiClusterServiceRange {
    /// ClusterNames is the list of clusters to be selected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterNames")]
    pub cluster_names: Option<Vec<String>>,
}

/// Status is the current state of the MultiClusterService.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MultiClusterServiceStatus {
    /// Current service state
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// LoadBalancer contains the current status of the load-balancer,
    /// if one is present.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "loadBalancer")]
    pub load_balancer: Option<MultiClusterServiceStatusLoadBalancer>,
}

/// LoadBalancer contains the current status of the load-balancer,
/// if one is present.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MultiClusterServiceStatusLoadBalancer {
    /// Ingress is a list containing ingress points for the load-balancer.
    /// Traffic intended for the service should be sent to these ingress points.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ingress: Option<Vec<MultiClusterServiceStatusLoadBalancerIngress>>,
}

/// LoadBalancerIngress represents the status of a load-balancer ingress point:
/// traffic intended for the service should be sent to an ingress point.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MultiClusterServiceStatusLoadBalancerIngress {
    /// Hostname is set for load-balancer ingress points that are DNS based
    /// (typically AWS load-balancers)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// IP is set for load-balancer ingress points that are IP based
    /// (typically GCE or OpenStack load-balancers)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// IPMode specifies how the load-balancer IP behaves, and may only be specified when the ip field is specified.
    /// Setting this to "VIP" indicates that traffic is delivered to the node with
    /// the destination set to the load-balancer's IP and port.
    /// Setting this to "Proxy" indicates that traffic is delivered to the node or pod with
    /// the destination set to the node's IP and node port or the pod's IP and port.
    /// Service implementations may use this information to adjust traffic routing.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipMode")]
    pub ip_mode: Option<String>,
    /// Ports is a list of records of service ports
    /// If used, every port defined in the service should have an entry in it
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<MultiClusterServiceStatusLoadBalancerIngressPorts>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MultiClusterServiceStatusLoadBalancerIngressPorts {
    /// Error is to record the problem with the service port
    /// The format of the error shall comply with the following rules:
    /// - built-in error values shall be specified in this file and those shall use
    ///   CamelCase names
    /// - cloud provider specific error values must have names that comply with the
    ///   format foo.example.com/CamelCase.
    /// ---
    /// The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Port is the port number of the service port of which status is recorded here
    pub port: i32,
    /// Protocol is the protocol of the service port of which status is recorded here
    /// The supported values are: "TCP", "UDP", "SCTP"
    pub protocol: String,
}

