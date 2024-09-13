// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/alauda/nativestor/topolvm.cybozu.com/v2/topolvmclusters.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// TopolvmClusterSpec defines the desired state of TopolvmCluster
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "topolvm.cybozu.com", version = "v2", kind = "TopolvmCluster", plural = "topolvmclusters")]
#[kube(namespaced)]
#[kube(status = "TopolvmClusterStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct TopolvmClusterSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "certsSecret")]
    pub certs_secret: Option<String>,
    pub cleanup: bool,
    pub storage: TopolvmClusterStorage,
    #[serde(rename = "topolvmVersion")]
    pub topolvm_version: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TopolvmClusterStorage {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "className")]
    pub class_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deviceClasses")]
    pub device_classes: Option<Vec<TopolvmClusterStorageDeviceClasses>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<TopolvmClusterStorageDevices>>,
    #[serde(rename = "useAllDevices")]
    pub use_all_devices: bool,
    #[serde(rename = "useAllNodes")]
    pub use_all_nodes: bool,
    #[serde(rename = "useLoop")]
    pub use_loop: bool,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeGroupName")]
    pub volume_group_name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TopolvmClusterStorageDeviceClasses {
    pub classes: Vec<TopolvmClusterStorageDeviceClassesClasses>,
    #[serde(rename = "nodeName")]
    pub node_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TopolvmClusterStorageDeviceClassesClasses {
    #[serde(rename = "className")]
    pub class_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
    pub devices: Vec<TopolvmClusterStorageDeviceClassesClassesDevices>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "spareGb")]
    pub spare_gb: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stripe: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "stripeSize")]
    pub stripe_size: Option<String>,
    #[serde(rename = "volumeGroup")]
    pub volume_group: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TopolvmClusterStorageDeviceClassesClassesDevices {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto: Option<bool>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TopolvmClusterStorageDevices {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto: Option<bool>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "type")]
    pub r#type: String,
}

/// TopolvmClusterStatus defines the observed state of TopolvmCluster
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TopolvmClusterStatus {
    #[serde(rename = "nodeStorageState")]
    pub node_storage_state: Vec<TopolvmClusterStatusNodeStorageState>,
    /// INSERT ADDITIONAL STATUS FIELD - define observed state of cluster Important: Run "make" to regenerate code after modifying this file
    pub phase: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TopolvmClusterStatusNodeStorageState {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failClasses")]
    pub fail_classes: Option<Vec<TopolvmClusterStatusNodeStorageStateFailClasses>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loops: Option<Vec<TopolvmClusterStatusNodeStorageStateLoops>>,
    pub node: String,
    pub phase: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "successClasses")]
    pub success_classes: Option<Vec<TopolvmClusterStatusNodeStorageStateSuccessClasses>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TopolvmClusterStatusNodeStorageStateFailClasses {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "className")]
    pub class_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deviceStates")]
    pub device_states: Option<Vec<TopolvmClusterStatusNodeStorageStateFailClassesDeviceStates>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vgName")]
    pub vg_name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TopolvmClusterStatusNodeStorageStateFailClassesDeviceStates {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TopolvmClusterStatusNodeStorageStateLoops {
    #[serde(rename = "deviceName")]
    pub device_name: String,
    pub file: String,
    pub message: String,
    pub name: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TopolvmClusterStatusNodeStorageStateSuccessClasses {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "className")]
    pub class_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deviceStates")]
    pub device_states: Option<Vec<TopolvmClusterStatusNodeStorageStateSuccessClassesDeviceStates>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vgName")]
    pub vg_name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TopolvmClusterStatusNodeStorageStateSuccessClassesDeviceStates {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

