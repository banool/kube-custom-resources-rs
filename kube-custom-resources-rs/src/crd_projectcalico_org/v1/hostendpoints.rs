// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/projectcalico/calico/crd.projectcalico.org/v1/hostendpoints.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;

/// HostEndpointSpec contains the specification for a HostEndpoint resource.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "crd.projectcalico.org", version = "v1", kind = "HostEndpoint", plural = "hostendpoints")]
#[kube(schema = "disabled")]
pub struct HostEndpointSpec {
    /// The expected IP addresses (IPv4 and IPv6) of the endpoint. If "InterfaceName" is not present, Calico will look for an interface matching any of the IPs in the list and apply policy to that. Note: 	When using the selector match criteria in an ingress or egress security Policy 	or Profile, Calico converts the selector into a set of IP addresses. For host 	endpoints, the ExpectedIPs field is used for that purpose. (If only the interface 	name is specified, Calico does not learn the IPs of the interface for use in match 	criteria.)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "expectedIPs")]
    pub expected_i_ps: Option<Vec<String>>,
    /// Either "*", or the name of a specific Linux interface to apply policy to; or empty.  "*" indicates that this HostEndpoint governs all traffic to, from or through the default network namespace of the host named by the "Node" field; entering and leaving that namespace via any interface, including those from/to non-host-networked local workloads. 
    ///  If InterfaceName is not "*", this HostEndpoint only governs traffic that enters or leaves the host through the specific interface named by InterfaceName, or - when InterfaceName is empty - through the specific interface that has one of the IPs in ExpectedIPs. Therefore, when InterfaceName is empty, at least one expected IP must be specified.  Only external interfaces (such as "eth0") are supported here; it isn't possible for a HostEndpoint to protect traffic through a specific local workload interface. 
    ///  Note: Only some kinds of policy are implemented for "*" HostEndpoints; initially just pre-DNAT policy.  Please check Calico documentation for the latest position.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "interfaceName")]
    pub interface_name: Option<String>,
    /// The node name identifying the Calico node instance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,
    /// Ports contains the endpoint's named ports, which may be referenced in security policy rules.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<HostEndpointPorts>>,
    /// A list of identifiers of security Profile objects that apply to this endpoint. Each profile is applied in the order that they appear in this list.  Profile rules are applied after the selector-based security policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profiles: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HostEndpointPorts {
    pub name: String,
    pub port: i64,
    pub protocol: IntOrString,
}

