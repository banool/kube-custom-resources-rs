// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubewharf/kubeadmiral/core.kubeadmiral.io/v1alpha1/propagatedversions.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.5


use serde::{Serialize, Deserialize};

/// PropagatedVersionStatus defines the observed state of PropagatedVersion
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PropagatedVersionStatus {
    /// The last versions produced in each cluster for this resource.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterVersions")]
    pub cluster_versions: Option<Vec<PropagatedVersionStatusClusterVersions>>,
    /// The observed version of the overrides for this resource.
    #[serde(rename = "overridesVersion")]
    pub overrides_version: String,
    /// The observed version of the template for this resource.
    #[serde(rename = "templateVersion")]
    pub template_version: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PropagatedVersionStatusClusterVersions {
    /// The name of the cluster the version is for.
    #[serde(rename = "clusterName")]
    pub cluster_name: String,
    /// The last version produced for the resource by a KubeAdmiral operation.
    pub version: String,
}

