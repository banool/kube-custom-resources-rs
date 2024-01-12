// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubernetes-sigs/aws-load-balancer-controller/elbv2.k8s.aws/v1beta1/targetgroupbindings.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;

/// TargetGroupBindingSpec defines the desired state of TargetGroupBinding
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "elbv2.k8s.aws", version = "v1beta1", kind = "TargetGroupBinding", plural = "targetgroupbindings")]
#[kube(namespaced)]
#[kube(status = "TargetGroupBindingStatus")]
#[kube(schema = "disabled")]
pub struct TargetGroupBindingSpec {
    /// ipAddressType specifies whether the target group is of type IPv4 or IPv6. If unspecified, it will be automatically inferred.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipAddressType")]
    pub ip_address_type: Option<TargetGroupBindingIpAddressType>,
    /// networking defines the networking rules to allow ELBV2 LoadBalancer to access targets in TargetGroup.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub networking: Option<TargetGroupBindingNetworking>,
    /// node selector for instance type target groups to only register certain nodes
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeSelector")]
    pub node_selector: Option<TargetGroupBindingNodeSelector>,
    /// serviceRef is a reference to a Kubernetes Service and ServicePort.
    #[serde(rename = "serviceRef")]
    pub service_ref: TargetGroupBindingServiceRef,
    /// targetGroupARN is the Amazon Resource Name (ARN) for the TargetGroup.
    #[serde(rename = "targetGroupARN")]
    pub target_group_arn: String,
    /// targetType is the TargetType of TargetGroup. If unspecified, it will be automatically inferred.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetType")]
    pub target_type: Option<TargetGroupBindingTargetType>,
}

/// TargetGroupBindingSpec defines the desired state of TargetGroupBinding
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TargetGroupBindingIpAddressType {
    #[serde(rename = "ipv4")]
    Ipv4,
    #[serde(rename = "ipv6")]
    Ipv6,
}

/// networking defines the networking rules to allow ELBV2 LoadBalancer to access targets in TargetGroup.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TargetGroupBindingNetworking {
    /// List of ingress rules to allow ELBV2 LoadBalancer to access targets in TargetGroup.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ingress: Option<Vec<TargetGroupBindingNetworkingIngress>>,
}

/// NetworkingIngressRule defines a particular set of traffic that is allowed to access TargetGroup's targets.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TargetGroupBindingNetworkingIngress {
    /// List of peers which should be able to access the targets in TargetGroup. At least one NetworkingPeer should be specified.
    pub from: Vec<TargetGroupBindingNetworkingIngressFrom>,
    /// List of ports which should be made accessible on the targets in TargetGroup. If ports is empty or unspecified, it defaults to all ports with TCP.
    pub ports: Vec<TargetGroupBindingNetworkingIngressPorts>,
}

/// NetworkingPeer defines the source/destination peer for networking rules.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TargetGroupBindingNetworkingIngressFrom {
    /// IPBlock defines an IPBlock peer. If specified, none of the other fields can be set.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipBlock")]
    pub ip_block: Option<TargetGroupBindingNetworkingIngressFromIpBlock>,
    /// SecurityGroup defines a SecurityGroup peer. If specified, none of the other fields can be set.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "securityGroup")]
    pub security_group: Option<TargetGroupBindingNetworkingIngressFromSecurityGroup>,
}

/// IPBlock defines an IPBlock peer. If specified, none of the other fields can be set.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TargetGroupBindingNetworkingIngressFromIpBlock {
    /// CIDR is the network CIDR. Both IPV4 or IPV6 CIDR are accepted.
    pub cidr: String,
}

/// SecurityGroup defines a SecurityGroup peer. If specified, none of the other fields can be set.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TargetGroupBindingNetworkingIngressFromSecurityGroup {
    /// GroupID is the EC2 SecurityGroupID.
    #[serde(rename = "groupID")]
    pub group_id: String,
}

/// NetworkingPort defines the port and protocol for networking rules.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TargetGroupBindingNetworkingIngressPorts {
    /// The port which traffic must match. When NodePort endpoints(instance TargetType) is used, this must be a numerical port. When Port endpoints(ip TargetType) is used, this can be either numerical or named port on pods. if port is unspecified, it defaults to all ports.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<IntOrString>,
    /// The protocol which traffic must match. If protocol is unspecified, it defaults to TCP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<TargetGroupBindingNetworkingIngressPortsProtocol>,
}

/// NetworkingPort defines the port and protocol for networking rules.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TargetGroupBindingNetworkingIngressPortsProtocol {
    #[serde(rename = "TCP")]
    Tcp,
    #[serde(rename = "UDP")]
    Udp,
}

/// node selector for instance type target groups to only register certain nodes
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TargetGroupBindingNodeSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<TargetGroupBindingNodeSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TargetGroupBindingNodeSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// serviceRef is a reference to a Kubernetes Service and ServicePort.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TargetGroupBindingServiceRef {
    /// Name is the name of the Service.
    pub name: String,
    /// Port is the port of the ServicePort.
    pub port: IntOrString,
}

/// TargetGroupBindingSpec defines the desired state of TargetGroupBinding
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TargetGroupBindingTargetType {
    #[serde(rename = "instance")]
    Instance,
    #[serde(rename = "ip")]
    Ip,
}

/// TargetGroupBindingStatus defines the observed state of TargetGroupBinding
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TargetGroupBindingStatus {
    /// The generation observed by the TargetGroupBinding controller.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
}

