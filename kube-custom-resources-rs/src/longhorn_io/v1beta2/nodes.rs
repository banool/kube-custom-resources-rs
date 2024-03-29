// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/longhorn/longhorn/longhorn.io/v1beta2/nodes.yaml --derive=Default --derive=PartialEq
// kopium version: 0.17.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;

/// NodeSpec defines the desired state of the Longhorn node
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "longhorn.io", version = "v1beta2", kind = "Node", plural = "nodes")]
#[kube(namespaced)]
#[kube(status = "NodeStatus")]
#[kube(schema = "disabled")]
pub struct NodeSpec {
    /// Allow scheduling replicas on the node.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowScheduling")]
    pub allow_scheduling: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disks: Option<BTreeMap<String, NodeDisks>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "evictionRequested")]
    pub eviction_requested: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "instanceManagerCPURequest")]
    pub instance_manager_cpu_request: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeDisks {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowScheduling")]
    pub allow_scheduling: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "diskDriver")]
    pub disk_driver: Option<NodeDisksDiskDriver>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "diskType")]
    pub disk_type: Option<NodeDisksDiskType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "evictionRequested")]
    pub eviction_requested: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageReserved")]
    pub storage_reserved: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum NodeDisksDiskDriver {
    #[serde(rename = "")]
    KopiumEmpty,
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "aio")]
    Aio,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum NodeDisksDiskType {
    #[serde(rename = "filesystem")]
    Filesystem,
    #[serde(rename = "block")]
    Block,
}

/// NodeStatus defines the observed state of the Longhorn node
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "autoEvicting")]
    pub auto_evicting: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// The status of the disks on the node.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "diskStatus")]
    pub disk_status: Option<BTreeMap<String, NodeStatusDiskStatus>>,
    /// The Region of the node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The status of the snapshot integrity check.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "snapshotCheckStatus")]
    pub snapshot_check_status: Option<NodeStatusSnapshotCheckStatus>,
    /// The Zone of the node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
}

/// The status of the disks on the node.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeStatusDiskStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "diskDriver")]
    pub disk_driver: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "diskName")]
    pub disk_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "diskPath")]
    pub disk_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "diskType")]
    pub disk_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "diskUUID")]
    pub disk_uuid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "filesystemType")]
    pub filesystem_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scheduledReplica")]
    pub scheduled_replica: Option<BTreeMap<String, i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageAvailable")]
    pub storage_available: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageMaximum")]
    pub storage_maximum: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageScheduled")]
    pub storage_scheduled: Option<i64>,
}

/// The status of the snapshot integrity check.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeStatusSnapshotCheckStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastPeriodicCheckedAt")]
    pub last_periodic_checked_at: Option<String>,
}

