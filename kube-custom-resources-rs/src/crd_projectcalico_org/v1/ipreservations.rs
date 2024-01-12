// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/projectcalico/calico/crd.projectcalico.org/v1/ipreservations.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// IPReservationSpec contains the specification for an IPReservation resource.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "crd.projectcalico.org", version = "v1", kind = "IPReservation", plural = "ipreservations")]
#[kube(schema = "disabled")]
pub struct IPReservationSpec {
    /// ReservedCIDRs is a list of CIDRs and/or IP addresses that Calico IPAM will exclude from new allocations.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reservedCIDRs")]
    pub reserved_cid_rs: Option<Vec<String>>,
}

