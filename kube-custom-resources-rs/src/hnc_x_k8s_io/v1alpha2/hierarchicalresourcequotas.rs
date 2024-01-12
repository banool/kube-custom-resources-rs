// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubernetes-sigs/hierarchical-namespaces/hnc.x-k8s.io/v1alpha2/hierarchicalresourcequotas.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;

/// Spec defines the desired quota
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "hnc.x-k8s.io", version = "v1alpha2", kind = "HierarchicalResourceQuota", plural = "hierarchicalresourcequotas")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
pub struct HierarchicalResourceQuotaSpec {
    /// Hard is the set of desired hard limits for each named resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hard: Option<BTreeMap<String, IntOrString>>,
}

/// Status defines the actual enforced quota and its current usage
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HierarchicalResourceQuotaStatus {
    /// Hard is the set of enforced hard limits for each named resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hard: Option<BTreeMap<String, IntOrString>>,
    /// LimitsSummary is used by kubectl get hrq, and summarizes the relevant information from .status.hard.limits and .status.used.limits.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "limitsSummary")]
    pub limits_summary: Option<String>,
    /// RequestsSummary is used by kubectl get hrq, and summarizes the relevant information from .status.hard.requests and .status.used.requests.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestsSummary")]
    pub requests_summary: Option<String>,
    /// Used is the current observed total usage of the resource in the namespace and its descendant namespaces.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used: Option<BTreeMap<String, IntOrString>>,
}

