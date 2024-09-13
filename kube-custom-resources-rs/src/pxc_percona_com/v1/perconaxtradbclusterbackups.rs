// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/percona/percona-xtradb-cluster-operator/pxc.percona.com/v1/perconaxtradbclusterbackups.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "pxc.percona.com", version = "v1", kind = "PerconaXtraDBClusterBackup", plural = "perconaxtradbclusterbackups")]
#[kube(namespaced)]
#[kube(status = "PerconaXtraDBClusterBackupStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct PerconaXtraDBClusterBackupSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerOptions")]
    pub container_options: Option<PerconaXtraDBClusterBackupContainerOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pxcCluster")]
    pub pxc_cluster: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageName")]
    pub storage_name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaXtraDBClusterBackupContainerOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<PerconaXtraDBClusterBackupContainerOptionsArgs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<PerconaXtraDBClusterBackupContainerOptionsEnv>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaXtraDBClusterBackupContainerOptionsArgs {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub xbcloud: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub xbstream: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub xtrabackup: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaXtraDBClusterBackupContainerOptionsEnv {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "valueFrom")]
    pub value_from: Option<PerconaXtraDBClusterBackupContainerOptionsEnvValueFrom>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaXtraDBClusterBackupContainerOptionsEnvValueFrom {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configMapKeyRef")]
    pub config_map_key_ref: Option<PerconaXtraDBClusterBackupContainerOptionsEnvValueFromConfigMapKeyRef>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldRef")]
    pub field_ref: Option<PerconaXtraDBClusterBackupContainerOptionsEnvValueFromFieldRef>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceFieldRef")]
    pub resource_field_ref: Option<PerconaXtraDBClusterBackupContainerOptionsEnvValueFromResourceFieldRef>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretKeyRef")]
    pub secret_key_ref: Option<PerconaXtraDBClusterBackupContainerOptionsEnvValueFromSecretKeyRef>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaXtraDBClusterBackupContainerOptionsEnvValueFromConfigMapKeyRef {
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaXtraDBClusterBackupContainerOptionsEnvValueFromFieldRef {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    #[serde(rename = "fieldPath")]
    pub field_path: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaXtraDBClusterBackupContainerOptionsEnvValueFromResourceFieldRef {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerName")]
    pub container_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub divisor: Option<IntOrString>,
    pub resource: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaXtraDBClusterBackupContainerOptionsEnvValueFromSecretKeyRef {
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaXtraDBClusterBackupStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub azure: Option<PerconaXtraDBClusterBackupStatusAzure>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lastscheduled: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "latestRestorableTime")]
    pub latest_restorable_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s3: Option<PerconaXtraDBClusterBackupStatusS3>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sslInternalSecretName")]
    pub ssl_internal_secret_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sslSecretName")]
    pub ssl_secret_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageName")]
    pub storage_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vaultSecretName")]
    pub vault_secret_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "verifyTLS")]
    pub verify_tls: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaXtraDBClusterBackupStatusAzure {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "credentialsSecret")]
    pub credentials_secret: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "endpointUrl")]
    pub endpoint_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageClass")]
    pub storage_class: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaXtraDBClusterBackupStatusS3 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "credentialsSecret")]
    pub credentials_secret: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "endpointUrl")]
    pub endpoint_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

