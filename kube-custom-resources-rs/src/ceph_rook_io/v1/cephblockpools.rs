// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/rook/rook/ceph.rook.io/v1/cephblockpools.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// NamedBlockPoolSpec allows a block pool to be created with a non-default name.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "ceph.rook.io", version = "v1", kind = "CephBlockPool", plural = "cephblockpools")]
#[kube(namespaced)]
#[kube(status = "CephBlockPoolStatus")]
#[kube(schema = "disabled")]
pub struct CephBlockPoolSpec {
    /// DEPRECATED: use Parameters instead, e.g.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "compressionMode")]
    pub compression_mode: Option<CephBlockPoolCompressionMode>,
    /// The root of the crush hierarchy utilized by the pool
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "crushRoot")]
    pub crush_root: Option<String>,
    /// The device class the OSD should set to for use in the pool
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deviceClass")]
    pub device_class: Option<String>,
    /// EnableRBDStats is used to enable gathering of statistics for all RBD images in the pool
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableRBDStats")]
    pub enable_rbd_stats: Option<bool>,
    /// The erasure code settings
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "erasureCoded")]
    pub erasure_coded: Option<CephBlockPoolErasureCoded>,
    /// The failure domain: osd/host/(region or zone if available) - technically also any type in the crush 
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureDomain")]
    pub failure_domain: Option<String>,
    /// The mirroring settings
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mirroring: Option<CephBlockPoolMirroring>,
    /// The desired name of the pool if different from the CephBlockPool CR name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<CephBlockPoolName>,
    /// Parameters is a list of properties to enable on a given pool
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<BTreeMap<String, String>>,
    /// The quota settings
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quotas: Option<CephBlockPoolQuotas>,
    /// The replication settings
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicated: Option<CephBlockPoolReplicated>,
    /// The mirroring statusCheck
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statusCheck")]
    pub status_check: Option<CephBlockPoolStatusCheck>,
}

/// NamedBlockPoolSpec allows a block pool to be created with a non-default name.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CephBlockPoolCompressionMode {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "passive")]
    Passive,
    #[serde(rename = "aggressive")]
    Aggressive,
    #[serde(rename = "force")]
    Force,
    #[serde(rename = "")]
    KopiumEmpty,
}

/// The erasure code settings
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephBlockPoolErasureCoded {
    /// The algorithm for erasure coding
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    /// Number of coding chunks per object in an erasure coded storage pool (required for erasure-coded pool
    #[serde(rename = "codingChunks")]
    pub coding_chunks: i64,
    /// Number of data chunks per object in an erasure coded storage pool (required for erasure-coded pool t
    #[serde(rename = "dataChunks")]
    pub data_chunks: i64,
}

/// The mirroring settings
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephBlockPoolMirroring {
    /// Enabled whether this pool is mirrored or not
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Mode is the mirroring mode: either pool or image
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// Peers represents the peers spec
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub peers: Option<CephBlockPoolMirroringPeers>,
    /// SnapshotSchedules is the scheduling of snapshot for mirrored images/pools
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "snapshotSchedules")]
    pub snapshot_schedules: Option<Vec<CephBlockPoolMirroringSnapshotSchedules>>,
}

/// Peers represents the peers spec
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephBlockPoolMirroringPeers {
    /// SecretNames represents the Kubernetes Secret names to add rbd-mirror or cephfs-mirror peers
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretNames")]
    pub secret_names: Option<Vec<String>>,
}

/// SnapshotScheduleSpec represents the snapshot scheduling settings of a mirrored pool
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephBlockPoolMirroringSnapshotSchedules {
    /// Interval represent the periodicity of the snapshot.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// Path is the path to snapshot, only valid for CephFS
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// StartTime indicates when to start the snapshot
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startTime")]
    pub start_time: Option<String>,
}

/// NamedBlockPoolSpec allows a block pool to be created with a non-default name.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CephBlockPoolName {
    #[serde(rename = "device_health_metrics")]
    DeviceHealthMetrics,
    #[serde(rename = ".nfs")]
    Nfs,
    #[serde(rename = ".mgr")]
    Mgr,
}

/// The quota settings
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephBlockPoolQuotas {
    /// MaxBytes represents the quota in bytes Deprecated in favor of MaxSize
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxBytes")]
    pub max_bytes: Option<i64>,
    /// MaxObjects represents the quota in objects
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxObjects")]
    pub max_objects: Option<i64>,
    /// MaxSize represents the quota in bytes as a string
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxSize")]
    pub max_size: Option<String>,
}

/// The replication settings
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephBlockPoolReplicated {
    /// HybridStorage represents hybrid storage tier settings
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hybridStorage")]
    pub hybrid_storage: Option<CephBlockPoolReplicatedHybridStorage>,
    /// ReplicasPerFailureDomain the number of replica in the specified failure domain
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replicasPerFailureDomain")]
    pub replicas_per_failure_domain: Option<i64>,
    /// RequireSafeReplicaSize if false allows you to set replica 1
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requireSafeReplicaSize")]
    pub require_safe_replica_size: Option<bool>,
    /// Size - Number of copies per object in a replicated storage pool, including the object itself (requir
    pub size: i64,
    /// SubFailureDomain the name of the sub-failure domain
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subFailureDomain")]
    pub sub_failure_domain: Option<String>,
    /// TargetSizeRatio gives a hint (%) to Ceph in terms of expected consumption of the total cluster capac
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetSizeRatio")]
    pub target_size_ratio: Option<f64>,
}

/// HybridStorage represents hybrid storage tier settings
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephBlockPoolReplicatedHybridStorage {
    /// PrimaryDeviceClass represents high performance tier (for example SSD or NVME) for Primary OSD
    #[serde(rename = "primaryDeviceClass")]
    pub primary_device_class: String,
    /// SecondaryDeviceClass represents low performance tier (for example HDDs) for remaining OSDs
    #[serde(rename = "secondaryDeviceClass")]
    pub secondary_device_class: String,
}

/// The mirroring statusCheck
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephBlockPoolStatusCheck {
    /// HealthCheckSpec represents the health check of an object store bucket
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mirror: Option<CephBlockPoolStatusCheckMirror>,
}

/// HealthCheckSpec represents the health check of an object store bucket
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephBlockPoolStatusCheckMirror {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    /// Interval is the internal in second or minute for the health check to run like 60s for 60 seconds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}

/// CephBlockPoolStatus represents the mirroring status of Ceph Storage Pool
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephBlockPoolStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<CephBlockPoolStatusConditions>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<BTreeMap<String, String>>,
    /// MirroringInfoSpec is the status of the pool mirroring
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mirroringInfo")]
    pub mirroring_info: Option<CephBlockPoolStatusMirroringInfo>,
    /// MirroringStatusSpec is the status of the pool mirroring
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mirroringStatus")]
    pub mirroring_status: Option<CephBlockPoolStatusMirroringStatus>,
    /// ObservedGeneration is the latest generation observed by the controller.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// ConditionType represent a resource's status
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    /// SnapshotScheduleStatusSpec is the status of the snapshot schedule
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "snapshotScheduleStatus")]
    pub snapshot_schedule_status: Option<CephBlockPoolStatusSnapshotScheduleStatus>,
}

/// Condition represents a status condition on any Rook-Ceph Custom Resource.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephBlockPoolStatusConditions {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastHeartbeatTime")]
    pub last_heartbeat_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastTransitionTime")]
    pub last_transition_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// ConditionReason is a reason for a condition
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// ConditionType represent a resource's status
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// MirroringInfoSpec is the status of the pool mirroring
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephBlockPoolStatusMirroringInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastChanged")]
    pub last_changed: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastChecked")]
    pub last_checked: Option<String>,
    /// Mode is the mirroring mode
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// Peers are the list of peer sites connected to that cluster
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub peers: Option<Vec<CephBlockPoolStatusMirroringInfoPeers>>,
    /// SiteName is the current site name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub site_name: Option<String>,
}

/// PeersSpec contains peer details
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephBlockPoolStatusMirroringInfoPeers {
    /// ClientName is the CephX user used to connect to the peer
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_name: Option<String>,
    /// Direction is the peer mirroring direction
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    /// MirrorUUID is the mirror UUID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mirror_uuid: Option<String>,
    /// SiteName is the current site name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub site_name: Option<String>,
    /// UUID is the peer UUID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

/// MirroringStatusSpec is the status of the pool mirroring
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephBlockPoolStatusMirroringStatus {
    /// Details contains potential status errors
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// LastChanged is the last time time the status last changed
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastChanged")]
    pub last_changed: Option<String>,
    /// LastChecked is the last time time the status was checked
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastChecked")]
    pub last_checked: Option<String>,
    /// Summary is the mirroring status summary
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<CephBlockPoolStatusMirroringStatusSummary>,
}

/// Summary is the mirroring status summary
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephBlockPoolStatusMirroringStatusSummary {
    /// DaemonHealth is the health of the mirroring daemon
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub daemon_health: Option<String>,
    /// Health is the mirroring health
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health: Option<String>,
    /// ImageHealth is the health of the mirrored image
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_health: Option<String>,
    /// States is the various state for all mirrored images
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub states: Option<CephBlockPoolStatusMirroringStatusSummaryStates>,
}

/// States is the various state for all mirrored images
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephBlockPoolStatusMirroringStatusSummaryStates {
    /// Error is when the mirroring state is errored
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<i64>,
    /// Replaying is when the replay of the mirroring journal is on-going
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replaying: Option<i64>,
    /// StartingReplay is when the replay of the mirroring journal starts
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub starting_replay: Option<i64>,
    /// Stopped is when the mirroring state is stopped
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stopped: Option<i64>,
    /// StopReplaying is when the replay of the mirroring journal stops
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stopping_replay: Option<i64>,
    /// Syncing is when the image is syncing
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub syncing: Option<i64>,
    /// Unknown is when the mirroring state is unknown
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unknown: Option<i64>,
}

/// SnapshotScheduleStatusSpec is the status of the snapshot schedule
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephBlockPoolStatusSnapshotScheduleStatus {
    /// Details contains potential status errors
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// LastChanged is the last time time the status last changed
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastChanged")]
    pub last_changed: Option<String>,
    /// LastChecked is the last time time the status was checked
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastChecked")]
    pub last_checked: Option<String>,
    /// SnapshotSchedules is the list of snapshots scheduled
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "snapshotSchedules")]
    pub snapshot_schedules: Option<Vec<CephBlockPoolStatusSnapshotScheduleStatusSnapshotSchedules>>,
}

/// SnapshotSchedulesSpec is the list of snapshot scheduled for images in a pool
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephBlockPoolStatusSnapshotScheduleStatusSnapshotSchedules {
    /// Image is the mirrored image
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Items is the list schedules times for a given snapshot
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<CephBlockPoolStatusSnapshotScheduleStatusSnapshotSchedulesItems>>,
    /// Namespace is the RADOS namespace the image is part of
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Pool is the pool name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pool: Option<String>,
}

/// SnapshotSchedule is a schedule
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephBlockPoolStatusSnapshotScheduleStatusSnapshotSchedulesItems {
    /// Interval is the interval in which snapshots will be taken
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// StartTime is the snapshot starting time
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}

