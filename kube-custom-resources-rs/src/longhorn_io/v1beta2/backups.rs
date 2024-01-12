// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/longhorn/longhorn/longhorn.io/v1beta2/backups.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// BackupSpec defines the desired state of the Longhorn backup
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "longhorn.io", version = "v1beta2", kind = "Backup", plural = "backups")]
#[kube(namespaced)]
#[kube(status = "BackupStatus")]
#[kube(schema = "disabled")]
pub struct BackupSpec {
    /// The labels of snapshot backup.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// The snapshot name.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "snapshotName")]
    pub snapshot_name: Option<String>,
    /// The time to request run sync the remote backup.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "syncRequestedAt")]
    pub sync_requested_at: Option<String>,
}

/// BackupStatus defines the observed state of the Longhorn backup
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackupStatus {
    /// The snapshot backup upload finished time.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backupCreatedAt")]
    pub backup_created_at: Option<String>,
    /// Compression method
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "compressionMethod")]
    pub compression_method: Option<String>,
    /// The error message when taking the snapshot backup.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// The labels of snapshot backup.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// The last time that the backup was synced with the remote backup target.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastSyncedAt")]
    pub last_synced_at: Option<String>,
    /// The error messages when calling longhorn engine on listing or inspecting backups.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub messages: Option<BTreeMap<String, String>>,
    /// The node ID on which the controller is responsible to reconcile this backup CR.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownerID")]
    pub owner_id: Option<String>,
    /// The snapshot backup progress.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<i64>,
    /// The address of the replica that runs snapshot backup.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replicaAddress")]
    pub replica_address: Option<String>,
    /// The snapshot size.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    /// The snapshot creation time.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "snapshotCreatedAt")]
    pub snapshot_created_at: Option<String>,
    /// The snapshot name.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "snapshotName")]
    pub snapshot_name: Option<String>,
    /// The backup creation state. Can be "", "InProgress", "Completed", "Error", "Unknown".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The snapshot backup URL.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The volume's backing image name.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeBackingImageName")]
    pub volume_backing_image_name: Option<String>,
    /// The volume creation time.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeCreated")]
    pub volume_created: Option<String>,
    /// The volume name.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeName")]
    pub volume_name: Option<String>,
    /// The volume size.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeSize")]
    pub volume_size: Option<String>,
}

