// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/longhorn/longhorn/longhorn.io/v1beta2/backupvolumes.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// BackupVolumeSpec defines the desired state of the Longhorn backup volume
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "longhorn.io", version = "v1beta2", kind = "BackupVolume", plural = "backupvolumes")]
#[kube(namespaced)]
#[kube(status = "BackupVolumeStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct BackupVolumeSpec {
    /// The time to request run sync the remote backup volume.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "syncRequestedAt")]
    pub sync_requested_at: Option<String>,
}

/// BackupVolumeStatus defines the observed state of the Longhorn backup volume
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackupVolumeStatus {
    /// the backing image checksum.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backingImageChecksum")]
    pub backing_image_checksum: Option<String>,
    /// The backing image name.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backingImageName")]
    pub backing_image_name: Option<String>,
    /// The backup volume creation time.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "createdAt")]
    pub created_at: Option<String>,
    /// The backup volume block count.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataStored")]
    pub data_stored: Option<String>,
    /// The backup volume labels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// The latest volume backup time.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastBackupAt")]
    pub last_backup_at: Option<String>,
    /// The latest volume backup name.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastBackupName")]
    pub last_backup_name: Option<String>,
    /// The backup volume config last modification time.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastModificationTime")]
    pub last_modification_time: Option<String>,
    /// The last time that the backup volume was synced into the cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastSyncedAt")]
    pub last_synced_at: Option<String>,
    /// The error messages when call longhorn engine on list or inspect backup volumes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub messages: Option<BTreeMap<String, String>>,
    /// The node ID on which the controller is responsible to reconcile this backup volume CR.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownerID")]
    pub owner_id: Option<String>,
    /// The backup volume size.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    /// the storage class name of pv/pvc binding with the volume.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageClassName")]
    pub storage_class_name: Option<String>,
}

