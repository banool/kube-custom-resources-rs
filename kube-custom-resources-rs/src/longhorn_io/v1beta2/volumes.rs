// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/longhorn/longhorn/longhorn.io/v1beta2/volumes.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// VolumeSpec defines the desired state of the Longhorn volume
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "longhorn.io", version = "v1beta2", kind = "Volume", plural = "volumes")]
#[kube(namespaced)]
#[kube(status = "VolumeStatus")]
#[kube(schema = "disabled")]
pub struct VolumeSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Standby")]
    pub standby: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessMode")]
    pub access_mode: Option<VolumeAccessMode>,
    /// Deprecated: Replaced by field `dataEngine`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backendStoreDriver")]
    pub backend_store_driver: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backingImage")]
    pub backing_image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backupCompressionMethod")]
    pub backup_compression_method: Option<VolumeBackupCompressionMethod>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataEngine")]
    pub data_engine: Option<VolumeDataEngine>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataLocality")]
    pub data_locality: Option<VolumeDataLocality>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataSource")]
    pub data_source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "disableFrontend")]
    pub disable_frontend: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "diskSelector")]
    pub disk_selector: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    /// Deprecated: Replaced by field `image`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "engineImage")]
    pub engine_image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fromBackup")]
    pub from_backup: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frontend: Option<VolumeFrontend>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastAttachedBy")]
    pub last_attached_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub migratable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "migrationNodeID")]
    pub migration_node_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeID")]
    pub node_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeSelector")]
    pub node_selector: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "numberOfReplicas")]
    pub number_of_replicas: Option<i64>,
    /// OfflineReplicaRebuilding is used to determine if the offline replica rebuilding feature is enabled or not
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "offlineReplicaRebuilding")]
    pub offline_replica_rebuilding: Option<VolumeOfflineReplicaRebuilding>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replicaAutoBalance")]
    pub replica_auto_balance: Option<VolumeReplicaAutoBalance>,
    /// Replica disk soft anti affinity of the volume. Set enabled to allow replicas to be scheduled in the same disk.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replicaDiskSoftAntiAffinity")]
    pub replica_disk_soft_anti_affinity: Option<VolumeReplicaDiskSoftAntiAffinity>,
    /// Replica soft anti affinity of the volume. Set enabled to allow replicas to be scheduled on the same node.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replicaSoftAntiAffinity")]
    pub replica_soft_anti_affinity: Option<VolumeReplicaSoftAntiAffinity>,
    /// Replica zone soft anti affinity of the volume. Set enabled to allow replicas to be scheduled in the same zone.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replicaZoneSoftAntiAffinity")]
    pub replica_zone_soft_anti_affinity: Option<VolumeReplicaZoneSoftAntiAffinity>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "restoreVolumeRecurringJob")]
    pub restore_volume_recurring_job: Option<VolumeRestoreVolumeRecurringJob>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "revisionCounterDisabled")]
    pub revision_counter_disabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "snapshotDataIntegrity")]
    pub snapshot_data_integrity: Option<VolumeSnapshotDataIntegrity>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "snapshotMaxCount")]
    pub snapshot_max_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "snapshotMaxSize")]
    pub snapshot_max_size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "staleReplicaTimeout")]
    pub stale_replica_timeout: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "unmapMarkSnapChainRemoved")]
    pub unmap_mark_snap_chain_removed: Option<VolumeUnmapMarkSnapChainRemoved>,
}

/// VolumeSpec defines the desired state of the Longhorn volume
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum VolumeAccessMode {
    #[serde(rename = "rwo")]
    Rwo,
    #[serde(rename = "rwx")]
    Rwx,
}

/// VolumeSpec defines the desired state of the Longhorn volume
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum VolumeBackupCompressionMethod {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "lz4")]
    Lz4,
    #[serde(rename = "gzip")]
    Gzip,
}

/// VolumeSpec defines the desired state of the Longhorn volume
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum VolumeDataEngine {
    #[serde(rename = "v1")]
    V1,
    #[serde(rename = "v2")]
    V2,
}

/// VolumeSpec defines the desired state of the Longhorn volume
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum VolumeDataLocality {
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "best-effort")]
    BestEffort,
    #[serde(rename = "strict-local")]
    StrictLocal,
}

/// VolumeSpec defines the desired state of the Longhorn volume
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum VolumeFrontend {
    #[serde(rename = "blockdev")]
    Blockdev,
    #[serde(rename = "iscsi")]
    Iscsi,
    #[serde(rename = "nvmf")]
    Nvmf,
    #[serde(rename = "")]
    KopiumEmpty,
}

/// VolumeSpec defines the desired state of the Longhorn volume
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum VolumeOfflineReplicaRebuilding {
    #[serde(rename = "ignored")]
    Ignored,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "enabled")]
    Enabled,
}

/// VolumeSpec defines the desired state of the Longhorn volume
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum VolumeReplicaAutoBalance {
    #[serde(rename = "ignored")]
    Ignored,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "least-effort")]
    LeastEffort,
    #[serde(rename = "best-effort")]
    BestEffort,
}

/// VolumeSpec defines the desired state of the Longhorn volume
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum VolumeReplicaDiskSoftAntiAffinity {
    #[serde(rename = "ignored")]
    Ignored,
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
}

/// VolumeSpec defines the desired state of the Longhorn volume
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum VolumeReplicaSoftAntiAffinity {
    #[serde(rename = "ignored")]
    Ignored,
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
}

/// VolumeSpec defines the desired state of the Longhorn volume
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum VolumeReplicaZoneSoftAntiAffinity {
    #[serde(rename = "ignored")]
    Ignored,
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
}

/// VolumeSpec defines the desired state of the Longhorn volume
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum VolumeRestoreVolumeRecurringJob {
    #[serde(rename = "ignored")]
    Ignored,
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
}

/// VolumeSpec defines the desired state of the Longhorn volume
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum VolumeSnapshotDataIntegrity {
    #[serde(rename = "ignored")]
    Ignored,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "fast-check")]
    FastCheck,
}

/// VolumeSpec defines the desired state of the Longhorn volume
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum VolumeUnmapMarkSnapChainRemoved {
    #[serde(rename = "ignored")]
    Ignored,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "enabled")]
    Enabled,
}

/// VolumeStatus defines the observed state of the Longhorn volume
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VolumeStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "actualSize")]
    pub actual_size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cloneStatus")]
    pub clone_status: Option<VolumeStatusCloneStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<VolumeStatusConditions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "currentImage")]
    pub current_image: Option<String>,
    /// the node that this volume is currently migrating to
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "currentMigrationNodeID")]
    pub current_migration_node_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "currentNodeID")]
    pub current_node_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "expansionRequired")]
    pub expansion_required: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "frontendDisabled")]
    pub frontend_disabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isStandby")]
    pub is_standby: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kubernetesStatus")]
    pub kubernetes_status: Option<VolumeStatusKubernetesStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastBackup")]
    pub last_backup: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastBackupAt")]
    pub last_backup_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastDegradedAt")]
    pub last_degraded_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "offlineReplicaRebuildingRequired")]
    pub offline_replica_rebuilding_required: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownerID")]
    pub owner_id: Option<String>,
    /// Deprecated.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pendingNodeID")]
    pub pending_node_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "remountRequestedAt")]
    pub remount_requested_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "restoreInitiated")]
    pub restore_initiated: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "restoreRequired")]
    pub restore_required: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub robustness: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "shareEndpoint")]
    pub share_endpoint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "shareState")]
    pub share_state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VolumeStatusCloneStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceVolume")]
    pub source_volume: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VolumeStatusConditions {
    /// Last time we probed the condition.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastProbeTime")]
    pub last_probe_time: Option<String>,
    /// Last time the condition transitioned from one status to another.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastTransitionTime")]
    pub last_transition_time: Option<String>,
    /// Human-readable message indicating details about last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Unique, one-word, CamelCase reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Status is the status of the condition. Can be True, False, Unknown.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Type is the type of the condition.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VolumeStatusKubernetesStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastPVCRefAt")]
    pub last_pvc_ref_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastPodRefAt")]
    pub last_pod_ref_at: Option<String>,
    /// determine if PVC/Namespace is history or not
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pvName")]
    pub pv_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pvStatus")]
    pub pv_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pvcName")]
    pub pvc_name: Option<String>,
    /// determine if Pod/Workload is history or not
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "workloadsStatus")]
    pub workloads_status: Option<Vec<VolumeStatusKubernetesStatusWorkloadsStatus>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VolumeStatusKubernetesStatusWorkloadsStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podName")]
    pub pod_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podStatus")]
    pub pod_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "workloadName")]
    pub workload_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "workloadType")]
    pub workload_type: Option<String>,
}

