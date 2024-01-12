// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/openshift/api/network.operator.openshift.io/v1/egressrouters.yaml --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// Specification of the desired egress router.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "network.operator.openshift.io", version = "v1", kind = "EgressRouter", plural = "egressrouters")]
#[kube(namespaced)]
#[kube(status = "EgressRouterStatus")]
#[kube(schema = "disabled")]
pub struct EgressRouterSpec {
    /// List of IP addresses to configure on the pod's secondary interface.
    pub addresses: Vec<EgressRouterAddresses>,
    /// Mode depicts the mode that is used for the egress router. The default mode is "Redirect" and is the only supported mode currently.
    pub mode: EgressRouterMode,
    /// Specification of interface to create/use. The default is macvlan. Currently only macvlan is supported.
    #[serde(rename = "networkInterface")]
    pub network_interface: EgressRouterNetworkInterface,
    /// Redirect represents the configuration parameters specific to redirect mode.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redirect: Option<EgressRouterRedirect>,
}

/// EgressRouterAddress contains a pair of IP CIDR and gateway to be configured on the router's interface
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct EgressRouterAddresses {
    /// IP address of the next-hop gateway, if it cannot be automatically determined. Can be IPv4 or IPv6.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
    /// IP is the address to configure on the router's interface. Can be IPv4 or IPv6.
    pub ip: String,
}

/// Specification of the desired egress router.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum EgressRouterMode {
    Redirect,
}

/// Specification of interface to create/use. The default is macvlan. Currently only macvlan is supported.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct EgressRouterNetworkInterface {
    /// Arguments specific to the interfaceType macvlan
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub macvlan: Option<EgressRouterNetworkInterfaceMacvlan>,
}

/// Arguments specific to the interfaceType macvlan
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct EgressRouterNetworkInterfaceMacvlan {
    /// Name of the master interface. Need not be specified if it can be inferred from the IP address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub master: Option<String>,
    /// Mode depicts the mode that is used for the macvlan interface; one of Bridge|Private|VEPA|Passthru. The default mode is "Bridge".
    pub mode: EgressRouterNetworkInterfaceMacvlanMode,
}

/// Arguments specific to the interfaceType macvlan
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum EgressRouterNetworkInterfaceMacvlanMode {
    Bridge,
    Private,
    #[serde(rename = "VEPA")]
    Vepa,
    Passthru,
}

/// Redirect represents the configuration parameters specific to redirect mode.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct EgressRouterRedirect {
    /// FallbackIP specifies the remote destination's IP address. Can be IPv4 or IPv6. If no redirect rules are specified, all traffic from the router are redirected to this IP. If redirect rules are specified, then any connections on any other port (undefined in the rules) on the router will be redirected to this IP. If redirect rules are specified and no fallback IP is provided, connections on other ports will simply be rejected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fallbackIP")]
    pub fallback_ip: Option<String>,
    /// List of L4RedirectRules that define the DNAT redirection from the pod to the destination in redirect mode.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "redirectRules")]
    pub redirect_rules: Option<Vec<EgressRouterRedirectRedirectRules>>,
}

/// L4RedirectRule defines a DNAT redirection from a given port to a destination IP and port.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct EgressRouterRedirectRedirectRules {
    /// IP specifies the remote destination's IP address. Can be IPv4 or IPv6.
    #[serde(rename = "destinationIP")]
    pub destination_ip: String,
    /// Port is the port number to which clients should send traffic to be redirected.
    pub port: i32,
    /// Protocol can be TCP, SCTP or UDP.
    pub protocol: EgressRouterRedirectRedirectRulesProtocol,
    /// TargetPort allows specifying the port number on the remote destination to which the traffic gets redirected to. If unspecified, the value from "Port" is used.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetPort")]
    pub target_port: Option<i32>,
}

/// L4RedirectRule defines a DNAT redirection from a given port to a destination IP and port.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum EgressRouterRedirectRedirectRulesProtocol {
    #[serde(rename = "TCP")]
    Tcp,
    #[serde(rename = "UDP")]
    Udp,
    #[serde(rename = "SCTP")]
    Sctp,
}

/// Observed status of EgressRouter.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct EgressRouterStatus {
    /// Observed status of the egress router
    pub conditions: Vec<EgressRouterStatusConditions>,
}

/// EgressRouterStatusCondition represents the state of the egress router's managed and monitored components.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct EgressRouterStatusConditions {
    /// LastTransitionTime is the time of the last update to the current status property.
    #[serde(rename = "lastTransitionTime")]
    pub last_transition_time: String,
    /// Message provides additional information about the current condition. This is only to be consumed by humans.  It may contain Line Feed characters (U+000A), which should be rendered as new lines.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Reason is the CamelCase reason for the condition's current status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Status of the condition, one of True, False, Unknown.
    pub status: EgressRouterStatusConditionsStatus,
    /// Type specifies the aspect reported by this condition; one of Available, Progressing, Degraded
    #[serde(rename = "type")]
    pub r#type: EgressRouterStatusConditionsType,
}

/// EgressRouterStatusCondition represents the state of the egress router's managed and monitored components.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum EgressRouterStatusConditionsStatus {
    True,
    False,
    Unknown,
}

/// EgressRouterStatusCondition represents the state of the egress router's managed and monitored components.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum EgressRouterStatusConditionsType {
    Available,
    Progressing,
    Degraded,
}

