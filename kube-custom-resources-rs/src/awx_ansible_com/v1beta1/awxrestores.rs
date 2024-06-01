// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/ansible/awx-operator/awx.ansible.com/v1beta1/awxrestores.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "awx.ansible.com", version = "v1beta1", kind = "AWXRestore", plural = "awxrestores")]
#[kube(namespaced)]
#[kube(status = "AWXRestoreStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct AWXRestoreSpec {
    /// Additional labels defined on the resource, which should be propagated to child resources
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_labels: Option<Vec<String>>,
    /// Backup directory name, set as a status found on the awxbackup object (backupDirectory)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backup_dir: Option<String>,
    /// AWXBackup object name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backup_name: Option<String>,
    /// Name of the PVC to be restored from, set as a status found on the awxbackup object (backupClaim)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backup_pvc: Option<String>,
    /// (Deprecated) Namespace the PVC is in
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backup_pvc_namespace: Option<String>,
    /// Backup source
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backup_source: Option<AWXRestoreBackupSource>,
    /// Cluster name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    /// nodeSelector for the Postgres pods to backup
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub db_management_pod_node_selector: Option<String>,
    /// Name of the restored deployment. This should be different from the original deployment name if the original deployment still exists.
    pub deployment_name: String,
    /// Force drop the database before restoring. USE WITH CAUTION!
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub force_drop_db: Option<bool>,
    /// The image pull policy
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_pull_policy: Option<AWXRestoreImagePullPolicy>,
    /// Configure no_log for no_log tasks
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_log: Option<bool>,
    /// Registry path to the PostgreSQL container to use
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postgres_image: Option<String>,
    /// PostgreSQL container image version to use
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postgres_image_version: Option<String>,
    /// Label selector used to identify postgres pod for backing up data
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postgres_label_selector: Option<String>,
    /// Resource requirements for the management pod that restores AWX from a backup
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restore_resource_requirements: Option<AWXRestoreRestoreResourceRequirements>,
    /// Maintain some of the recommended `app.kubernetes.io/*` labels on the resource (self)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set_self_labels: Option<bool>,
    /// Overrides for the AWX spec
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec_overrides: Option<BTreeMap<String, serde_json::Value>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AWXRestoreBackupSource {
    #[serde(rename = "Backup CR")]
    BackupCr,
    #[serde(rename = "PVC")]
    Pvc,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AWXRestoreImagePullPolicy {
    Always,
    #[serde(rename = "always")]
    AlwaysX,
    Never,
    #[serde(rename = "never")]
    NeverX,
    IfNotPresent,
    #[serde(rename = "ifnotpresent")]
    Ifnotpresent,
}

/// Resource requirements for the management pod that restores AWX from a backup
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AWXRestoreRestoreResourceRequirements {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<AWXRestoreRestoreResourceRequirementsLimits>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<AWXRestoreRestoreResourceRequirementsRequests>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AWXRestoreRestoreResourceRequirementsLimits {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AWXRestoreRestoreResourceRequirementsRequests {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AWXRestoreStatus {
    /// The resulting conditions when a Service Telemetry is instantiated
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<AWXRestoreStatusConditions>>,
    /// Restore process complete
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "restoreComplete")]
    pub restore_complete: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AWXRestoreStatusConditions {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastTransitionTime")]
    pub last_transition_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

