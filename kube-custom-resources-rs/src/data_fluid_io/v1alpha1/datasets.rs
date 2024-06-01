// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/fluid-cloudnative/fluid/data.fluid.io/v1alpha1/datasets.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// DatasetSpec defines the desired state of Dataset
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "data.fluid.io", version = "v1alpha1", kind = "Dataset", plural = "datasets")]
#[kube(namespaced)]
#[kube(status = "DatasetStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct DatasetSpec {
    /// AccessModes contains all ways the volume backing the PVC can be mounted
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessModes")]
    pub access_modes: Option<Vec<String>>,
    /// DataRestoreLocation is the location to load data of dataset  been backuped
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataRestoreLocation")]
    pub data_restore_location: Option<DatasetDataRestoreLocation>,
    /// Mount Points to be mounted on cache runtime. <br> This field can be empty because some runtimes don't need to mount external storage (e.g. <a href="https://v6d.io/">Vineyard</a>).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mounts: Option<Vec<DatasetMounts>>,
    /// NodeAffinity defines constraints that limit what nodes this dataset can be cached to. This field influences the scheduling of pods that use the cached dataset.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeAffinity")]
    pub node_affinity: Option<DatasetNodeAffinity>,
    /// The owner of the dataset
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<DatasetOwner>,
    /// Manage switch for opening Multiple datasets single node deployment or not TODO(xieydd) In future, evaluate node resources and runtime resources to decide whether to turn them on
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placement: Option<DatasetPlacement>,
    /// Runtimes for supporting dataset (e.g. AlluxioRuntime)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runtimes: Option<Vec<DatasetRuntimes>>,
    /// SharedEncryptOptions is the encryptOption to all mount
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sharedEncryptOptions")]
    pub shared_encrypt_options: Option<Vec<DatasetSharedEncryptOptions>>,
    /// SharedOptions is the options to all mount
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sharedOptions")]
    pub shared_options: Option<BTreeMap<String, String>>,
    /// If specified, the pod's tolerations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tolerations: Option<Vec<DatasetTolerations>>,
}

/// DataRestoreLocation is the location to load data of dataset  been backuped
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatasetDataRestoreLocation {
    /// NodeName describes the nodeName of restore if Path is  in the form of local://subpath
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeName")]
    pub node_name: Option<String>,
    /// Path describes the path of restore, in the form of  local://subpath or pvc://<pvcName>/subpath
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

/// Mount describes a mounting. <br> Refer to <a href="https://docs.alluxio.io/os/user/stable/en/ufs/S3.html">Alluxio Storage Integrations</a> for more info
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatasetMounts {
    /// The secret information
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "encryptOptions")]
    pub encrypt_options: Option<Vec<DatasetMountsEncryptOptions>>,
    /// MountPoint is the mount point of source.
    #[serde(rename = "mountPoint")]
    pub mount_point: String,
    /// The name of mount
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The Mount Options. <br> Refer to <a href="https://docs.alluxio.io/os/user/stable/en/reference/Properties-List.html">Mount Options</a>.  <br> The option has Prefix 'fs.' And you can Learn more from <a href="https://docs.alluxio.io/os/user/stable/en/ufs/S3.html">The Storage Integrations</a>
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<BTreeMap<String, String>>,
    /// The path of mount, if not set will be /{Name}
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Optional: Defaults to false (read-write).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "readOnly")]
    pub read_only: Option<bool>,
    /// Optional: Defaults to false (shared).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shared: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatasetMountsEncryptOptions {
    /// The name of encryptOption
    pub name: String,
    /// The valueFrom of encryptOption
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "valueFrom")]
    pub value_from: Option<DatasetMountsEncryptOptionsValueFrom>,
}

/// The valueFrom of encryptOption
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatasetMountsEncryptOptionsValueFrom {
    /// The encryptInfo obtained from secret
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretKeyRef")]
    pub secret_key_ref: Option<DatasetMountsEncryptOptionsValueFromSecretKeyRef>,
}

/// The encryptInfo obtained from secret
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatasetMountsEncryptOptionsValueFromSecretKeyRef {
    /// The required key in the secret
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The name of required secret
    pub name: String,
}

/// NodeAffinity defines constraints that limit what nodes this dataset can be cached to. This field influences the scheduling of pods that use the cached dataset.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatasetNodeAffinity {
    /// Required specifies hard node constraints that must be met.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<DatasetNodeAffinityRequired>,
}

/// Required specifies hard node constraints that must be met.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatasetNodeAffinityRequired {
    /// Required. A list of node selector terms. The terms are ORed.
    #[serde(rename = "nodeSelectorTerms")]
    pub node_selector_terms: Vec<DatasetNodeAffinityRequiredNodeSelectorTerms>,
}

/// A null or empty node selector term matches no objects. The requirements of them are ANDed. The TopologySelectorTerm type implements a subset of the NodeSelectorTerm.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatasetNodeAffinityRequiredNodeSelectorTerms {
    /// A list of node selector requirements by node's labels.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<DatasetNodeAffinityRequiredNodeSelectorTermsMatchExpressions>>,
    /// A list of node selector requirements by node's fields.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchFields")]
    pub match_fields: Option<Vec<DatasetNodeAffinityRequiredNodeSelectorTermsMatchFields>>,
}

/// A node selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatasetNodeAffinityRequiredNodeSelectorTermsMatchExpressions {
    /// The label key that the selector applies to.
    pub key: String,
    /// Represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
    pub operator: String,
    /// An array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. If the operator is Gt or Lt, the values array must have a single element, which will be interpreted as an integer. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// A node selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatasetNodeAffinityRequiredNodeSelectorTermsMatchFields {
    /// The label key that the selector applies to.
    pub key: String,
    /// Represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
    pub operator: String,
    /// An array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. If the operator is Gt or Lt, the values array must have a single element, which will be interpreted as an integer. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// The owner of the dataset
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatasetOwner {
    /// The gid to run the alluxio runtime
    pub gid: i64,
    /// The group name to run the alluxio runtime
    pub group: String,
    /// The uid to run the alluxio runtime
    pub uid: i64,
    /// The user name to run the alluxio runtime
    pub user: String,
}

/// DatasetSpec defines the desired state of Dataset
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DatasetPlacement {
    Exclusive,
    #[serde(rename = "")]
    KopiumEmpty,
    Shared,
}

/// Runtime describes a runtime to be used to support dataset
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatasetRuntimes {
    /// Category the runtime object belongs to (e.g. Accelerate)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Runtime master replicas
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "masterReplicas")]
    pub master_replicas: Option<i32>,
    /// Name of the runtime object
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the runtime object
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Runtime object's type (e.g. Alluxio)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatasetSharedEncryptOptions {
    /// The name of encryptOption
    pub name: String,
    /// The valueFrom of encryptOption
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "valueFrom")]
    pub value_from: Option<DatasetSharedEncryptOptionsValueFrom>,
}

/// The valueFrom of encryptOption
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatasetSharedEncryptOptionsValueFrom {
    /// The encryptInfo obtained from secret
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretKeyRef")]
    pub secret_key_ref: Option<DatasetSharedEncryptOptionsValueFromSecretKeyRef>,
}

/// The encryptInfo obtained from secret
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatasetSharedEncryptOptionsValueFromSecretKeyRef {
    /// The required key in the secret
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The name of required secret
    pub name: String,
}

/// The pod this Toleration is attached to tolerates any taint that matches the triple <key,value,effect> using the matching operator <operator>.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatasetTolerations {
    /// Effect indicates the taint effect to match. Empty means match all taint effects. When specified, allowed values are NoSchedule, PreferNoSchedule and NoExecute.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    /// Key is the taint key that the toleration applies to. Empty means match all taint keys. If the key is empty, operator must be Exists; this combination means to match all values and all keys.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Operator represents a key's relationship to the value. Valid operators are Exists and Equal. Defaults to Equal. Exists is equivalent to wildcard for value, so that a pod can tolerate all taints of a particular category.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    /// TolerationSeconds represents the period of time the toleration (which must be of effect NoExecute, otherwise this field is ignored) tolerates the taint. By default, it is not set, which means tolerate the taint forever (do not evict). Zero and negative values will be treated as 0 (evict immediately) by the system.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tolerationSeconds")]
    pub toleration_seconds: Option<i64>,
    /// Value is the taint value the toleration matches to. If the operator is Exists, the value should be empty, otherwise just a regular string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// DatasetStatus defines the observed state of Dataset
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatasetStatus {
    /// CacheStatus represents the total resources of the dataset.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cacheStates")]
    pub cache_states: Option<BTreeMap<String, String>>,
    /// Conditions is an array of current observed conditions.
    pub conditions: Vec<Condition>,
    /// DataBackupRef specifies the running Backup job that targets this Dataset. This is mainly used as a lock to prevent concurrent DataBackup jobs. Deprecated, use OperationRef instead
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataBackupRef")]
    pub data_backup_ref: Option<String>,
    /// DataLoadRef specifies the running DataLoad job that targets this Dataset. This is mainly used as a lock to prevent concurrent DataLoad jobs. Deprecated, use OperationRef instead
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataLoadRef")]
    pub data_load_ref: Option<String>,
    /// DatasetRef specifies the datasets namespaced name mounting this Dataset.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "datasetRef")]
    pub dataset_ref: Option<Vec<String>>,
    /// FileNum represents the file numbers of the dataset
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fileNum")]
    pub file_num: Option<String>,
    /// HCFSStatus represents hcfs info
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hcfs: Option<DatasetStatusHcfs>,
    /// the info of mount points have been mounted
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mounts: Option<Vec<DatasetStatusMounts>>,
    /// OperationRef specifies the Operation that targets this Dataset. This is mainly used as a lock to prevent concurrent same Operation jobs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "operationRef")]
    pub operation_ref: Option<BTreeMap<String, String>>,
    /// Dataset Phase. One of the four phases: `Pending`, `Bound`, `NotBound` and `Failed`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    /// Runtimes for supporting dataset
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runtimes: Option<Vec<DatasetStatusRuntimes>>,
    /// Total in GB of dataset in the cluster
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ufsTotal")]
    pub ufs_total: Option<String>,
}

/// HCFSStatus represents hcfs info
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatasetStatusHcfs {
    /// Endpoint for accessing
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    /// Underlayer HCFS Compatible Version
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "underlayerFileSystemVersion")]
    pub underlayer_file_system_version: Option<String>,
}

/// Mount describes a mounting. <br> Refer to <a href="https://docs.alluxio.io/os/user/stable/en/ufs/S3.html">Alluxio Storage Integrations</a> for more info
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatasetStatusMounts {
    /// The secret information
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "encryptOptions")]
    pub encrypt_options: Option<Vec<DatasetStatusMountsEncryptOptions>>,
    /// MountPoint is the mount point of source.
    #[serde(rename = "mountPoint")]
    pub mount_point: String,
    /// The name of mount
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The Mount Options. <br> Refer to <a href="https://docs.alluxio.io/os/user/stable/en/reference/Properties-List.html">Mount Options</a>.  <br> The option has Prefix 'fs.' And you can Learn more from <a href="https://docs.alluxio.io/os/user/stable/en/ufs/S3.html">The Storage Integrations</a>
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<BTreeMap<String, String>>,
    /// The path of mount, if not set will be /{Name}
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Optional: Defaults to false (read-write).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "readOnly")]
    pub read_only: Option<bool>,
    /// Optional: Defaults to false (shared).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shared: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatasetStatusMountsEncryptOptions {
    /// The name of encryptOption
    pub name: String,
    /// The valueFrom of encryptOption
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "valueFrom")]
    pub value_from: Option<DatasetStatusMountsEncryptOptionsValueFrom>,
}

/// The valueFrom of encryptOption
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatasetStatusMountsEncryptOptionsValueFrom {
    /// The encryptInfo obtained from secret
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretKeyRef")]
    pub secret_key_ref: Option<DatasetStatusMountsEncryptOptionsValueFromSecretKeyRef>,
}

/// The encryptInfo obtained from secret
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatasetStatusMountsEncryptOptionsValueFromSecretKeyRef {
    /// The required key in the secret
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The name of required secret
    pub name: String,
}

/// Runtime describes a runtime to be used to support dataset
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatasetStatusRuntimes {
    /// Category the runtime object belongs to (e.g. Accelerate)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Runtime master replicas
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "masterReplicas")]
    pub master_replicas: Option<i32>,
    /// Name of the runtime object
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the runtime object
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Runtime object's type (e.g. Alluxio)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

