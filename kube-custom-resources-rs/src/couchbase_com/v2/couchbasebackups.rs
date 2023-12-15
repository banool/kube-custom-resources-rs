// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/couchbase-partners/helm-charts/couchbase.com/v2/couchbasebackups.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// CouchbaseBackupSpec is allows the specification of how a Couchbase backup is configured, including when backups are performed, how long they are retained for, and where they are backed up to.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "couchbase.com", version = "v2", kind = "CouchbaseBackup", plural = "couchbasebackups")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
pub struct CouchbaseBackupSpec {
    /// AutoScaling allows the volume size to be dynamically increased. When specified, the backup volume will start with an initial size as defined by `spec.size`, and increase as required.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "autoScaling")]
    pub auto_scaling: Option<CouchbaseBackupAutoScaling>,
    /// Number of times a backup job should try to execute. Once it hits the BackoffLimit it will not run until the next scheduled job.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backoffLimit")]
    pub backoff_limit: Option<i32>,
    /// Number of hours to hold backups for, everything older will be deleted.  More info: https://golang.org/pkg/time/#ParseDuration
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backupRetention")]
    pub backup_retention: Option<String>,
    /// Data allows control over what key-value/document data is included in the backup.  By default, all data is included.  Modifications to this field will only take effect on the next full backup.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<CouchbaseBackupData>,
    /// DefaultRecoveryMethod specifies how cbbackupmgr should recover from broken backup/restore attempts.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultRecoveryMethod")]
    pub default_recovery_method: Option<CouchbaseBackupDefaultRecoveryMethod>,
    /// EphemeralVolume sets backup to use an ephemeral volume instead of a persistent volume. This is used when backing up to a remote cloud provider, where a persistent volume is not needed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ephemeralVolume")]
    pub ephemeral_volume: Option<bool>,
    /// Amount of failed jobs to keep.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failedJobsHistoryLimit")]
    pub failed_jobs_history_limit: Option<i32>,
    /// Full is the schedule on when to take full backups. Used in Full/Incremental and FullOnly backup strategies.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full: Option<CouchbaseBackupFull>,
    /// Incremental is the schedule on when to take incremental backups. Used in Full/Incremental backup strategies.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub incremental: Option<CouchbaseBackupIncremental>,
    /// Number of hours to hold script logs for, everything older will be deleted.  More info: https://golang.org/pkg/time/#ParseDuration
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "logRetention")]
    pub log_retention: Option<String>,
    /// ObjectStore allows for backing up to a remote cloud storage.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "objectStore")]
    pub object_store: Option<CouchbaseBackupObjectStore>,
    /// DEPRECATED - by spec.objectStore.uri Name of S3 bucket to backup to. If non-empty this overrides local backup.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s3bucket: Option<String>,
    /// Services allows control over what services are included in the backup. By default, all service data and metadata are included.  Modifications to this field will only take effect on the next full backup.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub services: Option<CouchbaseBackupServices>,
    /// Size allows the specification of a backup persistent volume, when using volume based backup. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/#resource-units-in-kubernetes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    /// Name of StorageClass to use.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageClassName")]
    pub storage_class_name: Option<String>,
    /// Strategy defines how to perform backups.  `full_only` will only perform full backups, and you must define a schedule in the `spec.full` field.  `full_incremental` will perform periodic full backups, and incremental backups in between.  You must define full and incremental schedules in the `spec.full` and `spec.incremental` fields respectively.  Care should be taken to ensure full and incremental schedules do not overlap, taking into account the backup time, as this will cause failures as the jobs attempt to mount the same backup volume. This field default to `full_incremental`. Info: https://docs.couchbase.com/server/current/backup-restore/cbbackupmgr-strategies.html
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strategy: Option<CouchbaseBackupStrategy>,
    /// Amount of successful jobs to keep.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "successfulJobsHistoryLimit")]
    pub successful_jobs_history_limit: Option<i32>,
    /// How many threads to use during the backup.  This field defaults to 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threads: Option<i64>,
    /// Amount of time to elapse before a completed job is deleted.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ttlSecondsAfterFinished")]
    pub ttl_seconds_after_finished: Option<i32>,
}

/// AutoScaling allows the volume size to be dynamically increased. When specified, the backup volume will start with an initial size as defined by `spec.size`, and increase as required.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CouchbaseBackupAutoScaling {
    /// IncrementPercent controls how much the volume is increased each time the threshold is exceeded, upto a maximum as defined by the limit. This field defaults to 20 if not specified.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "incrementPercent")]
    pub increment_percent: Option<i64>,
    /// Limit imposes a hard limit on the size we can autoscale to.  When not specified no bounds are imposed. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/#resource-units-in-kubernetes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
    /// ThresholdPercent determines the point at which a volume is autoscaled. This represents the percentage of free space remaining on the volume, when less than this threshold, it will trigger a volume expansion. For example, if the volume is 100Gi, and the threshold 20%, then a resize will be triggered when the used capacity exceeds 80Gi, and free space is less than 20Gi.  This field defaults to 20 if not specified.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "thresholdPercent")]
    pub threshold_percent: Option<i64>,
}

/// Data allows control over what key-value/document data is included in the backup.  By default, all data is included.  Modifications to this field will only take effect on the next full backup.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CouchbaseBackupData {
    /// Exclude defines the buckets, scopes or collections that are excluded from the backup. When this field is set, it implies that by default everything will be backed up, and data items can be explicitly excluded.  You may define an exclusion as a bucket -- `my-bucket`, a scope -- `my-bucket.my-scope`, or a collection -- `my-bucket.my-scope.my-collection`. Buckets may contain periods, and therefore must be escaped -- `my\.bucket.my-scope`, as period is the separator used to delimit scopes and collections.  Excluded data cannot overlap e.g. specifying `my-bucket` and `my-bucket.my-scope` is illegal.  This field cannot be used at the same time as included items.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<String>>,
    /// Include defines the buckets, scopes or collections that are included in the backup. When this field is set, it implies that by default nothing will be backed up, and data items must be explicitly included.  You may define an inclusion as a bucket -- `my-bucket`, a scope -- `my-bucket.my-scope`, or a collection -- `my-bucket.my-scope.my-collection`. Buckets may contain periods, and therefore must be escaped -- `my\.bucket.my-scope`, as period is the separator used to delimit scopes and collections.  Included data cannot overlap e.g. specifying `my-bucket` and `my-bucket.my-scope` is illegal.  This field cannot be used at the same time as excluded items.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
}

/// CouchbaseBackupSpec is allows the specification of how a Couchbase backup is configured, including when backups are performed, how long they are retained for, and where they are backed up to.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CouchbaseBackupDefaultRecoveryMethod {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "resume")]
    Resume,
    #[serde(rename = "purge")]
    Purge,
}

/// Full is the schedule on when to take full backups. Used in Full/Incremental and FullOnly backup strategies.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CouchbaseBackupFull {
    /// Schedule takes a cron schedule in string format.
    pub schedule: String,
}

/// Incremental is the schedule on when to take incremental backups. Used in Full/Incremental backup strategies.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CouchbaseBackupIncremental {
    /// Schedule takes a cron schedule in string format.
    pub schedule: String,
}

/// ObjectStore allows for backing up to a remote cloud storage.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CouchbaseBackupObjectStore {
    /// Endpoint contains the configuration for connecting to a custom Azure/S3/GCP compliant object store. If set will override `CouchbaseCluster.spec.backup.objectEndpoint` See https://docs.couchbase.com/server/current/backup-restore/cbbackupmgr-cloud.html#compatible-object-stores
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<CouchbaseBackupObjectStoreEndpoint>,
    /// ObjStoreSecret must contain two fields, access-key-id, secret-access-key and optionally either region or refresh-token. These correspond to the fields used by cbbackupmgr https://docs.couchbase.com/server/current/backup-restore/cbbackupmgr-backup.html#optional-2
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// URI is a reference to a remote object store. This is the prefix of the object store and the bucket name. i.e s3://bucket, az://bucket or gs://bucket.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    /// Whether to allow the backup SDK to attempt to authenticate using the instance metadata api. If set, will override `CouchbaseCluster.spec.backup.useIAM`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "useIAM")]
    pub use_iam: Option<bool>,
}

/// Endpoint contains the configuration for connecting to a custom Azure/S3/GCP compliant object store. If set will override `CouchbaseCluster.spec.backup.objectEndpoint` See https://docs.couchbase.com/server/current/backup-restore/cbbackupmgr-cloud.html#compatible-object-stores
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CouchbaseBackupObjectStoreEndpoint {
    /// The name of the secret, in this namespace, that contains the CA certificate for verification of a TLS endpoint The secret must have the key with the name "tls.crt"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The host/address of the custom object endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// UseVirtualPath will force the AWS SDK to use the new virtual style paths which are often required by S3 compatible object stores.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "useVirtualPath")]
    pub use_virtual_path: Option<bool>,
}

/// Services allows control over what services are included in the backup. By default, all service data and metadata are included.  Modifications to this field will only take effect on the next full backup.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CouchbaseBackupServices {
    /// Analytics enables the backup of analytics data. This field defaults to `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analytics: Option<bool>,
    /// BucketConfig enables the backup of bucket configuration. This field defaults to `true`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bucketConfig")]
    pub bucket_config: Option<bool>,
    /// BucketQuery enables the backup of query metadata for all buckets. This field defaults to `true`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bucketQuery")]
    pub bucket_query: Option<bool>,
    /// ClusterAnalytics enables the backup of cluster-wide analytics data, for example synonyms. This field defaults to `true`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterAnalytics")]
    pub cluster_analytics: Option<bool>,
    /// ClusterQuery enables the backup of cluster level query metadata. This field defaults to `true`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterQuery")]
    pub cluster_query: Option<bool>,
    /// Data enables the backup of key-value data/documents for all buckets. This can be further refined with the couchbasebackups.spec.data configuration. This field defaults to `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<bool>,
    /// Eventing enables the backup of eventing service metadata. This field defaults to `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eventing: Option<bool>,
    /// FTSAliases enables the backup of full-text search alias definitions. This field defaults to `true`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ftsAliases")]
    pub fts_aliases: Option<bool>,
    /// FTSIndexes enables the backup of full-text search index definitions for all buckets. This field defaults to `true`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ftsIndexes")]
    pub fts_indexes: Option<bool>,
    /// GSIndexes enables the backup of global secondary index definitions for all buckets. This field defaults to `true`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "gsIndexes")]
    pub gs_indexes: Option<bool>,
    /// Views enables the backup of view definitions for all buckets. This field defaults to `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub views: Option<bool>,
}

/// CouchbaseBackupSpec is allows the specification of how a Couchbase backup is configured, including when backups are performed, how long they are retained for, and where they are backed up to.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CouchbaseBackupStrategy {
    #[serde(rename = "full_incremental")]
    FullIncremental,
    #[serde(rename = "full_only")]
    FullOnly,
}

/// CouchbaseBackupStatus provides status notifications about the Couchbase backup including when the last backup occurred, whether is succeeded or not, the run time of the backup and the size of the backup.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CouchbaseBackupStatus {
    /// Location of Backup Archive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archive: Option<String>,
    /// Backups gives us a full list of all backups and their respective repository locations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backups: Option<Vec<CouchbaseBackupStatusBackups>>,
    /// CapacityUsed tells us how much of the PVC we are using. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/#resource-units-in-kubernetes
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "capacityUsed")]
    pub capacity_used: Option<String>,
    /// DEPRECATED - field may no longer be populated. Cronjob tells us which Cronjob the job belongs to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cronjob: Option<String>,
    /// Duration tells us how long the last backup took.  More info: https://golang.org/pkg/time/#ParseDuration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    /// Failed indicates whether the most recent backup has failed.
    pub failed: bool,
    /// DEPRECATED - field may no longer be populated. Job tells us which job is running/ran last.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job: Option<String>,
    /// LastFailure tells us the time the last failed backup failed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastFailure")]
    pub last_failure: Option<String>,
    /// LastRun tells us the time the last backup job started.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastRun")]
    pub last_run: Option<String>,
    /// LastSuccess gives us the time the last successful backup finished.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastSuccess")]
    pub last_success: Option<String>,
    /// DEPRECATED - field may no longer be populated. Output reports useful information from the backup_script.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    /// DEPRECATED - field may no longer be populated. Pod tells us which pod is running/ran last.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pod: Option<String>,
    /// Repo is where we are currently performing operations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repo: Option<String>,
    /// Running indicates whether a backup is currently being performed.
    pub running: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CouchbaseBackupStatusBackups {
    /// Full backup inside the repository.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full: Option<String>,
    /// Incremental backups inside the repository.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub incrementals: Option<Vec<String>>,
    /// Name of the repository.
    pub name: String,
}

