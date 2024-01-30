// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/k8up-io/k8up/k8up.io/v1/snapshots.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// SnapshotSpec contains all information needed about a restic snapshot so it can be restored.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "k8up.io", version = "v1", kind = "Snapshot", plural = "snapshots")]
#[kube(namespaced)]
#[kube(status = "SnapshotStatus")]
#[kube(schema = "disabled")]
pub struct SnapshotSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paths: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
}

/// SnapshotStatus defines the observed state of Snapshot
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SnapshotStatus {
}

