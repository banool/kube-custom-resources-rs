// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/stolostron/cluster-templates-operator/clustertemplate.openshift.io/v1alpha1/clustertemplatequotas.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "clustertemplate.openshift.io", version = "v1alpha1", kind = "ClusterTemplateQuota", plural = "clustertemplatequotas")]
#[kube(namespaced)]
#[kube(status = "ClusterTemplateQuotaStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ClusterTemplateQuotaSpec {
    /// Represents all ClusterTemplates which can be used in given namespace
    #[serde(rename = "allowedTemplates")]
    pub allowed_templates: Vec<ClusterTemplateQuotaAllowedTemplates>,
    /// Total budget for all clusters within given namespace
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub budget: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterTemplateQuotaAllowedTemplates {
    /// Defines how many instances of the ClusterTemplate can exist
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// Template instance will be removed after specified time This is a Duration value; see https://pkg.go.dev/time#ParseDuration for accepted formats. Note: due to discrepancies in validation vs parsing, we use a Pattern instead of `Format=duration`. See https://bugzilla.redhat.com/show_bug.cgi?id=2050332 https://github.com/kubernetes/apimachinery/issues/131 https://github.com/kubernetes/apiextensions-apiserver/issues/56
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deleteAfter")]
    pub delete_after: Option<String>,
    /// Name of the ClusterTemplate
    pub name: String,
}

/// ClusterTemplateQuotaStatus defines the observed state of ClusterTemplateQuota
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterTemplateQuotaStatus {
    /// How much budget is currenly spent
    #[serde(rename = "budgetSpent")]
    pub budget_spent: i64,
    /// Which instances are in use
    #[serde(rename = "templateInstances")]
    pub template_instances: Vec<ClusterTemplateQuotaStatusTemplateInstances>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterTemplateQuotaStatusTemplateInstances {
    /// Defines how many instances of the ClusterTemplate exist
    pub count: i64,
    /// Name of the ClusterTemplate
    pub name: String,
}

