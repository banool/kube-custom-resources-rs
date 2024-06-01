// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/GoogleCloudPlatform/elcarro-oracle-operator/oracle.db.anthosapis.com/v1alpha1/pitrs.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// PITRSpec defines the desired state of PITR
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "oracle.db.anthosapis.com", version = "v1alpha1", kind = "PITR", plural = "pitrs")]
#[kube(namespaced)]
#[kube(status = "PITRStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct PITRSpec {
    /// Schedule is a cron-style expression of the schedule on which Backup will be created for PITR. For allowed syntax, see en.wikipedia.org/wiki/Cron and godoc.org/github.com/robfig/cron. Default to backup every 4 hours.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backupSchedule")]
    pub backup_schedule: Option<String>,
    /// Images defines PITR service agent images. This is a required map that allows a customer to specify GCR images.
    pub images: BTreeMap<String, String>,
    /// InstanceRef references to the instance that the PITR applies to.
    #[serde(rename = "instanceRef")]
    pub instance_ref: PITRInstanceRef,
    /// StorageURI is the URI to store PITR backups and redo logs. Currently only gs:// (GCS) schemes are supported.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageURI")]
    pub storage_uri: Option<String>,
}

/// InstanceRef references to the instance that the PITR applies to.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PITRInstanceRef {
    /// `name` is the name of a database instance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// PITRStatus defines the observed state of PITR
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PITRStatus {
    /// AvailableRecoveryWindowSCN represents the actual PITR recoverable SCN ranges for an instance in the current timeline/incarnation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "availableRecoveryWindowSCN")]
    pub available_recovery_window_scn: Option<Vec<PITRStatusAvailableRecoveryWindowScn>>,
    /// AvailableRecoveryWindowTime represents the actual PITR recoverable time ranges for an instance in the current timeline/incarnation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "availableRecoveryWindowTime")]
    pub available_recovery_window_time: Option<Vec<PITRStatusAvailableRecoveryWindowTime>>,
    /// BackupTotal stores the total number of current existing backups managed by a PITR.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backupTotal")]
    pub backup_total: Option<i64>,
    /// Conditions represents the latest available observations of the PITR's current state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// CurrentDatabaseIncarnation stores the current database incarnation number for the PITR enabled instance.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "currentDatabaseIncarnation")]
    pub current_database_incarnation: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PITRStatusAvailableRecoveryWindowScn {
    /// Begin SCN.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub begin: Option<String>,
    /// End SCN.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PITRStatusAvailableRecoveryWindowTime {
    /// Begin time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub begin: Option<String>,
    /// End time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
}

