// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws-controllers-k8s/networkfirewall-controller/networkfirewall.services.k8s.aws/v1alpha1/rulegroups.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// RuleGroupSpec defines the desired state of RuleGroup.
/// 
/// 
/// The object that defines the rules in a rule group. This, along with RuleGroupResponse,
/// define the rule group. You can retrieve all objects for a rule group by calling
/// DescribeRuleGroup.
/// 
/// 
/// Network Firewall uses a rule group to inspect and control network traffic.
/// You define stateless rule groups to inspect individual packets and you define
/// stateful rule groups to inspect packets in the context of their traffic flow.
/// 
/// 
/// To use a rule group, you include it by reference in an Network Firewall firewall
/// policy, then you use the policy in a firewall. You can reference a rule group
/// from more than one firewall policy, and you can use a firewall policy in
/// more than one firewall.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "networkfirewall.services.k8s.aws", version = "v1alpha1", kind = "RuleGroup", plural = "rulegroups")]
#[kube(namespaced)]
#[kube(status = "RuleGroupStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct RuleGroupSpec {
    /// Indicates whether you want Network Firewall to analyze the stateless rules
    /// in the rule group for rule behavior such as asymmetric routing. If set to
    /// TRUE, Network Firewall runs the analysis and then creates the rule group
    /// for you. To run the stateless rule group analyzer without creating the rule
    /// group, set DryRun to TRUE.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "analyzeRuleGroup")]
    pub analyze_rule_group: Option<bool>,
    /// The maximum operating resources that this rule group can use. Rule group
    /// capacity is fixed at creation. When you update a rule group, you are limited
    /// to this capacity. When you reference a rule group from a firewall policy,
    /// Network Firewall reserves this capacity for the rule group.
    /// 
    /// 
    /// You can retrieve the capacity that would be required for a rule group before
    /// you create the rule group by calling CreateRuleGroup with DryRun set to TRUE.
    /// 
    /// 
    /// You can't change or exceed this capacity when you update the rule group,
    /// so leave room for your rule group to grow.
    /// 
    /// 
    /// Capacity for a stateless rule group
    /// 
    /// 
    /// For a stateless rule group, the capacity required is the sum of the capacity
    /// requirements of the individual rules that you expect to have in the rule
    /// group.
    /// 
    /// 
    /// To calculate the capacity requirement of a single rule, multiply the capacity
    /// requirement values of each of the rule's match settings:
    /// 
    /// 
    ///    * A match setting with no criteria specified has a value of 1.
    /// 
    /// 
    ///    * A match setting with Any specified has a value of 1.
    /// 
    /// 
    ///    * All other match settings have a value equal to the number of elements
    ///    provided in the setting. For example, a protocol setting ["UDP"] and a
    ///    source setting ["10.0.0.0/24"] each have a value of 1. A protocol setting
    ///    ["UDP","TCP"] has a value of 2. A source setting ["10.0.0.0/24","10.0.0.1/24","10.0.0.2/24"]
    ///    has a value of 3.
    /// 
    /// 
    /// A rule with no criteria specified in any of its match settings has a capacity
    /// requirement of 1. A rule with protocol setting ["UDP","TCP"], source setting
    /// ["10.0.0.0/24","10.0.0.1/24","10.0.0.2/24"], and a single specification or
    /// no specification for each of the other match settings has a capacity requirement
    /// of 6.
    /// 
    /// 
    /// Capacity for a stateful rule group
    /// 
    /// 
    /// For a stateful rule group, the minimum capacity required is the number of
    /// individual rules that you expect to have in the rule group.
    pub capacity: i64,
    /// A description of the rule group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Indicates whether you want Network Firewall to just check the validity of
    /// the request, rather than run the request.
    /// 
    /// 
    /// If set to TRUE, Network Firewall checks whether the request can run successfully,
    /// but doesn't actually make the requested changes. The call returns the value
    /// that the request would return if you ran it with dry run set to FALSE, but
    /// doesn't make additions or changes to your resources. This option allows you
    /// to make sure that you have the required permissions to run the request and
    /// that your request parameters are valid.
    /// 
    /// 
    /// If set to FALSE, Network Firewall makes the requested changes to your resources.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dryRun")]
    pub dry_run: Option<bool>,
    /// A complex type that contains settings for encryption of your rule group resources.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "encryptionConfiguration")]
    pub encryption_configuration: Option<RuleGroupEncryptionConfiguration>,
    /// An object that defines the rule group rules.
    /// 
    /// 
    /// You must provide either this rule group setting or a Rules setting, but not
    /// both.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ruleGroup")]
    pub rule_group: Option<RuleGroupRuleGroup>,
    /// The descriptive name of the rule group. You can't change the name of a rule
    /// group after you create it.
    #[serde(rename = "ruleGroupName")]
    pub rule_group_name: String,
    /// A string containing stateful rule group rules specifications in Suricata
    /// flat format, with one rule per line. Use this to import your existing Suricata
    /// compatible rule groups.
    /// 
    /// 
    /// You must provide either this rules setting or a populated RuleGroup setting,
    /// but not both.
    /// 
    /// 
    /// You can provide your rule group specification in Suricata flat format through
    /// this setting when you create or update your rule group. The call response
    /// returns a RuleGroup object that Network Firewall has populated from your
    /// string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rules: Option<String>,
    /// A complex type that contains metadata about the rule group that your own
    /// rule group is copied from. You can use the metadata to keep track of updates
    /// made to the originating rule group.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceMetadata")]
    pub source_metadata: Option<RuleGroupSourceMetadata>,
    /// The key:value pairs to associate with the resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<RuleGroupTags>>,
    /// Indicates whether the rule group is stateless or stateful. If the rule group
    /// is stateless, it contains stateless rules. If it is stateful, it contains
    /// stateful rules.
    #[serde(rename = "type_")]
    pub r#type: String,
}

/// A complex type that contains settings for encryption of your rule group resources.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RuleGroupEncryptionConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keyID")]
    pub key_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type_")]
    pub r#type: Option<String>,
}

/// An object that defines the rule group rules.
/// 
/// 
/// You must provide either this rule group setting or a Rules setting, but not
/// both.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RuleGroupRuleGroup {
    /// Contains a set of IP set references.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "referenceSets")]
    pub reference_sets: Option<RuleGroupRuleGroupReferenceSets>,
    /// Settings that are available for use in the rules in the RuleGroup where this
    /// is defined.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ruleVariables")]
    pub rule_variables: Option<RuleGroupRuleGroupRuleVariables>,
    /// The stateless or stateful rules definitions for use in a single rule group.
    /// Each rule group requires a single RulesSource. You can use an instance of
    /// this for either stateless rules or stateful rules.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rulesSource")]
    pub rules_source: Option<RuleGroupRuleGroupRulesSource>,
    /// Additional options governing how Network Firewall handles the rule group.
    /// You can only use these for stateful rule groups.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statefulRuleOptions")]
    pub stateful_rule_options: Option<RuleGroupRuleGroupStatefulRuleOptions>,
}

/// Contains a set of IP set references.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RuleGroupRuleGroupReferenceSets {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "iPSetReferences")]
    pub i_p_set_references: Option<BTreeMap<String, RuleGroupRuleGroupReferenceSetsIPSetReferences>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RuleGroupRuleGroupReferenceSetsIPSetReferences {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "referenceARN")]
    pub reference_arn: Option<String>,
}

/// Settings that are available for use in the rules in the RuleGroup where this
/// is defined.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RuleGroupRuleGroupRuleVariables {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "iPSets")]
    pub i_p_sets: Option<BTreeMap<String, RuleGroupRuleGroupRuleVariablesIPSets>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "portSets")]
    pub port_sets: Option<BTreeMap<String, RuleGroupRuleGroupRuleVariablesPortSets>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RuleGroupRuleGroupRuleVariablesIPSets {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RuleGroupRuleGroupRuleVariablesPortSets {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<Vec<String>>,
}

/// The stateless or stateful rules definitions for use in a single rule group.
/// Each rule group requires a single RulesSource. You can use an instance of
/// this for either stateless rules or stateful rules.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RuleGroupRuleGroupRulesSource {
    /// Stateful inspection criteria for a domain list rule group.
    /// 
    /// 
    /// For HTTPS traffic, domain filtering is SNI-based. It uses the server name
    /// indicator extension of the TLS handshake.
    /// 
    /// 
    /// By default, Network Firewall domain list inspection only includes traffic
    /// coming from the VPC where you deploy the firewall. To inspect traffic from
    /// IP addresses outside of the deployment VPC, you set the HOME_NET rule variable
    /// to include the CIDR range of the deployment VPC plus the other CIDR ranges.
    /// For more information, see RuleVariables in this guide and Stateful domain
    /// list rule groups in Network Firewall (https://docs.aws.amazon.com/network-firewall/latest/developerguide/stateful-rule-groups-domain-names.html)
    /// in the Network Firewall Developer Guide.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rulesSourceList")]
    pub rules_source_list: Option<RuleGroupRuleGroupRulesSourceRulesSourceList>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rulesString")]
    pub rules_string: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statefulRules")]
    pub stateful_rules: Option<Vec<RuleGroupRuleGroupRulesSourceStatefulRules>>,
    /// Stateless inspection criteria. Each stateless rule group uses exactly one
    /// of these data types to define its stateless rules.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statelessRulesAndCustomActions")]
    pub stateless_rules_and_custom_actions: Option<RuleGroupRuleGroupRulesSourceStatelessRulesAndCustomActions>,
}

/// Stateful inspection criteria for a domain list rule group.
/// 
/// 
/// For HTTPS traffic, domain filtering is SNI-based. It uses the server name
/// indicator extension of the TLS handshake.
/// 
/// 
/// By default, Network Firewall domain list inspection only includes traffic
/// coming from the VPC where you deploy the firewall. To inspect traffic from
/// IP addresses outside of the deployment VPC, you set the HOME_NET rule variable
/// to include the CIDR range of the deployment VPC plus the other CIDR ranges.
/// For more information, see RuleVariables in this guide and Stateful domain
/// list rule groups in Network Firewall (https://docs.aws.amazon.com/network-firewall/latest/developerguide/stateful-rule-groups-domain-names.html)
/// in the Network Firewall Developer Guide.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RuleGroupRuleGroupRulesSourceRulesSourceList {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "generatedRulesType")]
    pub generated_rules_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetTypes")]
    pub target_types: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<String>>,
}

/// A single Suricata rules specification, for use in a stateful rule group.
/// Use this option to specify a simple Suricata rule with protocol, source and
/// destination, ports, direction, and rule options. For information about the
/// Suricata Rules format, see Rules Format (https://suricata.readthedocs.io/en/suricata-6.0.9/rules/intro.html).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RuleGroupRuleGroupRulesSourceStatefulRules {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// The basic rule criteria for Network Firewall to use to inspect packet headers
    /// in stateful traffic flow inspection. Traffic flows that match the criteria
    /// are a match for the corresponding StatefulRule.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub header: Option<RuleGroupRuleGroupRulesSourceStatefulRulesHeader>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ruleOptions")]
    pub rule_options: Option<Vec<RuleGroupRuleGroupRulesSourceStatefulRulesRuleOptions>>,
}

/// The basic rule criteria for Network Firewall to use to inspect packet headers
/// in stateful traffic flow inspection. Traffic flows that match the criteria
/// are a match for the corresponding StatefulRule.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RuleGroupRuleGroupRulesSourceStatefulRulesHeader {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "destinationPort")]
    pub destination_port: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourcePort")]
    pub source_port: Option<String>,
}

/// Additional settings for a stateful rule. This is part of the StatefulRule
/// configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RuleGroupRuleGroupRulesSourceStatefulRulesRuleOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<Vec<String>>,
}

/// Stateless inspection criteria. Each stateless rule group uses exactly one
/// of these data types to define its stateless rules.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RuleGroupRuleGroupRulesSourceStatelessRulesAndCustomActions {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customActions")]
    pub custom_actions: Option<Vec<RuleGroupRuleGroupRulesSourceStatelessRulesAndCustomActionsCustomActions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statelessRules")]
    pub stateless_rules: Option<Vec<RuleGroupRuleGroupRulesSourceStatelessRulesAndCustomActionsStatelessRules>>,
}

/// An optional, non-standard action to use for stateless packet handling. You
/// can define this in addition to the standard action that you must specify.
/// 
/// 
/// You define and name the custom actions that you want to be able to use, and
/// then you reference them by name in your actions settings.
/// 
/// 
/// You can use custom actions in the following places:
/// 
/// 
///    * In a rule group's StatelessRulesAndCustomActions specification. The
///    custom actions are available for use by name inside the StatelessRulesAndCustomActions
///    where you define them. You can use them for your stateless rule actions
///    to specify what to do with a packet that matches the rule's match attributes.
/// 
/// 
///    * In a FirewallPolicy specification, in StatelessCustomActions. The custom
///    actions are available for use inside the policy where you define them.
///    You can use them for the policy's default stateless actions settings to
///    specify what to do with packets that don't match any of the policy's stateless
///    rules.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RuleGroupRuleGroupRulesSourceStatelessRulesAndCustomActionsCustomActions {
    /// A custom action to use in stateless rule actions settings. This is used in
    /// CustomAction.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "actionDefinition")]
    pub action_definition: Option<RuleGroupRuleGroupRulesSourceStatelessRulesAndCustomActionsCustomActionsActionDefinition>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "actionName")]
    pub action_name: Option<String>,
}

/// A custom action to use in stateless rule actions settings. This is used in
/// CustomAction.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RuleGroupRuleGroupRulesSourceStatelessRulesAndCustomActionsCustomActionsActionDefinition {
    /// Stateless inspection criteria that publishes the specified metrics to Amazon
    /// CloudWatch for the matching packet. This setting defines a CloudWatch dimension
    /// value to be published.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "publishMetricAction")]
    pub publish_metric_action: Option<RuleGroupRuleGroupRulesSourceStatelessRulesAndCustomActionsCustomActionsActionDefinitionPublishMetricAction>,
}

/// Stateless inspection criteria that publishes the specified metrics to Amazon
/// CloudWatch for the matching packet. This setting defines a CloudWatch dimension
/// value to be published.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RuleGroupRuleGroupRulesSourceStatelessRulesAndCustomActionsCustomActionsActionDefinitionPublishMetricAction {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<RuleGroupRuleGroupRulesSourceStatelessRulesAndCustomActionsCustomActionsActionDefinitionPublishMetricActionDimensions>>,
}

/// The value to use in an Amazon CloudWatch custom metric dimension. This is
/// used in the PublishMetrics CustomAction. A CloudWatch custom metric dimension
/// is a name/value pair that's part of the identity of a metric.
/// 
/// 
/// Network Firewall sets the dimension name to CustomAction and you provide
/// the dimension value.
/// 
/// 
/// For more information about CloudWatch custom metric dimensions, see Publishing
/// Custom Metrics (https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/publishingMetrics.html#usingDimensions)
/// in the Amazon CloudWatch User Guide (https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/WhatIsCloudWatch.html).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RuleGroupRuleGroupRulesSourceStatelessRulesAndCustomActionsCustomActionsActionDefinitionPublishMetricActionDimensions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// A single stateless rule. This is used in StatelessRulesAndCustomActions.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RuleGroupRuleGroupRulesSourceStatelessRulesAndCustomActionsStatelessRules {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    /// The inspection criteria and action for a single stateless rule. Network Firewall
    /// inspects each packet for the specified matching criteria. When a packet matches
    /// the criteria, Network Firewall performs the rule's actions on the packet.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ruleDefinition")]
    pub rule_definition: Option<RuleGroupRuleGroupRulesSourceStatelessRulesAndCustomActionsStatelessRulesRuleDefinition>,
}

/// The inspection criteria and action for a single stateless rule. Network Firewall
/// inspects each packet for the specified matching criteria. When a packet matches
/// the criteria, Network Firewall performs the rule's actions on the packet.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RuleGroupRuleGroupRulesSourceStatelessRulesAndCustomActionsStatelessRulesRuleDefinition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<String>>,
    /// Criteria for Network Firewall to use to inspect an individual packet in stateless
    /// rule inspection. Each match attributes set can include one or more items
    /// such as IP address, CIDR range, port number, protocol, and TCP flags.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchAttributes")]
    pub match_attributes: Option<RuleGroupRuleGroupRulesSourceStatelessRulesAndCustomActionsStatelessRulesRuleDefinitionMatchAttributes>,
}

/// Criteria for Network Firewall to use to inspect an individual packet in stateless
/// rule inspection. Each match attributes set can include one or more items
/// such as IP address, CIDR range, port number, protocol, and TCP flags.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RuleGroupRuleGroupRulesSourceStatelessRulesAndCustomActionsStatelessRulesRuleDefinitionMatchAttributes {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "destinationPorts")]
    pub destination_ports: Option<Vec<RuleGroupRuleGroupRulesSourceStatelessRulesAndCustomActionsStatelessRulesRuleDefinitionMatchAttributesDestinationPorts>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<RuleGroupRuleGroupRulesSourceStatelessRulesAndCustomActionsStatelessRulesRuleDefinitionMatchAttributesDestinations>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourcePorts")]
    pub source_ports: Option<Vec<RuleGroupRuleGroupRulesSourceStatelessRulesAndCustomActionsStatelessRulesRuleDefinitionMatchAttributesSourcePorts>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<RuleGroupRuleGroupRulesSourceStatelessRulesAndCustomActionsStatelessRulesRuleDefinitionMatchAttributesSources>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tcpFlags")]
    pub tcp_flags: Option<Vec<RuleGroupRuleGroupRulesSourceStatelessRulesAndCustomActionsStatelessRulesRuleDefinitionMatchAttributesTcpFlags>>,
}

/// A single port range specification. This is used for source and destination
/// port ranges in the stateless rule MatchAttributes, SourcePorts, and DestinationPorts
/// settings.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RuleGroupRuleGroupRulesSourceStatelessRulesAndCustomActionsStatelessRulesRuleDefinitionMatchAttributesDestinationPorts {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fromPort")]
    pub from_port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "toPort")]
    pub to_port: Option<i64>,
}

/// A single IP address specification. This is used in the MatchAttributes source
/// and destination specifications.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RuleGroupRuleGroupRulesSourceStatelessRulesAndCustomActionsStatelessRulesRuleDefinitionMatchAttributesDestinations {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "addressDefinition")]
    pub address_definition: Option<String>,
}

/// A single port range specification. This is used for source and destination
/// port ranges in the stateless rule MatchAttributes, SourcePorts, and DestinationPorts
/// settings.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RuleGroupRuleGroupRulesSourceStatelessRulesAndCustomActionsStatelessRulesRuleDefinitionMatchAttributesSourcePorts {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fromPort")]
    pub from_port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "toPort")]
    pub to_port: Option<i64>,
}

/// A single IP address specification. This is used in the MatchAttributes source
/// and destination specifications.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RuleGroupRuleGroupRulesSourceStatelessRulesAndCustomActionsStatelessRulesRuleDefinitionMatchAttributesSources {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "addressDefinition")]
    pub address_definition: Option<String>,
}

/// TCP flags and masks to inspect packets for, used in stateless rules MatchAttributes
/// settings.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RuleGroupRuleGroupRulesSourceStatelessRulesAndCustomActionsStatelessRulesRuleDefinitionMatchAttributesTcpFlags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub masks: Option<Vec<String>>,
}

/// Additional options governing how Network Firewall handles the rule group.
/// You can only use these for stateful rule groups.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RuleGroupRuleGroupStatefulRuleOptions {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ruleOrder")]
    pub rule_order: Option<String>,
}

/// A complex type that contains metadata about the rule group that your own
/// rule group is copied from. You can use the metadata to keep track of updates
/// made to the originating rule group.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RuleGroupSourceMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceARN")]
    pub source_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceUpdateToken")]
    pub source_update_token: Option<String>,
}

/// A key:value pair associated with an Amazon Web Services resource. The key:value
/// pair can be anything you define. Typically, the tag key represents a category
/// (such as "environment") and the tag value represents a specific value within
/// that category (such as "test," "development," or "production"). You can add
/// up to 50 tags to each Amazon Web Services resource.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RuleGroupTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// RuleGroupStatus defines the observed state of RuleGroup
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RuleGroupStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<RuleGroupStatusAckResourceMetadata>,
    /// All CRS managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// The high-level properties of a rule group. This, along with the RuleGroup,
    /// define the rule group. You can retrieve all objects for a rule group by calling
    /// DescribeRuleGroup.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ruleGroupResponse")]
    pub rule_group_response: Option<RuleGroupStatusRuleGroupResponse>,
    /// A token used for optimistic locking. Network Firewall returns a token to
    /// your requests that access the rule group. The token marks the state of the
    /// rule group resource at the time of the request.
    /// 
    /// 
    /// To make changes to the rule group, you provide the token in your request.
    /// Network Firewall uses the token to ensure that the rule group hasn't changed
    /// since you last retrieved it. If it has changed, the operation fails with
    /// an InvalidTokenException. If this happens, retrieve the rule group again
    /// to get a current copy of it with a current token. Reapply your changes as
    /// needed, then try the operation again using the new token.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "updateToken")]
    pub update_token: Option<String>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RuleGroupStatusAckResourceMetadata {
    /// ARN is the Amazon Resource Name for the resource. This is a
    /// globally-unique identifier and is set only by the ACK service controller
    /// once the controller has orchestrated the creation of the resource OR
    /// when it has verified that an "adopted" resource (a resource where the
    /// ARN annotation was set by the Kubernetes user on the CR) exists and
    /// matches the supplied CR's Spec field values.
    /// TODO(vijat@): Find a better strategy for resources that do not have ARN in CreateOutputResponse
    /// https://github.com/aws/aws-controllers-k8s/issues/270
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// OwnerAccountID is the AWS Account ID of the account that owns the
    /// backend AWS service API resource.
    #[serde(rename = "ownerAccountID")]
    pub owner_account_id: String,
    /// Region is the AWS region in which the resource exists or will exist.
    pub region: String,
}

/// The high-level properties of a rule group. This, along with the RuleGroup,
/// define the rule group. You can retrieve all objects for a rule group by calling
/// DescribeRuleGroup.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RuleGroupStatusRuleGroupResponse {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "analysisResults")]
    pub analysis_results: Option<Vec<RuleGroupStatusRuleGroupResponseAnalysisResults>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "consumedCapacity")]
    pub consumed_capacity: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A complex type that contains optional Amazon Web Services Key Management
    /// Service (KMS) encryption settings for your Network Firewall resources. Your
    /// data is encrypted by default with an Amazon Web Services owned key that Amazon
    /// Web Services owns and manages for you. You can use either the Amazon Web
    /// Services owned key, or provide your own customer managed key. To learn more
    /// about KMS encryption of your Network Firewall resources, see Encryption at
    /// rest with Amazon Web Services Key Managment Service (https://docs.aws.amazon.com/kms/latest/developerguide/kms-encryption-at-rest.html)
    /// in the Network Firewall Developer Guide.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "encryptionConfiguration")]
    pub encryption_configuration: Option<RuleGroupStatusRuleGroupResponseEncryptionConfiguration>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastModifiedTime")]
    pub last_modified_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "numberOfAssociations")]
    pub number_of_associations: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ruleGroupARN")]
    pub rule_group_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ruleGroupID")]
    pub rule_group_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ruleGroupName")]
    pub rule_group_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ruleGroupStatus")]
    pub rule_group_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "snsTopic")]
    pub sns_topic: Option<String>,
    /// High-level information about the managed rule group that your own rule group
    /// is copied from. You can use the the metadata to track version updates made
    /// to the originating rule group. You can retrieve all objects for a rule group
    /// by calling DescribeRuleGroup (https://docs.aws.amazon.com/network-firewall/latest/APIReference/API_DescribeRuleGroup.html).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceMetadata")]
    pub source_metadata: Option<RuleGroupStatusRuleGroupResponseSourceMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<RuleGroupStatusRuleGroupResponseTags>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type_")]
    pub r#type: Option<String>,
}

/// The analysis result for Network Firewall's stateless rule group analyzer.
/// Every time you call CreateRuleGroup, UpdateRuleGroup, or DescribeRuleGroup
/// on a stateless rule group, Network Firewall analyzes the stateless rule groups
/// in your account and identifies the rules that might adversely effect your
/// firewall's functionality. For example, if Network Firewall detects a rule
/// that's routing traffic asymmetrically, which impacts the service's ability
/// to properly process traffic, the service includes the rule in a list of analysis
/// results.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RuleGroupStatusRuleGroupResponseAnalysisResults {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "analysisDetail")]
    pub analysis_detail: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "identifiedRuleIDs")]
    pub identified_rule_i_ds: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "identifiedType")]
    pub identified_type: Option<String>,
}

/// A complex type that contains optional Amazon Web Services Key Management
/// Service (KMS) encryption settings for your Network Firewall resources. Your
/// data is encrypted by default with an Amazon Web Services owned key that Amazon
/// Web Services owns and manages for you. You can use either the Amazon Web
/// Services owned key, or provide your own customer managed key. To learn more
/// about KMS encryption of your Network Firewall resources, see Encryption at
/// rest with Amazon Web Services Key Managment Service (https://docs.aws.amazon.com/kms/latest/developerguide/kms-encryption-at-rest.html)
/// in the Network Firewall Developer Guide.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RuleGroupStatusRuleGroupResponseEncryptionConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keyID")]
    pub key_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type_")]
    pub r#type: Option<String>,
}

/// High-level information about the managed rule group that your own rule group
/// is copied from. You can use the the metadata to track version updates made
/// to the originating rule group. You can retrieve all objects for a rule group
/// by calling DescribeRuleGroup (https://docs.aws.amazon.com/network-firewall/latest/APIReference/API_DescribeRuleGroup.html).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RuleGroupStatusRuleGroupResponseSourceMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceARN")]
    pub source_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceUpdateToken")]
    pub source_update_token: Option<String>,
}

/// A key:value pair associated with an Amazon Web Services resource. The key:value
/// pair can be anything you define. Typically, the tag key represents a category
/// (such as "environment") and the tag value represents a specific value within
/// that category (such as "test," "development," or "production"). You can add
/// up to 50 tags to each Amazon Web Services resource.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RuleGroupStatusRuleGroupResponseTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

