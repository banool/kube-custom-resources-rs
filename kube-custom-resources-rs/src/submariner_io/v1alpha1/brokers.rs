// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/submariner-io/submariner-operator/submariner.io/v1alpha1/brokers.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// BrokerSpec defines the desired state of Broker.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "submariner.io", version = "v1alpha1", kind = "Broker", plural = "brokers")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct BrokerSpec {
    /// List of the components to be installed - any of [service-discovery, connectivity].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<String>>,
    /// List of domains to use for multi-cluster service discovery.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultCustomDomains")]
    pub default_custom_domains: Option<Vec<String>>,
    /// Default cluster size for GlobalCIDR allocated to each cluster (amount of global IPs).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultGlobalnetClusterSize")]
    pub default_globalnet_cluster_size: Option<i64>,
    /// GlobalCIDR supernet range for allocating GlobalCIDRs to each cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "globalnetCIDRRange")]
    pub globalnet_cidr_range: Option<String>,
    /// Enable support for Overlapping CIDRs in connecting clusters.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "globalnetEnabled")]
    pub globalnet_enabled: Option<bool>,
}

/// BrokerStatus defines the observed state of Broker.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BrokerStatus {
}

