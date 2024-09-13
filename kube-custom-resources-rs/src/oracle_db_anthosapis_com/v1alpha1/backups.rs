// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/GoogleCloudPlatform/elcarro-oracle-operator/oracle.db.anthosapis.com/v1alpha1/backups.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// BackupSpec defines the desired state of Backup.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "oracle.db.anthosapis.com", version = "v1alpha1", kind = "Backup", plural = "backups")]
#[kube(namespaced)]
#[kube(status = "BackupStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct BackupSpec {
    /// For a Physical backup this slice can be used to indicate what PDBs, schemas, tablespaces or tables to back up.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backupItems")]
    pub backup_items: Option<Vec<String>>,
    /// For a Physical backup the choices are Backupset and Image Copies. Backupset is the default, but if Image Copies are required, flip this flag to false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backupset: Option<bool>,
    /// For a Physical backup, optionally turn on an additional "check logical" option. The default is off.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "checkLogical")]
    pub check_logical: Option<bool>,
    /// For a Physical backup, optionally turn on compression, by flipping this flag to true. The default is false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compressed: Option<bool>,
    /// For a Physical backup, optionally indicate a degree of parallelism also known as DOP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dop: Option<i32>,
    /// For a Physical backup, optionally specify filesperset. The default depends on a type of backup, generally 64.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filesperset: Option<i32>,
    /// Similar to GcsPath but specify a Gcs directory. The backup sets of physical backup will be transferred to this GcsDir under a folder named .backup.Spec.Name. This field is usually set in .backupSchedule.Spec.backSpec to specify a GcsDir which all scheduled backups will be uploaded to. A user is to ensure proper write access to the bucket from within the Oracle Operator.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "gcsDir")]
    pub gcs_dir: Option<String>,
    /// If set up ahead of time, the backup sets of a physical backup can be optionally transferred to a GCS bucket. A user is to ensure proper write access to the bucket from within the Oracle Operator.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "gcsPath")]
    pub gcs_path: Option<String>,
    /// Instance is a name of an instance to take a backup for.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    /// KeepDataOnDeletion defines whether to keep backup data when backup resource is removed. The default value is false.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keepDataOnDeletion")]
    pub keep_data_on_deletion: Option<bool>,
    /// For a Physical backup, optionally specify an incremental level. The default is 0 (the whole database).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
    /// For a Physical backup, optionally specify a local backup dir. If omitted, /u03/app/oracle/rman is assumed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "localPath")]
    pub local_path: Option<String>,
    /// Mode specifies how this backup will be managed by the operator. if it is not set, the operator tries to create a backup based on the specifications. if it is set to VerifyExists, the operator verifies the existence of a backup.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<BackupMode>,
    /// For a Physical backup, optionally specify a section size in various units (K M G).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sectionSize")]
    pub section_size: Option<IntOrString>,
    /// Backup sub-type, which is only relevant for a Physical backup type (e.g. RMAN). If omitted, the default of Instance(Level) is assumed. Supported options at this point are: Instance or Database level backups.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subType")]
    pub sub_type: Option<BackupSubType>,
    /// For a Physical backup, optionally specify the time threshold. If a threshold is reached, the backup request would time out and error out. The threshold is expressed in minutes. Don't include the unit (minutes), just the integer.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeLimitMinutes")]
    pub time_limit_minutes: Option<i32>,
    /// Type describes a type of a backup to take. Immutable. Available options are: - Snapshot: storage level disk snapshot. - Physical: database engine specific backup that relies on a redo stream / continuous archiving (WAL) and may allow a PITR. Examples include pg_backup, pgBackRest, mysqlbackup. A Physical backup may be file based or database block based (e.g. Oracle RMAN). - Logical: database engine specific backup that relies on running SQL statements, e.g. mysqldump, pg_dump, expdp. If not specified, the default of Snapshot is assumed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<BackupType>,
    /// VolumeSnapshotClass points to a particular CSI driver and is used for taking a volume snapshot. If requested here at the Backup level, this setting overrides the platform default as well as the default set via the Config (global user preferences).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeSnapshotClass")]
    pub volume_snapshot_class: Option<String>,
}

/// BackupSpec defines the desired state of Backup.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum BackupMode {
    VerifyExists,
}

/// BackupSpec defines the desired state of Backup.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum BackupSubType {
    Instance,
    Database,
    Tablespace,
    Datafile,
}

/// BackupSpec defines the desired state of Backup.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum BackupType {
    Snapshot,
    Physical,
    Logical,
}

/// BackupStatus defines the observed state of Backup.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackupStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backupid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backuptime: Option<String>,
    /// Conditions represents the latest available observations of the backup's current state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "gcsPath")]
    pub gcs_path: Option<String>,
    /// Phase is a summary of current state of the Backup.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startTime")]
    pub start_time: Option<String>,
}

