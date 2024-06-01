// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/istio/istio/networking.istio.io/v1/workloadentries.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// Configuration affecting VMs onboarded into the mesh. See more details at: https://istio.io/docs/reference/config/networking/workload-entry.html
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "networking.istio.io", version = "v1", kind = "WorkloadEntry", plural = "workloadentries")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct WorkloadEntrySpec {
    /// Address associated with the network endpoint without the port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// One or more labels associated with the endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// The locality associated with the endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    /// Network enables Istio to group endpoints resident in the same L3 domain/network.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    /// Set of ports associated with the endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ports: Option<BTreeMap<String, i64>>,
    /// The service account associated with the workload if a sidecar is present in the workload.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceAccount")]
    pub service_account: Option<String>,
    /// The load balancing weight associated with the endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
}

