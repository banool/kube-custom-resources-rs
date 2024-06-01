// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/longhorn/longhorn/longhorn.io/v1beta2/systembackups.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// SystemBackupSpec defines the desired state of the Longhorn SystemBackup
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "longhorn.io", version = "v1beta2", kind = "SystemBackup", plural = "systembackups")]
#[kube(namespaced)]
#[kube(status = "SystemBackupStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct SystemBackupSpec {
    /// The create volume backup policy Can be "if-not-present", "always" or "disabled"
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeBackupPolicy")]
    pub volume_backup_policy: Option<String>,
}

/// SystemBackupStatus defines the observed state of the Longhorn SystemBackup
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SystemBackupStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// The system backup creation time.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "createdAt")]
    pub created_at: Option<String>,
    /// The saved Longhorn manager git commit.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "gitCommit")]
    pub git_commit: Option<String>,
    /// The last time that the system backup was synced into the cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastSyncedAt")]
    pub last_synced_at: Option<String>,
    /// The saved manager image.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "managerImage")]
    pub manager_image: Option<String>,
    /// The node ID of the responsible controller to reconcile this SystemBackup.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownerID")]
    pub owner_id: Option<String>,
    /// The system backup state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The saved Longhorn version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

