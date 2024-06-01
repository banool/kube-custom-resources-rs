// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/grafana/loki/loki.grafana.com/v1/recordingrules.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// RecordingRuleSpec defines the desired state of RecordingRule
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "loki.grafana.com", version = "v1", kind = "RecordingRule", plural = "recordingrules")]
#[kube(namespaced)]
#[kube(status = "RecordingRuleStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct RecordingRuleSpec {
    /// List of groups for recording rules.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<RecordingRuleGroups>>,
    /// TenantID of tenant where the recording rules are evaluated in.
    #[serde(rename = "tenantID")]
    pub tenant_id: String,
}

/// RecordingRuleGroup defines a group of Loki  recording rules.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RecordingRuleGroups {
    /// Interval defines the time interval between evaluation of the given
    /// recoding rule.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// Limit defines the number of series a recording rule can produce. 0 is no limit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Name of the recording rule group. Must be unique within all recording rules.
    pub name: String,
    /// Rules defines a list of recording rules
    pub rules: Vec<RecordingRuleGroupsRules>,
}

/// RecordingRuleGroupSpec defines the spec for a Loki recording rule.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RecordingRuleGroupsRules {
    /// The LogQL expression to evaluate. Every evaluation cycle this is
    /// evaluated at the current time, and all resultant time series become
    /// pending/firing alerts.
    pub expr: String,
    /// Labels to add to each recording rule.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// The name of the time series to output to. Must be a valid metric name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record: Option<String>,
}

/// RecordingRuleStatus defines the observed state of RecordingRule
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RecordingRuleStatus {
    /// Conditions of the RecordingRule generation health.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
}

