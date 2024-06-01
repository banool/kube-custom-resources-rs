// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/crossplane/crossplane/pkg.crossplane.io/v1/configurations.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// ConfigurationSpec specifies details about a request to install a
/// configuration to Crossplane.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "pkg.crossplane.io", version = "v1", kind = "Configuration", plural = "configurations")]
#[kube(status = "ConfigurationStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ConfigurationSpec {
    /// Map of string keys and values that can be used to organize and categorize
    /// (scope and select) objects. May match selectors of replication controllers
    /// and services.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "commonLabels")]
    pub common_labels: Option<BTreeMap<String, String>>,
    /// IgnoreCrossplaneConstraints indicates to the package manager whether to
    /// honor Crossplane version constrains specified by the package.
    /// Default is false.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ignoreCrossplaneConstraints")]
    pub ignore_crossplane_constraints: Option<bool>,
    /// Package is the name of the package that is being requested.
    pub package: String,
    /// PackagePullPolicy defines the pull policy for the package.
    /// Default is IfNotPresent.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "packagePullPolicy")]
    pub package_pull_policy: Option<String>,
    /// PackagePullSecrets are named secrets in the same namespace that can be used
    /// to fetch packages from private registries.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "packagePullSecrets")]
    pub package_pull_secrets: Option<Vec<ConfigurationPackagePullSecrets>>,
    /// RevisionActivationPolicy specifies how the package controller should
    /// update from one revision to the next. Options are Automatic or Manual.
    /// Default is Automatic.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "revisionActivationPolicy")]
    pub revision_activation_policy: Option<String>,
    /// RevisionHistoryLimit dictates how the package controller cleans up old
    /// inactive package revisions.
    /// Defaults to 1. Can be disabled by explicitly setting to 0.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "revisionHistoryLimit")]
    pub revision_history_limit: Option<i64>,
    /// SkipDependencyResolution indicates to the package manager whether to skip
    /// resolving dependencies for a package. Setting this value to true may have
    /// unintended consequences.
    /// Default is false.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "skipDependencyResolution")]
    pub skip_dependency_resolution: Option<bool>,
}

/// LocalObjectReference contains enough information to let you locate the
/// referenced object inside the same namespace.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationPackagePullSecrets {
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// ConfigurationStatus represents the observed state of a Configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationStatus {
    /// Conditions of the resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// CurrentIdentifier is the most recent package source that was used to
    /// produce a revision. The package manager uses this field to determine
    /// whether to check for package updates for a given source when
    /// packagePullPolicy is set to IfNotPresent. Manually removing this field
    /// will cause the package manager to check that the current revision is
    /// correct for the given package source.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "currentIdentifier")]
    pub current_identifier: Option<String>,
    /// CurrentRevision is the name of the current package revision. It will
    /// reflect the most up to date revision, whether it has been activated or
    /// not.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "currentRevision")]
    pub current_revision: Option<String>,
}

