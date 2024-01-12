// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/projectcalico/calico/crd.projectcalico.org/v1/clusterinformations.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// ClusterInformationSpec contains the values of describing the cluster.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "crd.projectcalico.org", version = "v1", kind = "ClusterInformation", plural = "clusterinformations")]
#[kube(schema = "disabled")]
pub struct ClusterInformationSpec {
    /// CalicoVersion is the version of Calico that the cluster is running
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "calicoVersion")]
    pub calico_version: Option<String>,
    /// ClusterGUID is the GUID of the cluster
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterGUID")]
    pub cluster_guid: Option<String>,
    /// ClusterType describes the type of the cluster
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterType")]
    pub cluster_type: Option<String>,
    /// DatastoreReady is used during significant datastore migrations to signal to components such as Felix that it should wait before accessing the datastore.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "datastoreReady")]
    pub datastore_ready: Option<bool>,
    /// Variant declares which variant of Calico should be active.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
}

