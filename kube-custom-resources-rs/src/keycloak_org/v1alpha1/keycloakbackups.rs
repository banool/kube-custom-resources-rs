// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/keycloak/keycloak-operator/keycloak.org/v1alpha1/keycloakbackups.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// KeycloakBackupSpec defines the desired state of KeycloakBackup.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "keycloak.org", version = "v1alpha1", kind = "KeycloakBackup", plural = "keycloakbackups")]
#[kube(namespaced)]
#[kube(status = "KeycloakBackupStatus")]
#[kube(schema = "disabled")]
pub struct KeycloakBackupSpec {
    /// If provided, an automatic database backup will be created on AWS S3 instead of a local Persistent Volume. If this property is not provided - a local Persistent Volume backup will be chosen.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aws: Option<KeycloakBackupAws>,
    /// Selector for looking up Keycloak Custom Resources.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "instanceSelector")]
    pub instance_selector: Option<KeycloakBackupInstanceSelector>,
    /// Controls automatic restore behavior. Currently not implemented. 
    ///  In the future this will be used to trigger automatic restore for a given KeycloakBackup. Each backup will correspond to a single snapshot of the database (stored either in a Persistent Volume or AWS). If a user wants to restore it, all he/she needs to do is to change this flag to true. Potentially, it will be possible to restore a single backup multiple times.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restore: Option<bool>,
    /// Name of the StorageClass for Postgresql Backup Persistent Volume Claim
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageClassName")]
    pub storage_class_name: Option<String>,
}

/// If provided, an automatic database backup will be created on AWS S3 instead of a local Persistent Volume. If this property is not provided - a local Persistent Volume backup will be chosen.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeycloakBackupAws {
    /// Provides a secret name used for connecting to AWS S3 Service. The secret needs to be in the following form: 
    ///      apiVersion: v1     kind: Secret     metadata:       name: <Secret name>     type: Opaque     stringData:       AWS_S3_BUCKET_NAME: <S3 Bucket Name>       AWS_ACCESS_KEY_ID: <AWS Access Key ID>       AWS_SECRET_ACCESS_KEY: <AWS Secret Key> 
    ///  For more information, please refer to the Operator documentation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "credentialsSecretName")]
    pub credentials_secret_name: Option<String>,
    /// If provided, the database backup will be encrypted. Provides a secret name used for encrypting database data. The secret needs to be in the following form: 
    ///      apiVersion: v1     kind: Secret     metadata:       name: <Secret name>     type: Opaque     stringData:       GPG_PUBLIC_KEY: <GPG Public Key>       GPG_TRUST_MODEL: <GPG Trust Model>       GPG_RECIPIENT: <GPG Recipient> 
    ///  For more information, please refer to the Operator documentation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "encryptionKeySecretName")]
    pub encryption_key_secret_name: Option<String>,
    /// If specified, it will be used as a schedule for creating a CronJob.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
}

/// Selector for looking up Keycloak Custom Resources.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeycloakBackupInstanceSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<KeycloakBackupInstanceSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeycloakBackupInstanceSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// KeycloakBackupStatus defines the observed state of KeycloakBackup.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeycloakBackupStatus {
    /// Human-readable message indicating details about current operator phase or error.
    pub message: String,
    /// Current phase of the operator.
    pub phase: String,
    /// True if all resources are in a ready state and all work is done.
    pub ready: bool,
    /// A map of all the secondary resources types and names created for this CR. e.g "Deployment": [ "DeploymentName1", "DeploymentName2" ]
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secondaryResources")]
    pub secondary_resources: Option<BTreeMap<String, String>>,
}

